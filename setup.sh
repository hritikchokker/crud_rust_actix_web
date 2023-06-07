#!/bin/bash

# Create SQLite database
touch crud_app.db

# Generate .env file
echo "DATABASE_URL=crud_app.db" > .env
echo "ADDRESS=3000" >> .env
echo "DOMAIN=localhost" >> .env

echo "SECRET_KEY=superstrongkey >>" .env

echo "SQLite database created successfully."
echo ".env file generated."