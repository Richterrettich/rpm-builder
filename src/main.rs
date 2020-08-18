use chrono;
extern crate clap;

use regex::Regex;
use rpm;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::str::FromStr;

mod cli;

fn main() -> Result<(), AppError> {
    let matches = cli::build_cli().get_matches();
    let name = matches.value_of("name").unwrap();
    let version = matches.value_of("version").unwrap();
    let license = matches.value_of("license").unwrap();
    let arch = matches.value_of("arch").unwrap();
    let description = matches.value_of("desc").unwrap();
    let epoch: i32 = matches
        .value_of("epoch")
        .unwrap()
        .parse()
        .map_err(|_e| AppError::new("unable to convert provided epoch value to integer"))?;

    let release = matches.value_of("release").unwrap();

    let output_path = match matches.value_of("out") {
        Some(p) => p.to_string(),
        None => format!("./{}.rpm", name),
    };

    let compressor = rpm::Compressor::from_str(matches.value_of("compression").unwrap())?;
    let mut builder =
        rpm::RPMBuilder::new(name, version, license, arch, description).compression(compressor);

    let files = matches
        .values_of("file")
        .map(|v| v.collect())
        .unwrap_or(Vec::new());

    for (src, options) in parse_file_options(files)? {
        builder = builder
            .with_file(src, options)
            .map_err(|e| AppError::new(format!("error adding regular file {}: {}", src, e)))?;
    }

    builder = builder
        .release(release.parse::<u16>().unwrap())
        .epoch(epoch);

    let files = matches
        .values_of("exec-file")
        .map(|v| v.collect())
        .unwrap_or(Vec::new());

    for (src, options) in parse_file_options(files)? {
        builder = builder
            .with_file(src, options.mode(0o100755))
            .map_err(|e| AppError::new(format!("error adding executable file {}: {}", src, e)))?;
    }

    let files = matches
        .values_of("config-file")
        .map(|v| v.collect())
        .unwrap_or(Vec::new());
    for (src, options) in parse_file_options(files)? {
        builder = builder
            .with_file(src, options.is_config())
            .map_err(|e| AppError::new(format!("error adding config file {}: {}", src, e)))?;
    }

    let dirs = matches
        .values_of("dir")
        .map(|v| v.collect())
        .unwrap_or(Vec::new());

    for dir in dirs {
        let parts: Vec<&str> = dir.split(":").collect();
        if parts.len() != 2 {
            return Err(AppError::new(format!(
                "invalid file argument:{} it needs to be of the form <source-path>:<dest-path>",
                dir
            )));
        }
        let dir = parts[0];
        let target = PathBuf::from(parts[1]);
        builder = add_dir(dir, &target, builder)
            .map_err(|e| AppError::new(format!("error adding dir {}: {}", dir, e)))?;
    }

    let files = matches
        .values_of("doc-file")
        .map(|v| v.collect())
        .unwrap_or(Vec::new());
    for (src, options) in parse_file_options(files)? {
        builder = builder
            .with_file(src, options.is_doc())
            .map_err(|e| AppError::new(format!("error adding doc file {}: {}", src, e)))?;
    }

    let possible_preinst_script = matches.value_of("pre-install-script");

    if possible_preinst_script.is_some() {
        let preinstall_script = possible_preinst_script.unwrap();
        let mut f = std::fs::File::open(preinstall_script)?;
        let mut content = String::new();
        f.read_to_string(&mut content).map_err(|e| {
            AppError::new(format!(
                "error reading pre-install-script {}: {}",
                preinstall_script, e
            ))
        })?;
        builder = builder.pre_install_script(content);
    }

    let possible_postinst_script = matches.value_of("post-install-script");

    if possible_postinst_script.is_some() {
        let post_install_script = possible_postinst_script.unwrap();
        let mut f = std::fs::File::open(post_install_script)?;
        let mut content = String::new();
        f.read_to_string(&mut content).map_err(|e| {
            AppError::new(format!(
                "error reading post-install-script {}: {}",
                post_install_script, e
            ))
        })?;
        builder = builder.post_install_script(content);
    }

    let raw_changelog = matches
        .values_of("changelog")
        .map(|v| v.collect())
        .unwrap_or(Vec::new());

    for raw_entry in raw_changelog {
        let parts: Vec<&str> = raw_entry.split(":").collect();
        if parts.len() != 3 {
            return Err(AppError::new(format!(
                    "invalid file argument:{} it needs to be of the form <author>:<content>:<yyyy-mm-dd>",
                    &raw_entry
               )));
        }
        let name = parts[0];
        let content = parts[1];
        let raw_time = parts[2];
        let parse_result = chrono::NaiveDate::parse_from_str(raw_time, "%Y-%m-%d");
        if parse_result.is_err() {
            return Err(AppError::new(format!(
                "error while parsing date time: {}",
                parse_result.err().unwrap()
            )));
        }
        let seconds = parse_result
            .unwrap()
            .and_time(chrono::NaiveTime::from_hms(0, 0, 0))
            .timestamp();
        builder = builder.add_changelog_entry(name, content, seconds as i32);
    }

    let re = Regex::new(r"^([a-zA-Z0-9\-\._]+)(\s*(>=|>|=|<=|<)(.+))?$").unwrap();

    let requires = matches
        .values_of("requires")
        .map(|v| v.collect())
        .unwrap_or(Vec::new());

    for req in requires {
        let dependency = parse_dependency(&re, req)?;
        builder = builder.requires(dependency);
    }

    let obsoletes = matches
        .values_of("obsoletes")
        .map(|v| v.collect())
        .unwrap_or(Vec::new());

    for item in obsoletes {
        let dependency = parse_dependency(&re, item)?;
        builder = builder.obsoletes(dependency);
    }

    let conflicts = matches
        .values_of("conflicts")
        .map(|v| v.collect())
        .unwrap_or(Vec::new());

    for item in conflicts {
        let dependency = parse_dependency(&re, item)?;
        builder = builder.conflicts(dependency);
    }

    let provides = matches
        .values_of("provides")
        .map(|v| v.collect())
        .unwrap_or(Vec::new());

    for item in provides {
        let dependency = parse_dependency(&re, item)?;
        builder = builder.provides(dependency);
    }

    let pkg = if let Some(signing_key_path) = matches.value_of("sign-with-pgp-asc") {
        let raw_key = std::fs::read(signing_key_path).map_err(|e| {
            AppError::new(format!(
                "unable to load private key file from path {}: {}",
                signing_key_path, e
            ))
        })?;

        let signer = rpm::signature::pgp::Signer::load_from_asc_bytes(&raw_key).map_err(|e| {
            AppError::new(format!(
                "unable to create signer from private key {}: {}",
                signing_key_path, e
            ))
        })?;
        builder.build_and_sign(signer)?
    } else {
        builder.build()?
    };

    let mut out_file = std::fs::File::create(&output_path).map_err(|e| {
        AppError::new(format!(
            "unable to create output file {}: {}",
            output_path, e
        ))
    })?;
    pkg.write(&mut out_file).map_err(|e| {
        AppError::new(format!(
            "unable to write package to path {}: {}",
            output_path, e
        ))
    })?;
    Ok(())
}

fn add_dir<P: AsRef<Path>>(
    full_path: P,
    target_path: &PathBuf,
    mut builder: rpm::RPMBuilder,
) -> Result<rpm::RPMBuilder, AppError> {
    for entry in std::fs::read_dir(full_path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let mut new_target = target_path.clone();

        let source = if metadata.file_type().is_symlink() {
            std::fs::read_link(entry.path().as_path())?
        } else {
            entry.path()
        };

        let file_name = source
            .file_name()
            .ok_or_else(|| AppError::new("path does not have filename"))?;
        new_target.push(file_name);

        builder = if metadata.file_type().is_dir() {
            add_dir(&source, &new_target, builder)?
        } else {
            builder.with_file(
                &source,
                rpm::RPMFileOptions::new(new_target.to_string_lossy()),
            )?
        }
    }
    Ok(builder)
}

fn parse_file_options(
    raw_files: Vec<&str>,
) -> Result<Vec<(&str, rpm::RPMFileOptionsBuilder)>, AppError> {
    raw_files
        .iter()
        .map(|input| {
            let parts: Vec<&str> = input.split(":").collect();
            if parts.len() != 2 {
                return Err(AppError::new(format!(
                    "invalid file argument:{} it needs to be of the form <source-path>:<dest-path>",
                    input,
                )));
            }
            Ok((parts[0], rpm::RPMFileOptions::new(parts[1])))
        })
        .collect()
}

fn parse_dependency(re: &Regex, line: &str) -> Result<rpm::Dependency, AppError> {
    let parts = re.captures(line).ok_or(AppError::new(format!(
        "invalid pattern in dependency block {}",
        line
    )))?;
    let parts: Vec<String> = parts
        .iter()
        .filter(|c| c.is_some())
        .map(|c| String::from(c.unwrap().as_str()))
        .collect();

    if parts.len() <= 2 {
        Ok(rpm::Dependency::any(&parts[1]))
    } else {
        let dep = match parts[3].as_str() {
            "=" => rpm::Dependency::eq(&parts[1], &parts[4]),
            "<" => rpm::Dependency::less(&parts[1], &parts[4]),
            "<=" => rpm::Dependency::less_eq(&parts[1], &parts[4]),
            ">=" => rpm::Dependency::greater_eq(&parts[1], &parts[4]),
            ">" => rpm::Dependency::greater(&parts[1], &parts[4]),
            _ => {
                return Err(AppError::new(format!(
                    "regex is invalid here, got unknown match {}",
                    &parts[3]
                )))
            }
        };
        Ok(dep)
    }
}

struct AppError {
    cause: String,
}

impl AppError {
    fn new<T: Into<String>>(cause: T) -> Self {
        return AppError {
            cause: cause.into(),
        };
    }
}

impl std::error::Error for AppError {}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.cause)
    }
}

impl std::fmt::Debug for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.cause)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> AppError {
        AppError::new(format!("{}", err))
    }
}

impl From<rpm::RPMError> for AppError {
    fn from(err: rpm::RPMError) -> AppError {
        AppError::new(format!("{}", err))
    }
}
