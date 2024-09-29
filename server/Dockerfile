ARG RUST_VERSION="1.81.0"

FROM rust:${RUST_VERSION}-alpine as build

ARG MODE_FLAG=""
ARG WORKDIR="/app"
ARG TARGET="x86_64-unknown-linux-musl"

RUN apk add --no-cache openssl-dev musl-dev

RUN rustup target add "${TARGET}"

WORKDIR ${WORKDIR}

# https://stackoverflow.com/questions/58473606/cache-rust-dependencies-with-docker-build
# build cargo deps without trying to build the app source code
RUN echo "fn main() {}" > dummy.rs
COPY Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN RUSTFLAGS="-Ctarget-feature=-crt-static" cargo build --target=${TARGET} ${MODE_FLAG}
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml

COPY assets assets
COPY src src
COPY test_data test_data
RUN RUSTFLAGS="-Ctarget-feature=-crt-static" cargo build --target=${TARGET} ${MODE_FLAG}
RUN RUSTFLAGS="-Ctarget-feature=-crt-static" cargo test --target=${TARGET} ${MODE_FLAG}

FROM alpine:latest
RUN apk add --no-cache libgcc
ARG WORKDIR="/app"
ARG MODE_DIR="debug"
ARG TARGET="x86_64-unknown-linux-musl"
ENV TAXES_CONFIG_PATH="/app/taxes.json"
ENV PORT="3000"
COPY --from=build ${WORKDIR}/target/${TARGET}/${MODE_DIR}/taxes-compare ${WORKDIR}/taxes-compare
COPY --from=build ${WORKDIR}/assets/taxes.json ${TAXES_CONFIG_PATH}
CMD ["/app/taxes-compare"]
