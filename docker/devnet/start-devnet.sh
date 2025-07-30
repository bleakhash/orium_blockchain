#!/bin/bash

set -e

echo "🚀 Starting ORIUM Blockchain 4-Validator Devnet..."

if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed. Please install Docker first."
    exit 1
fi

if ! command -v docker-compose &> /dev/null; then
    echo "❌ Docker Compose is not installed. Please install Docker Compose first."
    exit 1
fi

echo "🧹 Cleaning up existing containers..."
docker-compose down -v --remove-orphans 2>/dev/null || true

echo "🔨 Building ORIUM node Docker image..."
docker-compose build --no-cache

echo "🌐 Starting 4-validator devnet..."
docker-compose up -d

echo "⏳ Waiting for nodes to initialize..."
sleep 30

echo "📊 Checking node status..."
for i in {1..4}; do
    port=$((9932 + i))
    echo "Validator $i (port $port):"
    curl -s -H "Content-Type: application/json" \
         -d '{"id":1, "jsonrpc":"2.0", "method": "system_health", "params":[]}' \
         http://localhost:$port 2>/dev/null | jq '.result' || echo "  ❌ Not ready yet"
done

echo ""
echo "✅ ORIUM Devnet is starting up!"
echo ""
echo "📡 RPC Endpoints:"
echo "  Validator 1: http://localhost:9933 (WebSocket: ws://localhost:9944)"
echo "  Validator 2: http://localhost:9934 (WebSocket: ws://localhost:9945)"
echo "  Validator 3: http://localhost:9935 (WebSocket: ws://localhost:9946)"
echo "  Validator 4: http://localhost:9936 (WebSocket: ws://localhost:9947)"
echo ""
echo "📈 Monitoring:"
echo "  Prometheus: http://localhost:9090"
echo "  Grafana: http://localhost:3000 (admin/admin)"
echo ""
echo "🔧 Useful commands:"
echo "  View logs: docker-compose logs -f"
echo "  Stop devnet: docker-compose down"
echo "  Restart devnet: docker-compose restart"
echo ""
echo "⚡ To run TPS benchmarks:"
echo "  ./benchmark-tps.sh"
