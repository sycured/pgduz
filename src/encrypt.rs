use std::process::Command;

pub fn age(filename: &str, public_key: &str) -> bool {
    let filename_with_path = format!("/dump/{filename}");
    let status = Command::new("age")
        .args([
            "-r",
            public_key,
            "-o",
            format!("{filename_with_path}.age").as_str(),
            &filename_with_path,
        ])
        .status()
        .unwrap();
    status.success()
}
