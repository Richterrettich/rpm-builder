use std::env;
use std::fs;

use std::path::PathBuf;
use std::process::Command;

#[test]
fn test_gzipped() -> Result<(), Box<dyn std::error::Error>> {
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

    Command::new(rpm_builder_path)
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
            "gzip",
            "rpm-builder",
            "-o",
            &out_file.to_string_lossy(),
            "--pre-install-script",
            &format!(
                "{}/tests/test_assets/preinst.sh",
                &work_dir.to_string_lossy()
            ),
        ])
        .output()
        .expect("failed to execute process");
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

    Command::new(rpm_builder_path)
        .args(vec![
            "--exec-file",
            "target/debug/rpm-builder:/usr/bin/rpm-builder",
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
