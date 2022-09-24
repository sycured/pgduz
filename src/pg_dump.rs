use std::process::Command;

pub fn dump_db(host: &str, port: &str, db: &str, user: &str) -> bool {
    let date = String::from_utf8(
        Command::new("date")
            .arg("+%Y%m%d_%H%M%S")
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();
    let filename = format!("{}-{}.sqlc", host, date);
    let filename_with_path = format!("/dump/{}", filename);

    println!("Job started: Dumping to {}", filename);

    true
}
