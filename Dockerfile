# Use the official Rust Docker image as the base image
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the project files to the container
COPY . .

# Build the project inside the container
RUN cargo build 

# Create a new Docker image
FROM debian:buster-slim

# Install SQLite3 and other required dependencies
RUN apt-get update && apt-get install -y sqlite3 libsqlite3-dev

# Set the working directory inside the container
WORKDIR /

# Copy the built binary from the builder image
COPY --from=builder /usr/src/app/target/release/crud_actix_rust /usr/local/bin/

# Set up an empty SQLite database file
RUN touch database.db

# Expose the port that Actix-Web listens on
EXPOSE 8080

# Run the Actix-Web server
CMD ["crud_actix_rust"]
