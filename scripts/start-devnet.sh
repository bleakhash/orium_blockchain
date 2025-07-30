#!/bin/bash

set -e

echo "🚀 Starting ORIUM 4-Validator Devnet..."

if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed. Please install Docker first."
    exit 1
fi

if ! command -v docker-compose &> /dev/null; then
    echo "❌ Docker Compose is not installed. Please install Docker Compose first."
    exit 1
fi

echo "🧹 Cleaning up existing containers..."
docker-compose down -v 2>/dev/null || true

echo "🔨 Building Docker images..."
docker-compose build

echo "🌐 Starting 4-validator devnet..."
docker-compose up -d

echo "⏳ Waiting for validators to start..."
sleep 30

echo "🔍 Checking validator status..."
for i in {1..4}; do
    if docker ps | grep -q "orium-validator-$i"; then
        echo "✅ Validator $i is running"
    else
        echo "❌ Validator $i failed to start"
        docker-compose logs validator-$i
        exit 1
    fi
done

echo ""
echo "🎉 ORIUM Devnet is now running!"
echo ""
echo "📊 Access Points:"
echo "  • Validator 1 RPC: http://localhost:9933"
echo "  • Validator 1 WebSocket: ws://localhost:9944"
echo "  • Validator 2 RPC: http://localhost:9934"
echo "  • Validator 2 WebSocket: ws://localhost:9945"
echo "  • Validator 3 RPC: http://localhost:9935"
echo "  • Validator 3 WebSocket: ws://localhost:9946"
echo "  • Validator 4 RPC: http://localhost:9936"
echo "  • Validator 4 WebSocket: ws://localhost:9947"
echo "  • Prometheus: http://localhost:9090"
echo "  • Grafana: http://localhost:3000 (admin/admin)"
echo ""
echo "🔧 Useful Commands:"
echo "  • View logs: docker-compose logs -f"
echo "  • Stop devnet: docker-compose down"
echo "  • Restart devnet: docker-compose restart"
echo "  • Clean restart: docker-compose down -v && docker-compose up -d"
echo ""
echo "📈 To run TPS benchmarks:"
echo "  ./scripts/benchmark-tps.sh"
