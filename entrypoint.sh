#!/bin/bash

set -ae

COMMAND=${1:-dump-cron}
CRON_SCHEDULE=${CRON_SCHEDULE:-0 1 * * *}
PGUSER=${PGUSER:-postgres}
PGDB=${PGDB:-postgres}
PGHOST=${PGHOST:-db}
PGPORT=${PGPORT:-5432}
client_id=${client_id}
client_secret=${client_secret}
parent_id=${parent_id}

if [[ "${COMMAND}" == 'dump' ]]; then
    exec /app/pgduz
elif [[ "${COMMAND}" == 'dump-cron' ]]; then
    LOGFIFO='/var/log/cron.fifo'
    if [[ ! -e "${LOGFIFO}" ]]; then
        mkfifo "${LOGFIFO}"
    fi
    CRON_ENV="PGUSER='${PGUSER}'\nPGDB='${PGDB}'\nPGHOST='${PGHOST}'\nPGPORT='${PGPORT}'\nAGE_PUBLIC_KEY='${AGE_PUBLIC_KEY}'\nparent_id='${parent_id}'\nrefresh_token='${refresh_token}'\nclient_secret='${client_secret}'\nclient_id='${client_id}'"
    if [[ -n "${PGPASSWORD}" ]]; then
        CRON_ENV="$CRON_ENV\nPGPASSWORD='${PGPASSWORD}'"
    fi

    if [[ ! -z "${RETAIN_COUNT}" ]]; then
    	CRON_ENV="$CRON_ENV\nRETAIN_COUNT='${RETAIN_COUNT}'"
    fi

    echo -e "$CRON_ENV\n$CRON_SCHEDULE /app/pgduz > $LOGFIFO 2>&1" | crontab -
    cron
    tail -f "$LOGFIFO"
else
    echo "Unknown command $COMMAND"
    echo "Available commands: dump, dump-cron"
    exit 1
fi
