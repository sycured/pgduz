#![forbid(unsafe_code)]

#[cfg(not(target_os = "windows"))]
use jemallocator::Jemalloc;

#[cfg(not(target_os = "windows"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, ArgMatches, Command};

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
            Arg::new("retain_count")
                .help("A number to retain, delete older files")
                .env("RETAIN_COUNT")
                .required(true),
        )
        .arg(
            Arg::new("client_id")
                .help("Zoho API client id")
                .env("client_id")
                .required(true),
        )
        .arg(
            Arg::new("client_secret")
                .help("Zão API client secret")
                .env("client_secret")
                .required(true),
        )
        .arg(
            Arg::new("parent_id")
                .help("Zoho WorkDrive folder id")
                .env("parent_id")
                .required(true),
        )
        .arg(
            Arg::new("refresh_token")
                .help("Zoho API refresh token")
                .env("refresh_token")
                .required(true),
        )
}

fn main() {
    let matches: ArgMatches = build_cli().get_matches();
    let (pg_dump_status, dump_filename): (bool, String) = pg_dump::dump_db(
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
    if pg_dump_status {
        if encrypt::age(
            dump_filename.as_str(),
            matches
                .get_one::<String>("age_public_key")
                .unwrap()
                .replace('"', "")
                .as_str(),
        ) {
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
                format!("{}.age", dump_filename).as_str(),
            );
        } else {
            eprintln!("Failed to encrypt the dump.");
        }
    } else {
        eprintln!("Failed to dump the database.");
    }
}
