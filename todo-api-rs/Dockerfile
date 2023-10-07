FROM messense/rust-musl-cross:x86_64-musl as builder
# allow building rust with sqlx macros without an active database connection
ENV SQLX_OFFILINE=true

WORKDIR /todo-api-rs
# copy source code
COPY . .

# build in release mode
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /todo-api-rs/target/x86_64-unknown-linux-musl/release/todo-api-rs /todo-api-rs
ENTRYPOINT [ "todo-api-rs" ]

EXPOSE 8000

