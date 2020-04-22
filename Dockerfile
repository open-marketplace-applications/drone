FROM rust:latest

WORKDIR /usr/src/drone

COPY . .

RUN cargo install --path .

EXPOSE 5000

ENTRYPOINT ["drone"]