FROM rust:1.54

RUN mkdir -p /app
WORKDIR /app

COPY ./Cargo.toml /app/
COPY ./Cargo.lock /app/
COPY ./src /app/src/

RUN cargo install --path . && \
    cargo install diesel_cli --no-default-features --features mysql

CMD ["/bin/bash"]
