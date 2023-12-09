FROM rust:1.74-slim-bullseye
WORKDIR /app

# CACHING DEPS
COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir src
RUN echo "fn main() {}" >> src/main.rs
RUN cargo build --release
RUN rm src/main.rs

# ENV ARGS
ENV SERVER_IP=0.0.0.0:3000
ENV JWT_SECRET_TIMEOUT_SECONDS=2592000

# ACTUALLY BUILDING BINARY
COPY . .
RUN cargo build --release
CMD ["cargo", "run", "--release"]
EXPOSE 3000:3000
