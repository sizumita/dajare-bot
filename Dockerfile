FROM debian:buster-slim as runner

RUN apt update; apt install -y libssl1.1 build-essential autoconf automake libtool m4 && \
    apt-get install -y --no-install-recommends ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

FROM rust:1.55.0 AS chef

RUN cargo install cargo-chef
WORKDIR dajare

FROM chef AS planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /dajare/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application

COPY Cargo.lock .
COPY Cargo.toml .
COPY src src
COPY detector ./detector
RUN cargo build --release --bin dajare


FROM chef AS dic-builder

RUN apt-get update; apt-get install -y sudo mecab libmecab-dev mecab-ipadic-utf8 \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install lindera-ipadic-neologd-builder

COPY neologd/make_ipadic.sh /neologd/make_ipadic.sh

RUN wget -O mecab-ipadic-2.7.0-20070801.tar.gz 'https://drive.google.com/uc?export=download&id=0B4y35FiV1wh7MWVlSDBCSXZMTXM' \
    && tar zxfv mecab-ipadic-2.7.0-20070801.tar.gz \
    && cd mecab-ipadic-2.7.0-20070801 \
    && ./configure --with-charset=utf8 \
    && make install
RUN bash /neologd/make_ipadic.sh

FROM runner

COPY --from=builder /dajare/target/release/dajare /usr/local/bin
COPY --from=dic-builder /dajare/lindera-ipadic ./lindera-ipadic

USER 1000

ENTRYPOINT ["/usr/local/bin/dajare"]
