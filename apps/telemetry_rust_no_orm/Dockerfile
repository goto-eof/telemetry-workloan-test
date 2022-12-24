# Build stage
FROM rust:latest as builder
WORKDIR /app
ADD . /app
# migration
COPY .env.prod .env
# application
RUN cargo build --release

# Prod stage
FROM gcr.io/distroless/cc
# copy configuration
COPY .env /
COPY default.json /
COPY production.json /
COPY log4rs.yml /

COPY --from=builder /app/target/release/telemetry_rust_no_orm /
ENV DEV_BOARD_ENV production
CMD ["./telemetry_rust_no_orm"]