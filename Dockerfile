# Stage 1: Build the Rust project
FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Stage 2: Create the final image
FROM debian:bookworm-slim

# Install cron
RUN apt-get update && apt-get install -y cron && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/duck_updater .
COPY .env .

# Add the cron job
RUN echo "*/15 * * * * cd /app && /app/duck_updater >> /var/log/duck_updater.log 2>&1" > /etc/cron.d/duck_updater_cron
RUN chmod 0644 /etc/cron.d/duck_updater_cron
RUN crontab /etc/cron.d/duck_updater_cron
RUN touch /var/log/duck_updater.log
RUN chmod 0666 /var/log/duck_updater.log

# Add a health check
HEALTHCHECK --interval=5m --timeout=3s \
  CMD grep -q "duck_updater" /var/log/duck_updater.log || exit 1

# Start cron and tail the log file
CMD ["sh", "-c", "cron && tail -f /var/log/duck_updater.log"]
