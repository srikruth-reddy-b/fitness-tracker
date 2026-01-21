# ==============================================================================
# 1. BUILD STAGE
# ==============================================================================
FROM rust:1.84-slim-bookworm as builder

# Set the working directory inside the container
WORKDIR /app

# Install system dependencies required for building (libpq-dev for Postgres)
RUN apt-get update && apt-get install -y libpq-dev pkg-config

# Copy the entire project
COPY . .

# Build BOTH the main backend AND the seed_data utility
RUN cargo build --release --bin backend --bin seed_data

# Install Diesel CLI for migrations
RUN cargo install diesel_cli --version 2.2.0 --no-default-features --features postgres

# ==============================================================================
# 2. RUNTIME STAGE
# ==============================================================================
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    openssl \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binaries
COPY --from=builder /app/target/release/backend ./backend
COPY --from=builder /app/target/release/seed_data ./seed_data

# Copy the diesel CLI binary
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

# Copy the migrations folder
COPY --from=builder /app/migrations ./migrations

# Copy the CSV files required by seed_data
# (Using COPY from context, since they are in the root of the backend folder)
COPY cardio_exercises.csv .
COPY muscle_groups.csv .
COPY variations.csv .

# Expose the port
EXPOSE 8080

# The command to start the application
# 1. Run Migrations
# 2. Run Seed Data (This will populate muscle groups/variations if missing)
# 3. Start the Backend API
CMD ["sh", "-c", "diesel migration run && ./seed_data && ./backend"]
