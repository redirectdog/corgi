FROM alpine:3.9 AS builder
RUN apk add --no-cache rust cargo
WORKDIR /usr/src/corgi
COPY Cargo.* ./
COPY src ./src
RUN cargo build --release

FROM alpine:3.9
RUN apk add --no-cache libgcc
COPY --from=builder /usr/src/corgi/target/release/corgi /usr/bin/
CMD ["corgi"]
