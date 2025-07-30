# ORIUM Node Setup Guide

This comprehensive guide covers setting up and operating ORIUM blockchain nodes in various configurations, from development to production validator nodes.

## Table of Contents

- [Node Types](#node-types)
- [Development Setup](#development-setup)
- [Full Node Setup](#full-node-setup)
- [Validator Setup](#validator-setup)
- [Network Configuration](#network-configuration)
- [Monitoring](#monitoring)
- [Maintenance](#maintenance)
- [Troubleshooting](#troubleshooting)

## Node Types

### 1. Development Node
- Single node for testing and development
- Uses `--dev` flag with temporary storage
- Pre-funded accounts (Alice, Bob, etc.)
- Fast block production for testing

### 2. Full Node
- Participates in network without validating
- Maintains full blockchain state
- Serves RPC requests
- Helps with network decentralization

### 3. Validator Node
- Produces blocks and participates in consensus
- Requires staking and session keys
- Higher hardware requirements
- Earns rewards for honest behavior

### 4. Archive Node
- Stores complete historical state
- Larger storage requirements
- Useful for block explorers and analytics
- Slower sync but complete data

## Development Setup

### Quick Start
```bash
# Start development node with temporary storage
orium-node --dev --tmp

# Start with persistent storage
orium-node --dev --base-path ~/.local/share/orium-dev
```

### Development Configuration
```bash
orium-node \
  --dev \
  --base-path ~/.local/share/orium-dev \
  --name "ORIUM-Dev" \
  --rpc-cors all \
  --unsafe-rpc-external \
  --unsafe-ws-external \
  --rpc-methods unsafe \
  --log info,runtime::system=debug
```

### Development Features
- **Pre-funded Accounts**: Alice, Bob, Charlie, Dave, Eve, Ferdie
- **Fast Blocks**: 2-second block time
- **Instant Finality**: No need to wait for finalization
- **RPC Access**: All RPC methods enabled
- **Hot Reloading**: Runtime upgrades without restart

### Useful Development Commands
```bash
# Reset development chain
rm -rf ~/.local/share/orium-dev

# Start with custom block time
orium-node --dev --tmp --dev-block-time 1000

# Enable detailed logging
RUST_LOG=debug orium-node --dev --tmp
```

## Full Node Setup

### Basic Full Node
```bash
orium-node \
  --name "ORIUM-FullNode" \
  --chain mainnet \
  --base-path ~/.local/share/orium-node \
  --port 30333 \
  --rpc-port 9933 \
  --ws-port 9944 \
  --bootnodes /dns/bootnode1.orium.network/tcp/30333/p2p/12D3KooW... \
  --bootnodes /dns/bootnode2.orium.network/tcp/30333/p2p/12D3KooW...
```

### RPC-Enabled Full Node
```bash
orium-node \
  --name "ORIUM-RPC" \
  --chain mainnet \
  --base-path ~/.local/share/orium-node \
  --port 30333 \
  --rpc-port 9933 \
  --ws-port 9944 \
  --rpc-cors all \
  --unsafe-rpc-external \
  --unsafe-ws-external \
  --rpc-methods safe \
  --max-runtime-instances 32 \
  --rpc-max-connections 1000
```

### Archive Full Node
```bash
orium-node \
  --name "ORIUM-Archive" \
  --chain mainnet \
  --base-path ~/.local/share/orium-archive \
  --port 30333 \
  --rpc-port 9933 \
  --ws-port 9944 \
  --pruning archive \
  --state-cache-size 1073741824  # 1GB state cache
```

## Validator Setup

### Prerequisites
- Stable internet connection (99.9% uptime)
- Sufficient hardware resources
- ORM tokens for staking
- Session keys generated

### Hardware Requirements

#### Minimum
- **CPU**: 4 cores, 2.5 GHz
- **RAM**: 8 GB
- **Storage**: 200 GB SSD
- **Network**: 100 Mbps, <100ms latency

#### Recommended
- **CPU**: 8 cores, 3.0 GHz
- **RAM**: 32 GB
- **Storage**: 1 TB NVMe SSD
- **Network**: 1 Gbps, <50ms latency

### Step 1: Generate Session Keys

#### Method 1: RPC Call
```bash
# Start node first
orium-node \
  --validator \
  --name "ORIUM-Validator" \
  --chain mainnet \
  --base-path ~/.local/share/orium-validator

# Generate keys via RPC
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys", "params":[]}' \
     http://localhost:9933
```

#### Method 2: Subkey Tool
```bash
# Install subkey
cargo install --force subkey --git https://github.com/paritytech/substrate

# Generate BABE key
subkey generate --scheme sr25519 --output-type json

# Generate GRANDPA key
subkey generate --scheme ed25519 --output-type json

# Generate ImOnline key
subkey generate --scheme sr25519 --output-type json
```

### Step 2: Insert Keys into Keystore
```bash
# Insert BABE key
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "author_insertKey", "params":["babe","YOUR_SEED","YOUR_PUBLIC_KEY"]}' \
     http://localhost:9933

# Insert GRANDPA key
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "author_insertKey", "params":["gran","YOUR_SEED","YOUR_PUBLIC_KEY"]}' \
     http://localhost:9933

# Insert ImOnline key
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "author_insertKey", "params":["imon","YOUR_SEED","YOUR_PUBLIC_KEY"]}' \
     http://localhost:9933
```

### Step 3: Validator Configuration
```bash
orium-node \
  --validator \
  --name "ORIUM-Validator-1" \
  --chain mainnet \
  --base-path ~/.local/share/orium-validator \
  --port 30333 \
  --rpc-port 9933 \
  --ws-port 9944 \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
  --bootnodes /dns/bootnode1.orium.network/tcp/30333/p2p/12D3KooW... \
  --prometheus-external \
  --prometheus-port 9615
```

### Step 4: Set Session Keys On-Chain
Using Polkadot.js Apps or custom script:
```javascript
// Set session keys
const setKeys = api.tx.session.setKeys(sessionKeys, proof);
await setKeys.signAndSend(validatorAccount);
```

### Step 5: Start Validating
```javascript
// Bond tokens and nominate yourself
const bond = api.tx.staking.bond(controller, amount, 'Staked');
const validate = api.tx.staking.validate({
  commission: 1000000, // 1% commission (in parts per billion)
  blocked: false
});

const batch = api.tx.utility.batch([bond, validate]);
await batch.signAndSend(stashAccount);
```

## Network Configuration

### Firewall Setup

#### UFW (Ubuntu)
```bash
# Allow SSH
sudo ufw allow 22

# Allow P2P port
sudo ufw allow 30333

# Allow RPC (only if needed publicly)
sudo ufw allow 9933
sudo ufw allow 9944

# Enable firewall
sudo ufw enable
```

#### iptables
```bash
# Allow P2P
sudo iptables -A INPUT -p tcp --dport 30333 -j ACCEPT

# Allow RPC (be careful with public access)
sudo iptables -A INPUT -p tcp --dport 9933 -j ACCEPT
sudo iptables -A INPUT -p tcp --dport 9944 -j ACCEPT

# Save rules
sudo iptables-save > /etc/iptables/rules.v4
```

### Reverse Proxy (Nginx)

#### Install Nginx
```bash
sudo apt install nginx
```

#### Configure RPC Proxy
```nginx
# /etc/nginx/sites-available/orium-rpc
server {
    listen 80;
    server_name rpc.yourdomain.com;

    location / {
        proxy_pass http://127.0.0.1:9933;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}

# WebSocket proxy
server {
    listen 80;
    server_name ws.yourdomain.com;

    location / {
        proxy_pass http://127.0.0.1:9944;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

#### Enable Site
```bash
sudo ln -s /etc/nginx/sites-available/orium-rpc /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

### SSL/TLS with Let's Encrypt
```bash
# Install certbot
sudo apt install certbot python3-certbot-nginx

# Get certificates
sudo certbot --nginx -d rpc.yourdomain.com -d ws.yourdomain.com

# Auto-renewal
sudo crontab -e
# Add: 0 12 * * * /usr/bin/certbot renew --quiet
```

## Monitoring

### Prometheus Metrics

#### Enable Metrics
```bash
orium-node \
  --validator \
  --prometheus-external \
  --prometheus-port 9615
```

#### Prometheus Configuration
```yaml
# prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'orium-node'
    static_configs:
      - targets: ['localhost:9615']
    scrape_interval: 5s
```

### Grafana Dashboard

#### Key Metrics to Monitor
- Block height and sync status
- Peer connections
- Memory and CPU usage
- Disk I/O and space
- Network traffic
- Validator performance (if applicable)

#### Sample Queries
```promql
# Block height
substrate_block_height{instance="localhost:9615"}

# Peer count
substrate_sub_libp2p_peers_count{instance="localhost:9615"}

# Memory usage
process_resident_memory_bytes{instance="localhost:9615"}

# Validator status
substrate_node_is_active_validator{instance="localhost:9615"}
```

### Log Monitoring

#### Structured Logging
```bash
# JSON logging for better parsing
orium-node --validator --log json

# Specific log levels
orium-node --validator --log info,runtime::system=debug,babe=trace
```

#### Log Rotation
```bash
# Install logrotate configuration
sudo tee /etc/logrotate.d/orium-node << EOF
/var/log/orium-node.log {
    daily
    rotate 30
    compress
    delaycompress
    missingok
    notifempty
    create 644 orium orium
    postrotate
        systemctl reload orium-node
    endscript
}
EOF
```

## Maintenance

### System Service Setup

#### Create Service File
```bash
sudo tee /etc/systemd/system/orium-node.service << EOF
[Unit]
Description=ORIUM Blockchain Node
After=network.target

[Service]
Type=simple
User=orium
Group=orium
WorkingDirectory=/home/orium
ExecStart=/usr/local/bin/orium-node \\
  --validator \\
  --name "ORIUM-Validator" \\
  --chain mainnet \\
  --base-path /home/orium/.local/share/orium-node \\
  --port 30333 \\
  --rpc-port 9933 \\
  --ws-port 9944 \\
  --prometheus-external \\
  --prometheus-port 9615
Restart=always
RestartSec=10
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
EOF
```

#### Enable and Start Service
```bash
sudo systemctl daemon-reload
sudo systemctl enable orium-node
sudo systemctl start orium-node
sudo systemctl status orium-node
```

### Backup Procedures

#### Database Backup
```bash
# Stop node
sudo systemctl stop orium-node

# Create backup
tar -czf orium-backup-$(date +%Y%m%d).tar.gz ~/.local/share/orium-node/

# Restart node
sudo systemctl start orium-node
```

#### Key Backup
```bash
# Backup keystore
cp -r ~/.local/share/orium-node/keystore ~/orium-keystore-backup

# Backup node key
cp ~/.local/share/orium-node/network/secret_ed25519 ~/node-key-backup
```

### Updates

#### Binary Updates
```bash
# Stop node
sudo systemctl stop orium-node

# Backup current binary
sudo cp /usr/local/bin/orium-node /usr/local/bin/orium-node.backup

# Install new binary
sudo cp target/release/orium-node /usr/local/bin/

# Start node
sudo systemctl start orium-node

# Check logs
sudo journalctl -u orium-node -f
```

#### Runtime Updates
Runtime updates are performed on-chain via governance and don't require node restarts.

## Troubleshooting

### Common Issues

#### 1. Sync Problems
```bash
# Check sync status
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "system_syncState", "params":[]}' \
     http://localhost:9933

# Force resync
orium-node --validator --force-authoring --unsafe-force-authoring
```

#### 2. Peer Connection Issues
```bash
# Check peer count
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "system_health", "params":[]}' \
     http://localhost:9933

# Add more bootnodes
orium-node --validator --bootnodes /ip4/1.2.3.4/tcp/30333/p2p/12D3KooW...
```

#### 3. Validator Not Producing Blocks
```bash
# Check if keys are in keystore
ls ~/.local/share/orium-node/keystore/

# Verify session keys
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "author_hasSessionKeys", "params":["YOUR_SESSION_KEYS"]}' \
     http://localhost:9933
```

#### 4. High Resource Usage
```bash
# Monitor resources
htop
iotop
nethogs

# Adjust cache sizes
orium-node --validator --state-cache-size 268435456  # 256MB
```

### Performance Tuning

#### Database Optimization
```bash
# Use faster database backend
orium-node --validator --database rocksdb

# Adjust cache sizes
orium-node --validator \
  --state-cache-size 1073741824 \  # 1GB state cache
  --db-cache 2048                  # 2GB database cache
```

#### Network Optimization
```bash
# Increase connection limits
orium-node --validator \
  --in-peers 50 \
  --out-peers 50 \
  --max-runtime-instances 32
```

### Getting Help

1. **Check Logs**: `sudo journalctl -u orium-node -f`
2. **Community Support**: [Discord](https://discord.gg/orium)
3. **GitHub Issues**: [Report problems](https://github.com/your-org/orium-blockchain/issues)
4. **Documentation**: [Full docs](https://docs.orium.network)

## Security Best Practices

1. **Keep Software Updated**: Regular updates for security patches
2. **Secure Key Management**: Hardware security modules for production
3. **Network Security**: Proper firewall configuration
4. **Monitoring**: 24/7 monitoring for validator nodes
5. **Backup Strategy**: Regular backups of keys and data
6. **Access Control**: Limit SSH access and use key-based authentication

## Next Steps

- [Token Usage Guide](TOKEN_USAGE.md)
- [Stablecoin Operations](STABLECOIN_GUIDE.md)
- [Development Guide](DEVELOPMENT.md)
- [API Reference](API_REFERENCE.md)
