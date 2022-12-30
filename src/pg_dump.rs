use chrono::Local;
use std::process::{exit, Command};
pub fn dump_db(host: &str, port: &str, db: &str, user: &str) -> String {
    let date: String = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let filename: String = format!("{db}-{date}.sqlc");
    let filename_with_path: String = format!("/dump/{}", &filename);

    println!("Job started: Dumping to {}", &filename);

    let status = Command::new("pg_dump")
        .args([
            "-Fc",
            "-h",
            host,
            "-p",
            port,
            "-U",
            user,
            "-f",
            &filename_with_path,
            "-d",
            db,
        ])
        .status()
        .unwrap();

    if status.success() {
        filename
    } else {
        eprintln!("Failed to dump the database {db}");
        exit(1);
    }
}
