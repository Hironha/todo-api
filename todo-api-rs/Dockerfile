FROM rust:1.75-buster as builder

# add `x86_64-unknown-linux-musl` target
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# allow building rust with sqlx macros without an active database connection
ENV SQLX_OFFLINE=true

WORKDIR /app
# copy source code
COPY . .

# build in release mode
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/todo-api-rs ./

EXPOSE 8000

CMD [ "/app/todo-api-rs" ]