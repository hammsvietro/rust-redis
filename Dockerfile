FROM rust:1.31

RUN rustup override set nightly
WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .


EXPOSE 3000