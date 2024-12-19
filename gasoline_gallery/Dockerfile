FROM rust:1.82

WORKDIR /

COPY . .
RUN cargo build --release

CMD [ "./target/release/gasoline-gallery" ]