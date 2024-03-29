use std::env;
use std::fs;

use std::path::PathBuf;
use std::process::Command;
use std::thread;

#[test]
fn test_compressed() -> Result<(), Box<dyn std::error::Error>> {
    let mut tmp_dir = env::temp_dir();
    tmp_dir.push("rpm-builder-test-gzipped");
    fs::create_dir_all(&tmp_dir)?;
    let mut out_file = tmp_dir.clone();
    out_file.push("test.rpm");

    let work_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let mut cargo_toml = work_dir.clone();
    cargo_toml.push("Cargo.toml");
    let mut rpm_builder_path = work_dir.clone();
    rpm_builder_path.push("target/debug/rpm-builder");
    let mut handles = Vec::new();
    for compression in ["gzip", "zstd"] {
        let rpm_builder_path = rpm_builder_path.clone();
        let cargo_toml = cargo_toml.clone();
        let work_dir = work_dir.clone();
        let out_file = out_file.clone();
        let handle = thread::spawn(move || {
            let result = Command::new(&rpm_builder_path)
                .args(vec![
                    "--exec-file",
                    "target/debug/rpm-builder:/usr/bin/rpm-builder",
                    "--doc-file",
                    &format!("{}:/foo/bar", &cargo_toml.to_string_lossy()),
                    "--config-file",
                    &format!("{}:/bar/bazz", &cargo_toml.to_string_lossy()),
                    "--version",
                    "1.0.0",
                    "--dir",
                    &format!("{}/tests/test_assets:/src", &work_dir.to_string_lossy()),
                    "--compression",
                    compression,
                    "rpm-builder",
                    "-o",
                    &out_file.to_string_lossy(),
                    "--release",
                    "foo-bar",
                    "--pre-install-script",
                    &format!(
                        "{}/tests/test_assets/preinst.sh",
                        &work_dir.to_string_lossy()
                    ),
                ])
                .output();
            (compression,result)
        });
        handles.push(handle);
    }
    for handle in handles {
       let (compression,result) =  handle.join().expect("unable to join thread");
       let result = result?;
       if !result.status.success() {
           let stdout = String::from_utf8_lossy(&result.stdout);
           let stderr = String::from_utf8_lossy(&result.stderr);
           panic!("{} faild with stdout: {}\nstderr:{}",compression,stdout,stderr);
       }
    }

    std::fs::remove_dir_all(tmp_dir)?;
    Ok(())
}

#[test]
fn test_not_compressed() -> Result<(), Box<dyn std::error::Error>> {
    let mut tmp_dir = env::temp_dir();
    tmp_dir.push("rpm-builder-test-not-compressed");
    fs::create_dir_all(&tmp_dir)?;
    let mut out_file = tmp_dir.clone();
    out_file.push("test.rpm");

    let mut rpm_builder_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    rpm_builder_path.push("target/debug/rpm-builder");

    Command::new(rpm_builder_path.clone())
        .args(vec![
            "--exec-file",
            &format!(
                "{}:/usr/bin/rpm-builder",
                rpm_builder_path.clone().to_string_lossy()
            ),
            "--version",
            "1.0.0",
            "--epoch",
            "5",
            "rpm-builder",
            "-o",
            &out_file.to_string_lossy(),
        ])
        .output()
        .expect("failed to execute process");
    std::fs::remove_dir_all(tmp_dir)?;
    Ok(())
}

#[test]
fn test_signature() -> Result<(), Box<dyn std::error::Error>> {
    let mut tmp_dir = env::temp_dir();
    tmp_dir.push("rpm-builder-test-signature");
    fs::create_dir_all(&tmp_dir)?;
    let mut out_file = tmp_dir.clone();
    out_file.push("test.rpm");

    let workspace_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut rpm_builder_path = workspace_path.clone();
    rpm_builder_path.push("target/debug/rpm-builder");

    let mut private_key_path = workspace_path.clone();
    private_key_path.push("tests/test_assets/package-manager.key");
    let mut public_key_path = workspace_path.clone();
    public_key_path.push("tests/test_assets/package-manager.key.pub");
    let output = Command::new(rpm_builder_path)
        .args(vec![
            "--exec-file",
            &format!(
                "{}/target/debug/rpm-builder:/usr/bin/rpm-builder",
                workspace_path.clone().to_string_lossy()
            ),
            "--version",
            "1.0.0",
            "--epoch",
            "5",
            "rpm-builder",
            "-o",
            &out_file.to_string_lossy(),
            "--sign-with-pgp-asc",
            &private_key_path.to_string_lossy(),
        ])
        .output()
        .expect("failed to execute process");
    if !output.stderr.is_empty() {
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
    assert!(output.status.success());

    let rpm_file = std::fs::File::open(&out_file)?;
    let mut buffer = std::io::BufReader::new(rpm_file);
    let pkg = rpm::RPMPackage::parse(&mut buffer)?;

    let raw_public_key = std::fs::read(public_key_path)?;
    let verifier = rpm::signature::pgp::Verifier::load_from_asc_bytes(&raw_public_key)?;
    pkg.verify_signature(verifier)?;

    std::fs::remove_dir_all(tmp_dir)?;
    Ok(())
}
