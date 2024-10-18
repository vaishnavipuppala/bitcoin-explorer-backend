# Use the official Rust image as the base image
FROM rust:1.70 as builder

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy the entire project
COPY . .

# Build the application
RUN cargo build --release

# Start a new stage with a minimal image
FROM debian:buster-slim

# Install necessary dependencies
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

# Set the working directory in the container
WORKDIR /usr/local/bin

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/bitcoin-explorer-backend ./

# Copy the .env file if it exists
COPY --from=builder /usr/src/app/.env ./ 

# Expose the port the app runs on
EXPOSE 8000

# Command to run the application
CMD ["./bitcoin-explorer-backend"]