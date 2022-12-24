# Build stage
FROM rust:latest as builder
WORKDIR /app
ADD . /app
RUN cargo build --release

# Prod stage
FROM gcr.io/distroless/cc
# COPY configuration/* /
COPY default.json /
COPY production.json /
COPY log4rs.yml /
COPY --from=builder /app/target/release/telemetry_rust /
ENV DEV_BOARD_ENV production
CMD ["./telemetry_rust"]