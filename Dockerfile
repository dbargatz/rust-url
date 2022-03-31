FROM rustlang/rust:nightly-bullseye

RUN cargo install -f cargo-fuzz

COPY idna/ .

RUN cargo fuzz build punycode-encode