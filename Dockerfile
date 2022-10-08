FROM docker.io/rust:1.64-slim-buster as builder

RUN USER=root cargo new --vcs none --bin todone-backend
RUN USER=root cargo new --vcs none --lib todone-core

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

WORKDIR /todone-core
COPY ./todone-core/Cargo.toml  ./Cargo.toml

WORKDIR /todone-backend
COPY ./todone-backend/Cargo.toml  ./Cargo.toml

# Caching dependenies
WORKDIR /
RUN cargo fetch

WORKDIR /todone-core
COPY ./todone-core/src ./src

WORKDIR /todone-backend
COPY ./todone-backend/src ./src

WORKDIR /
RUN cargo install --path ./todone-backend

EXPOSE 8000

CMD ["todone-backend"]