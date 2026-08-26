FROM rust:1.85-slim AS build
WORKDIR /src
COPY . .
RUN cargo build --release -p nexus-cli

FROM debian:bookworm-slim
RUN useradd --create-home --uid 10001 nexus
COPY --from=build /src/target/release/nexus /usr/local/bin/nexus
USER nexus
ENTRYPOINT ["nexus"]
CMD ["doctor"]
