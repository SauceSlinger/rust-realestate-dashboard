#!/bin/bash

# Real Estate Dashboard Setup Script
# This script sets up the development environment for both backend and frontend

set -e

echo "========================================="
echo "Real Estate Dashboard - Setup Script"
echo "========================================="
echo ""

# Check prerequisites
echo "Checking prerequisites..."

# Check for Rust
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust is not installed. Please install from https://rustup.rs/"
    exit 1
fi
echo "✅ Rust installed: $(rustc --version)"

# Check for Node.js
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install from https://nodejs.org/"
    exit 1
fi
echo "✅ Node.js installed: $(node --version)"

# Check for npm
if ! command -v npm &> /dev/null; then
    echo "❌ npm is not installed."
    exit 1
fi
echo "✅ npm installed: $(npm --version)"

echo ""
echo "========================================="
echo "Setting up Backend"
echo "========================================="

cd backend

# Copy .env.example to .env if not exists
if [ ! -f .env ]; then
    echo "Creating .env file..."
    cp .env.example .env
    echo "✅ Created .env file (you may need to configure it)"
fi

# Create data directory
mkdir -p data
echo "✅ Created data directory"

# Install sqlx-cli if not installed
if ! command -v sqlx &> /dev/null; then
    echo "Installing sqlx-cli..."
    cargo install sqlx-cli --no-default-features --features sqlite
    echo "✅ Installed sqlx-cli"
else
    echo "✅ sqlx-cli already installed"
fi

# Create database and run migrations
echo "Setting up database..."
sqlx database create 2>/dev/null || echo "Database already exists"
sqlx migrate run
echo "✅ Database migrations completed"

cd ..

echo ""
echo "========================================="
echo "Setting up Frontend"
echo "========================================="

cd frontend

# Install dependencies
echo "Installing npm dependencies..."
npm install
echo "✅ Installed npm dependencies"

cd ..

echo ""
echo "========================================="
echo "Setup Complete! 🎉"
echo "========================================="
echo ""
echo "To start the development servers:"
echo ""
echo "Backend:"
echo "  cd backend"
echo "  cargo run"
echo "  (Server will run on http://localhost:3000)"
echo ""
echo "Frontend:"
echo "  cd frontend"
echo "  npm run dev"
echo "  (App will run on http://localhost:5173)"
echo ""
echo "Or use Docker:"
echo "  docker-compose up --build"
echo "  (Frontend: http://localhost, Backend: http://localhost:3000)"
echo ""
echo "========================================="
