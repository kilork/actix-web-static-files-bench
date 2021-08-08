# docker build --build-arg app_name=example-01-linux-basic -t actix-web-static-files-bench/example-01-linux-basic .
FROM rust:latest as cargo-build

ARG app_name

RUN apt-get update

RUN apt-get install musl-tools -y

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/$app_name

COPY $app_name/Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN --mount=type=cache,target=/usr/local/cargo/registry \
#    --mount=type=cache,target=/usr/src/$app_name/target \
    RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/${app_name}*

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
#    --mount=type=cache,target=/usr/src/$app_name/target \
    cd $app_name && \
    RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

ARG app_name

RUN addgroup -g 1000 ${app_name}

RUN adduser -D -s /bin/sh -u 1000 -G ${app_name} ${app_name}

WORKDIR /home/${app_name}/bin/

COPY --from=cargo-build /usr/src/${app_name}/target/x86_64-unknown-linux-musl/release/${app_name} .

RUN chown ${app_name}:${app_name} ${app_name}

USER ${app_name}

CMD ["./${app_name}"]

