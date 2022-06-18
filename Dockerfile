FROM rust:alpine

WORKDIR /usr/src/piresgg
COPY . .

RUN cargo build --release

EXPOSE 80
EXPOSE 443
CMD ["./target/release/piresgg"]