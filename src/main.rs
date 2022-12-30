#![forbid(unsafe_code)]

#[cfg(not(target_os = "windows"))]
use jemallocator::Jemalloc;

#[cfg(not(target_os = "windows"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

use clap::{
    crate_authors, crate_description, crate_name, crate_version, Arg, ArgAction::SetTrue,
    ArgMatches, Command,
};

mod encrypt;
mod pg_dump;
mod zoho_workdrive_uploader;

fn build_cli() -> Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg_required_else_help(true)
        .arg(
            Arg::new("age_public_key")
                .help("AGE public key to encrypt backup")
                .env("AGE_PUBLIC_KEY")
                .required(true),
        )
        .arg(
            Arg::new("pg_db")
                .help("The name of the database")
                .env("PGDB")
                .required(true),
        )
        .arg(
            Arg::new("pg_host")
                .help("The hostname of the database")
                .env("PGHOST")
                .required(true),
        )
        .arg(
            Arg::new("pg_password")
                .help("The password of the database")
                .env("PGPASSWORD")
                .required(true),
        )
        .arg(
            Arg::new("pg_port")
                .help("The port of the database")
                .env("PGPORT")
                .required(true),
        )
        .arg(
            Arg::new("pg_user")
                .help("The username of the database")
                .env("PGUSER")
                .required(true),
        )
        .arg(
            Arg::new("enable_upload")
                .help("Enable uploading the encrypted backup to Zoho WorkDrive")
                .long("enable_upload")
                .short('u')
                .action(SetTrue)
                .env("ENABLE_UPLOAD"),
        )
        .arg(
            Arg::new("client_id")
                .help("Zoho API client id")
                .env("CLIENT_ID")
                .requires("enable_upload"),
        )
        .arg(
            Arg::new("client_secret")
                .help("Zoho API client secret")
                .env("CLIENT_SECRET")
                .requires("enable_upload"),
        )
        .arg(
            Arg::new("parent_id")
                .help("Zoho WorkDrive folder id")
                .env("PARENT_ID")
                .requires("enable_upload"),
        )
        .arg(
            Arg::new("refresh_token")
                .help("Zoho API refresh token")
                .env("REFRESH_TOKEN")
                .requires("enable_upload"),
        )
}

fn main() {
    let matches: ArgMatches = build_cli().get_matches();
    let upload_enabled: bool = *matches.get_one::<bool>("enable_upload").unwrap_or(&false);
    let dump_filename: String = pg_dump::dump_db(
        matches
            .get_one::<String>("pg_host")
            .unwrap()
            .replace('"', "")
            .as_str(),
        matches
            .get_one::<String>("pg_port")
            .unwrap()
            .replace('"', "")
            .as_str(),
        matches
            .get_one::<String>("pg_db")
            .unwrap()
            .replace('"', "")
            .as_str(),
        matches
            .get_one::<String>("pg_user")
            .unwrap()
            .replace('"', "")
            .as_str(),
    );
    if encrypt::age(
        dump_filename.as_str(),
        matches
            .get_one::<String>("age_public_key")
            .unwrap()
            .replace('"', "")
            .as_str(),
    ) {
        println!("Encryption done.");
        if upload_enabled {
            zoho_workdrive_uploader::upload(
                matches
                    .get_one::<String>("client_id")
                    .unwrap()
                    .replace('"', "")
                    .as_str(),
                matches
                    .get_one::<String>("client_secret")
                    .unwrap()
                    .replace('"', "")
                    .as_str(),
                matches
                    .get_one::<String>("parent_id")
                    .unwrap()
                    .replace('"', "")
                    .as_str(),
                matches
                    .get_one::<String>("refresh_token")
                    .unwrap()
                    .replace('"', "")
                    .as_str(),
                format!("{dump_filename}.age").as_str(),
            );
        } else {
            println!(
                "Encryption done without the upload to Zoho WorkDrive: the option wasn't enabled."
            );
        }
    } else {
        eprintln!("Failed to encrypt the dump.");
    }
}
