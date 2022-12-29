FROM rust:slim as builder
ENV RUSTFLAGS="-C target-feature=+crt-static"
WORKDIR /build
COPY . .
RUN apt-get update \
    && apt-get install -y libjemalloc-dev make \
    && cargo build --release

FROM postgres:14-bullseye
RUN echo "deb http://deb.debian.org/debian bullseye-backports main" > /etc/apt/sources.list.d/backports.list \
    && apt-get update \
    && apt-get install -y --no-install-recommends age/bullseye-backports bash cron shadow \
    && rm -rf /var/cache/apt/* \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -m -s /bin/bash app \
    && mkdir /dump \
    && chown app:app /dump

USER app
WORKDIR /app
COPY entrypoint.sh .
COPY --from=builder --chown=app:app /build/target/release/pgduz .
VOLUME /dump

ENTRYPOINT ["./entrypoint.sh"]
CMD ["dump-cron"]
