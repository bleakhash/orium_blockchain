# Node Setup Guide

This guide covers setting up ORIUM blockchain nodes in different configurations: development, validator, and archive nodes.

## Overview

ORIUM blockchain supports multiple node types:

- **Development Node**: Single-node setup for testing and development
- **Validator Node**: Participates in consensus and block production
- **Full Node**: Syncs with the network but doesn't validate
- **Archive Node**: Stores complete blockchain history
- **Light Client**: Minimal resource usage for basic operations

## Development Node Setup

### Single Development Node

The simplest way to start experimenting with ORIUM:

```bash
# Start development node
./target/release/solochain-template-node --dev

# With custom data directory
./target/release/solochain-template-node --dev \
    --base-path /tmp/orium-dev

# With RPC access from external hosts
./target/release/solochain-template-node --dev \
    --rpc-external \
    --rpc-cors all \
    --rpc-methods unsafe
```

### Development Node Features

- Pre-funded accounts (Alice, Bob, Charlie, etc.)
- Instant block production
- No peer connections required
- Suitable for testing and development

### Accessing Development Node

```bash
# Check node health
curl -H "Content-Type: application/json" -d '{
    "id":1, 
    "jsonrpc":"2.0", 
    "method":"system_health"
}' http://localhost:9933

# Get node info
curl -H "Content-Type: application/json" -d '{
    "id":1, 
    "jsonrpc":"2.0", 
    "method":"system_name"
}' http://localhost:9933
```

## Validator Node Setup

### Prerequisites

- Stable internet connection
- Sufficient hardware resources
- Unique node key for each validator

### Hardware Requirements

**Minimum:**
- CPU: 4 cores
- RAM: 8GB
- Storage: 100GB SSD
- Network: 100 Mbps

**Recommended:**
- CPU: 8+ cores
- RAM: 16GB+
- Storage: 500GB+ NVMe SSD
- Network: 1 Gbps

### Generate Node Keys

```bash
# Generate a new node key
./target/release/solochain-template-node key generate-node-key

# Output example: 0x1234567890abcdef...
# Save this key securely - it identifies your node
```

### Create Chain Specification

```bash
# Generate local testnet spec
./target/release/solochain-template-node build-spec \
    --chain local \
    --disable-default-bootnode > local-spec.json

# Convert to raw format (required for production)
./target/release/solochain-template-node build-spec \
    --chain local-spec.json \
    --raw > local-spec-raw.json
```

### Start Validator Node

```bash
# Start validator with generated key
./target/release/solochain-template-node \
    --chain local-spec-raw.json \
    --validator \
    --base-path /data/orium-validator \
    --node-key 0x1234567890abcdef... \
    --port 30333 \
    --rpc-port 9933 \
    --name "ORIUM-Validator-1" \
    --telemetry-url "wss://telemetry.polkadot.io/submit/ 0"
```

### Validator Configuration Options

```bash
# Full validator configuration
./target/release/solochain-template-node \
    --chain local-spec-raw.json \
    --validator \
    --base-path /data/orium-validator \
    --node-key-file /secure/path/node-key \
    --keystore-path /secure/path/keystore \
    --port 30333 \
    --rpc-port 9933 \
    --ws-port 9944 \
    --prometheus-port 9615 \
    --name "ORIUM-Validator-1" \
    --max-peers 50 \
    --in-peers 25 \
    --out-peers 25 \
    --pool-limit 8192 \
    --pool-kbytes 32768 \
    --pruning 1000 \
    --state-cache-size 1073741824
```

## Multi-Validator Network

### 4-Validator Local Network

#### Validator 1 (Bootnode)

```bash
./target/release/solochain-template-node \
    --chain local-spec-raw.json \
    --validator \
    --base-path /tmp/validator1 \
    --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
    --port 30333 \
    --rpc-port 9933 \
    --name "validator-1"
```

#### Validator 2

```bash
./target/release/solochain-template-node \
    --chain local-spec-raw.json \
    --validator \
    --base-path /tmp/validator2 \
    --node-key 0000000000000000000000000000000000000000000000000000000000000002 \
    --port 30334 \
    --rpc-port 9934 \
    --name "validator-2" \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWHdiAxVd8uMQR1hGWXccidmfCwLqcMpGwR6QcTP6QRMuD
```

#### Validator 3

```bash
./target/release/solochain-template-node \
    --chain local-spec-raw.json \
    --validator \
    --base-path /tmp/validator3 \
    --node-key 0000000000000000000000000000000000000000000000000000000000000003 \
    --port 30335 \
    --rpc-port 9935 \
    --name "validator-3" \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWHdiAxVd8uMQR1hGWXccidmfCwLqcMpGwR6QcTP6QRMuD
```

#### Validator 4

```bash
./target/release/solochain-template-node \
    --chain local-spec-raw.json \
    --validator \
    --base-path /tmp/validator4 \
    --node-key 0000000000000000000000000000000000000000000000000000000000000004 \
    --port 30336 \
    --rpc-port 9936 \
    --name "validator-4" \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWHdiAxVd8uMQR1hGWXccidmfCwLqcMpGwR6QcTP6QRMuD
```

## Key Management

### Session Keys

Validators need session keys for BABE and GRANDPA consensus:

```bash
# Generate session keys
curl -H "Content-Type: application/json" -d '{
    "id":1, 
    "jsonrpc":"2.0", 
    "method":"author_rotateKeys"
}' http://localhost:9933

# Insert BABE key
./target/release/solochain-template-node key insert \
    --base-path /tmp/validator1 \
    --chain local-spec-raw.json \
    --scheme Sr25519 \
    --suri "//Alice" \
    --key-type babe

# Insert GRANDPA key
./target/release/solochain-template-node key insert \
    --base-path /tmp/validator1 \
    --chain local-spec-raw.json \
    --scheme Ed25519 \
    --suri "//Alice" \
    --key-type gran
```

### Key Security

**Production Best Practices:**

1. **Hardware Security Modules (HSM)**: Use HSM for key storage
2. **Key Rotation**: Regularly rotate session keys
3. **Backup Strategy**: Secure backup of node keys
4. **Access Control**: Limit access to key files

```bash
# Secure key file permissions
chmod 600 /secure/path/node-key
chown validator:validator /secure/path/node-key

# Create secure keystore directory
mkdir -p /secure/keystore
chmod 700 /secure/keystore
```

## Archive Node Setup

Archive nodes store complete blockchain history:

```bash
./target/release/solochain-template-node \
    --chain local-spec-raw.json \
    --base-path /data/orium-archive \
    --pruning archive \
    --rpc-external \
    --rpc-cors all \
    --rpc-methods safe \
    --name "ORIUM-Archive" \
    --max-peers 100
```

### Archive Node Benefits

- Complete transaction history
- Historical state queries
- Block explorer backend
- Research and analytics

## Monitoring and Maintenance

### Prometheus Metrics

```bash
# Enable Prometheus metrics
./target/release/solochain-template-node \
    --validator \
    --prometheus-external \
    --prometheus-port 9615
```

### Health Checks

```bash
# Node health
curl http://localhost:9933 -d '{
    "id":1, 
    "jsonrpc":"2.0", 
    "method":"system_health"
}'

# Peer count
curl http://localhost:9933 -d '{
    "id":1, 
    "jsonrpc":"2.0", 
    "method":"system_peers"
}'

# Sync status
curl http://localhost:9933 -d '{
    "id":1, 
    "jsonrpc":"2.0", 
    "method":"system_syncState"
}'
```

### Log Management

```bash
# Set log levels
export RUST_LOG=info,sc_consensus_babe=debug

# Log to file
./target/release/solochain-template-node --validator 2>&1 | tee validator.log

# Rotate logs with logrotate
sudo tee /etc/logrotate.d/orium-validator << EOF
/var/log/orium-validator.log {
    daily
    rotate 30
    compress
    delaycompress
    missingok
    notifempty
    create 644 validator validator
}
EOF
```

## Network Configuration

### Firewall Setup

```bash
# Allow P2P port
sudo ufw allow 30333/tcp

# Allow RPC port (be careful with external access)
sudo ufw allow from 10.0.0.0/8 to any port 9933

# Allow WebSocket port
sudo ufw allow from 10.0.0.0/8 to any port 9944

# Allow Prometheus metrics
sudo ufw allow from 10.0.0.0/8 to any port 9615
```

### NAT and Port Forwarding

For validators behind NAT:

```bash
# Use external IP
./target/release/solochain-template-node \
    --validator \
    --public-addr /ip4/YOUR_PUBLIC_IP/tcp/30333

# Or use automatic discovery
./target/release/solochain-template-node \
    --validator \
    --discover-local
```

## Performance Tuning

### System Optimization

```bash
# Increase file descriptor limits
echo "* soft nofile 65536" | sudo tee -a /etc/security/limits.conf
echo "* hard nofile 65536" | sudo tee -a /etc/security/limits.conf

# Optimize network settings
echo "net.core.rmem_max = 134217728" | sudo tee -a /etc/sysctl.conf
echo "net.core.wmem_max = 134217728" | sudo tee -a /etc/sysctl.conf
sudo sysctl -p
```

### Node Optimization

```bash
# High-performance configuration
./target/release/solochain-template-node \
    --validator \
    --pool-limit 8192 \
    --pool-kbytes 32768 \
    --max-runtime-instances 32 \
    --runtime-cache-size 64 \
    --state-cache-size 1073741824 \
    --db-cache 2048
```

## Troubleshooting

### Common Issues

**Node won't start:**
```bash
# Check port availability
sudo netstat -tlnp | grep :30333

# Check permissions
ls -la /data/orium-validator

# Check disk space
df -h /data
```

**Sync issues:**
```bash
# Clear database and resync
rm -rf /data/orium-validator/chains/*/db/

# Check bootnodes
./target/release/solochain-template-node \
    --validator \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/NODE_ID
```

**Performance issues:**
```bash
# Monitor resource usage
htop
iotop
nethogs

# Check node metrics
curl http://localhost:9615/metrics
```

### Recovery Procedures

**Database corruption:**
```bash
# Backup current state
cp -r /data/orium-validator /data/orium-validator.backup

# Clear corrupted database
rm -rf /data/orium-validator/chains/*/db/

# Restart node (will resync)
./target/release/solochain-template-node --validator
```

**Key recovery:**
```bash
# Restore from backup
cp /secure/backup/node-key /data/orium-validator/

# Regenerate session keys
curl -d '{"id":1,"jsonrpc":"2.0","method":"author_rotateKeys"}' \
    http://localhost:9933
```

## Systemd Service

Create a systemd service for production deployment:

```bash
# Create service file
sudo tee /etc/systemd/system/orium-validator.service << EOF
[Unit]
Description=ORIUM Validator Node
After=network.target

[Service]
Type=simple
User=validator
Group=validator
ExecStart=/usr/local/bin/solochain-template-node \\
    --chain /etc/orium/local-spec-raw.json \\
    --validator \\
    --base-path /data/orium-validator \\
    --node-key-file /etc/orium/node-key \\
    --name "ORIUM-Validator-1"
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal
SyslogIdentifier=orium-validator

[Install]
WantedBy=multi-user.target
EOF

# Enable and start service
sudo systemctl enable orium-validator
sudo systemctl start orium-validator

# Check status
sudo systemctl status orium-validator
```

## Docker Deployment

### Single Validator

```bash
# Run validator in Docker
docker run -d \
    --name orium-validator \
    -p 30333:30333 \
    -p 9933:9933 \
    -p 9944:9944 \
    -v /data/orium:/data \
    orium-node \
    --chain local \
    --validator \
    --base-path /data \
    --rpc-external \
    --rpc-cors all
```

### Multi-Validator with Docker Compose

See the provided `docker-compose.yml` for a complete 4-validator setup.

## Security Considerations

### Network Security

1. **Firewall Configuration**: Restrict RPC access
2. **VPN Access**: Use VPN for administrative access
3. **DDoS Protection**: Implement rate limiting
4. **Regular Updates**: Keep software updated

### Operational Security

1. **Key Management**: Secure key storage and rotation
2. **Access Control**: Limit administrative access
3. **Monitoring**: Continuous monitoring and alerting
4. **Backup Strategy**: Regular backups of critical data

### Incident Response

1. **Monitoring Alerts**: Set up comprehensive alerting
2. **Response Procedures**: Document incident response
3. **Recovery Plans**: Test recovery procedures
4. **Communication**: Establish communication channels

## Support and Resources

- [ORIUM Documentation](../README.md)
- [Substrate Documentation](https://docs.substrate.io/)
- [Polkadot Wiki](https://wiki.polkadot.network/)
- [Community Discord](#)
- [GitHub Issues](https://github.com/bleakhash/orium_blockchain/issues)

For additional help with node setup, please refer to the troubleshooting section or reach out to the community.
