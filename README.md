# pg_dump with Zoho WorkDrive uploader

## Usage

Attach a target postgres container to this container and mount a volume
to `/dump` folder. Backups will appear in this volume.
Optionally set up cron job schedule (default is `0 1 * * *` - runs every day at 1:00 am).

Steps done:
    - dump the db in PostgreSQL custom format (binary)
    - encrypt using [age](https://github.com/FiloSottile/age/releases/latest) (open source and cross-platform: FreeBSD, Linux, macOS, and Windows)
    - send encrypted file to Zoho WorkDrive


## Create API client id, secret and code

- Go to https://api-console.zoho.com/
- Create a Self Client
- Take client id and secret in the Client Secret tab
- Generate code in Generate Code tab:
    - scope: `WorkDrive.files.CREATE,ZohoFiles.files.CREATE`
    - time duration: 10 minutes
    - scope description: same value as scope

Now, we need to get the refresh token: `curl -XPOST https://accounts.zoho.com/oauth/v2/token?code=MY_CODE&grant_type=authorization_code&client_id=MY_CLIENT_ID&client_secret=MY_CLIENT_SECRET&redirect_uri=https%3A%2F%2Fwww.example.com`

## Environment Variables:

| Variable         | Required? |  Default  | Description                                         |
|------------------|:---------:|:---------:|:----------------------------------------------------|
| `AGE_PUBLIC_KEY` | Required  |  `None`   | AGE public key to encrypt backup                    |
| `CRON_SCHEDULE`  | Required  | 0 1 * * * | The cron schedule at which to run the pg_dump (UTC) |
| `PGDB`           | Optional  | postgres  | The name of the database                            |
| `PGHOST`         | Optional  |    db     | The hostname of the database                        |
| `PGPASSWORD`     | Optional  |  `None`   | The password for accessing the database             |
| `PGPORT`         | Optional  |  `5432`   | The port for the database                           |
| `PGUSER`         | Required  | postgres  | The user for accessing the database                 |
| `RETAIN_COUNT`   | Optional  |  `None`   | A number to retain, delete older files              |
| `TZ`             | Optional  |    UTC    | Timezone used by the container_name                 |
| `client_id`      | Required  |  `None`   | Zoho API client id                                  |
| `client_secret`  | Required  |  `None`   | Zoho API client secret                              |
| `refresh_token`  | Required  |  `None`   | Zoho API refresh token                              |
| `parent_id`      | Required  |  `None`   | Zoho WorkDrive folder id                            | 

Docker Compose
==============

Example with docker-compose:

```yaml

database:
  image: postgres:14
  volumes:
    - ./db_data:/var/lib/postgresql/data
  environment:
    - POSTGRES_PASSWORD=SumPassw0rdHere
    - POSTGRES_DB=postgres
  restart: always

postgres-backup:
  image: pg_dump-zoho_workdrive_uploader:latest
  container_name: postgres-backup
  links:
    - database:db # Maps as "db"
  environment:
    - TZ=America/Bogota
    - PGUSER=postgres
    - PGPASSWORD=SumPassw0rdHere
    - CRON_SCHEDULE=0 3 * * * # Every day at 3am
    - RETAIN_COUNT=1 # Keep this number of backups on the file system (don't touch Zoho WorkDrive that you need to manage yourself)
    - PGDB=postgres # The name of the database to dump 
  #  - PGHOST=db # The hostname of the PostgreSQL database to dump
  volumes:
    - ./dump:/dump
  restart: always
```
