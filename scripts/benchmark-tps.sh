#!/bin/bash

set -e

echo "📊 ORIUM TPS Benchmark Tool"
echo "=========================="

if ! docker ps | grep -q "orium-validator-1"; then
    echo "❌ Devnet is not running. Please start it first with:"
    echo "   ./scripts/start-devnet.sh"
    exit 1
fi

RPC_URL="http://localhost:9933"
DURATION=${1:-60}  # Default 60 seconds
CONCURRENT_USERS=${2:-10}  # Default 10 concurrent users
TRANSACTIONS_PER_USER=${3:-100}  # Default 100 transactions per user

echo "🔧 Benchmark Configuration:"
echo "  • Duration: ${DURATION} seconds"
echo "  • Concurrent Users: ${CONCURRENT_USERS}"
echo "  • Transactions per User: ${TRANSACTIONS_PER_USER}"
echo "  • Target RPC: ${RPC_URL}"
echo ""

send_transactions() {
    local user_id=$1
    local tx_count=$2
    local success_count=0
    
    echo "👤 User ${user_id}: Starting ${tx_count} transactions..."
    
    for ((i=1; i<=tx_count; i++)); do
        response=$(curl -s -X POST \
            -H "Content-Type: application/json" \
            -d '{
                "jsonrpc": "2.0",
                "method": "author_submitExtrinsic",
                "params": ["0x280403000b63ce64c10c0542"],
                "id": 1
            }' \
            ${RPC_URL} 2>/dev/null)
        
        if [[ $response == *"result"* ]]; then
            ((success_count++))
        fi
        
        sleep 0.01
    done
    
    echo "👤 User ${user_id}: Completed ${success_count}/${tx_count} transactions"
    echo ${success_count}
}

echo "🚀 Starting TPS benchmark..."
start_time=$(date +%s)

pids=()
temp_files=()

for ((user=1; user<=CONCURRENT_USERS; user++)); do
    temp_file=$(mktemp)
    temp_files+=("$temp_file")
    
    (send_transactions $user $TRANSACTIONS_PER_USER > "$temp_file") &
    pids+=($!)
done

echo "⏳ Running benchmark for ${DURATION} seconds..."
sleep $DURATION

for pid in "${pids[@]}"; do
    if kill -0 $pid 2>/dev/null; then
        kill $pid 2>/dev/null || true
    fi
done

wait 2>/dev/null || true

end_time=$(date +%s)
actual_duration=$((end_time - start_time))

total_successful=0
for temp_file in "${temp_files[@]}"; do
    if [[ -f "$temp_file" ]]; then
        success_count=$(tail -1 "$temp_file" | grep -o '[0-9]*' | tail -1)
        if [[ -n "$success_count" ]]; then
            total_successful=$((total_successful + success_count))
        fi
        rm -f "$temp_file"
    fi
done

if [[ $actual_duration -gt 0 ]]; then
    tps=$((total_successful / actual_duration))
else
    tps=0
fi

echo ""
echo "📈 Benchmark Results:"
echo "====================="
echo "  • Total Successful Transactions: ${total_successful}"
echo "  • Actual Duration: ${actual_duration} seconds"
echo "  • Transactions Per Second (TPS): ${tps}"
echo "  • Concurrent Users: ${CONCURRENT_USERS}"
echo ""

if [[ $tps -ge 50000 ]]; then
    echo "🎉 EXCELLENT: TPS target of 50,000+ achieved!"
elif [[ $tps -ge 10000 ]]; then
    echo "✅ GOOD: High TPS achieved (${tps})"
elif [[ $tps -ge 1000 ]]; then
    echo "⚠️  MODERATE: Decent TPS but below target (${tps})"
else
    echo "❌ LOW: TPS below expectations (${tps})"
fi

echo ""
echo "💡 Tips to improve TPS:"
echo "  • Increase block size limits in runtime configuration"
echo "  • Optimize transaction weights"
echo "  • Use batch transactions"
echo "  • Increase concurrent users: ./scripts/benchmark-tps.sh 60 20 100"
