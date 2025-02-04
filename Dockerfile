# syntax=docker/dockerfile:1

ARG BUILDER_BASE=rust:bookworm
ARG RUNTIME_BASE=debian:bookworm

FROM --platform=${TARGETPLATFORM} ${BUILDER_BASE} AS builder

# Install build dependencies if needed

FROM --platform=${TARGETPLATFORM} ${RUNTIME_BASE} AS runtime

# Install runtime dependencies if needed

FROM --platform=${TARGETPLATFORM} builder AS build

WORKDIR /build

RUN \
  --mount=type=bind,target=.,readwrite \
  --mount=type=cache,target=/usr/local/rustup,id=${TARGETPLATFORM} \
  --mount=type=cache,target=/usr/local/cargo/registry,id=${TARGETPLATFORM} \
  --mount=type=cache,target=target,id=${TARGETPLATFORM} \
  RUST_BACKTRACE=1 \
  cargo build --release --workspace

RUN \
  --mount=type=cache,target=target,id=${TARGETPLATFORM} \
  mkdir -p /artifacts \
  && cd target/release \
  && cp -t /artifacts \
  hello \
  && ls -la /artifacts

FROM --platform=${TARGETPLATFORM} runtime AS hello
COPY --from=build /artifacts/hello /usr/local/bin
RUN ldd /usr/local/bin/hello
CMD ["hello"]
