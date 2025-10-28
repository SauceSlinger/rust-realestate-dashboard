#!/bin/bash

# Real Estate Dashboard Run Script

set -e

echo "🏠 Real Estate Dashboard Launcher"
echo "================================="
echo ""

# Check if running in Docker
if [ "$1" == "docker" ]; then
    echo "🐳 Starting with Docker Compose..."
    docker-compose up --build
    exit 0
fi

# Check if dependencies are installed
check_rust() {
    if ! command -v cargo &> /dev/null; then
        echo "❌ Rust is not installed. Please install from https://rustup.rs/"
        exit 1
    fi
    echo "✅ Rust found"
}

check_node() {
    if ! command -v node &> /dev/null; then
        echo "❌ Node.js is not installed. Please install from https://nodejs.org/"
        exit 1
    fi
    echo "✅ Node.js found"
}

# Install backend dependencies
setup_backend() {
    echo ""
    echo "📦 Setting up backend..."
    cd backend
    if [ ! -d "target" ]; then
        cargo build
    fi
    cd ..
}

# Install frontend dependencies
setup_frontend() {
    echo ""
    echo "📦 Setting up frontend..."
    cd frontend
    if [ ! -d "node_modules" ]; then
        npm install
    fi
    cd ..
}

# Run the application
run_app() {
    echo ""
    echo "🚀 Starting application..."
    echo ""
    
    # Start backend
    echo "Starting backend server on http://localhost:3000"
    cd backend
    cargo run &
    BACKEND_PID=$!
    cd ..
    
    # Wait for backend to be ready
    sleep 3
    
    # Start frontend
    echo "Starting frontend server on http://localhost:5173"
    cd frontend
    npm run dev &
    FRONTEND_PID=$!
    cd ..
    
    echo ""
    echo "✅ Application is running!"
    echo "   Frontend: http://localhost:5173"
    echo "   Backend API: http://localhost:3000/api"
    echo ""
    echo "Press Ctrl+C to stop..."
    
    # Wait for Ctrl+C
    trap "echo ''; echo 'Stopping servers...'; kill $BACKEND_PID $FRONTEND_PID; exit 0" INT
    wait
}

# Main execution
check_rust
check_node
setup_backend
setup_frontend
run_app
