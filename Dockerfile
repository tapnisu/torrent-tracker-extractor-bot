FROM rust:alpine3.22 AS builder
LABEL authors="tapnisu"

WORKDIR /usr/src/torrent-tracker-extractor-bot

RUN apk update \
    && apk upgrade --available \
    && apk add --no-cache alpine-sdk libressl-dev

COPY . .
RUN cargo build --release

FROM alpine:3.22 AS runner

RUN apk update \
    && apk upgrade --available \
    && apk add --no-cache ca-certificates \
    && update-ca-certificates

COPY --from=builder /usr/src/torrent-tracker-extractor-bot/target/release/torrent-tracker-extractor-bot /usr/local/bin/torrent-tracker-extractor-bot

CMD ["torrent-tracker-extractor-bot"]
