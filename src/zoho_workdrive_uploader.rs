use attohttpc::{body, post, Response};
use serde_json::Value;
use std::fs::File;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

fn get_access_token(client_id: &str, client_secret: &str, refresh_token: &str) -> String {
    let zoho_token_endpoint = "https://accounts.zoho.com/oauth/v2/token";
    let url: &str = &format!(
        "{}?refresh_token={}&client_secret={}&grant_type=refresh_token&client_id={}",
        zoho_token_endpoint, refresh_token, client_secret, client_id
    );
    let r: Response = post(url)
        .header("Accept", "application/json")
        .send()
        .unwrap();
    r.json::<Value>().unwrap()["access_token"]
        .to_string()
        .replace('"', "")
}

fn get_timestamp() -> u128 {
    let now = SystemTime::now();
    now.duration_since(UNIX_EPOCH).unwrap().as_nanos()
}

pub fn upload(
    client_id: &str,
    client_secret: &str,
    parent_id: &str,
    refresh_token: &str,
    filename: &str,
) {
    let backup_folder_path: &str = "/dump";
    let filename_with_path: &str = &format!("{backup_folder_path}/{filename}");
    if Path::new(filename_with_path).exists() {
        let timestamp: u128 = get_timestamp();
        let access_token = get_access_token(client_id, client_secret, refresh_token);
        let file = File::open(filename_with_path).unwrap();
        println!("Start uploading {filename_with_path}");
        let r: Response = post("https://upload.zoho.com/workdrive-api/v1/stream/upload")
            .header("Content-Type", "text/plain")
            .header_append("Authorization", format!("Zoho-oauthtoken {access_token}"))
            .header_append("x-filename", filename)
            .header_append("x-parent_id", parent_id)
            .header_append("upload-id", timestamp.to_string())
            .header_append("x-streammode", 1)
            .body(body::File(file))
            .send()
            .unwrap();
        match r.status() {
            x if x == 200 => println!("Upload complete"),
            _ => println!("Upload failed: {}", r.status()),
        }
    } else {
        println!("Can't upload, file not found: {filename_with_path}");
    }
}
