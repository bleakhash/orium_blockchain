# API Reference

Complete API reference for ORIUM blockchain RPC methods and WebSocket subscriptions.

## Overview

ORIUM blockchain provides JSON-RPC API endpoints for interacting with the network. The API includes standard Substrate methods plus custom methods for ORIUM-specific functionality.

## Connection Endpoints

- **HTTP RPC**: `http://localhost:9933`
- **WebSocket**: `ws://localhost:9944`
- **Prometheus Metrics**: `http://localhost:9615/metrics`

## Standard Substrate APIs

### System Methods

#### system_name
Get the node implementation name.

```bash
curl -H "Content-Type: application/json" -d '{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "system_name"
}' http://localhost:9933
```

Response:
```json
{
    "jsonrpc": "2.0",
    "result": "ORIUM Node",
    "id": 1
}
```

#### system_version
Get the node implementation version.

```bash
curl -H "Content-Type: application/json" -d '{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "system_version"
}' http://localhost:9933
```

#### system_chain
Get the chain name.

```bash
curl -H "Content-Type: application/json" -d '{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "system_chain"
}' http://localhost:9933
```

#### system_health
Get node health status.

```bash
curl -H "Content-Type: application/json" -d '{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "system_health"
}' http://localhost:9933
```

Response:
```json
{
    "jsonrpc": "2.0",
    "result": {
        "peers": 3,
        "isSyncing": false,
        "shouldHavePeers": true
    },
    "id": 1
}
```

#### system_peers
Get connected peers information.

```bash
curl -H "Content-Type: application/json" -d '{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "system_peers"
}' http://localhost:9933
```

#### system_syncState
Get synchronization state.

```bash
curl -H "Content-Type: application/json" -d '{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "system_syncState"
}' http://localhost:9933
```

### Chain Methods

#### chain_getBlock
Get block by hash.

```bash
curl -H "Content-Type: application/json" -d '{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "chain_getBlock",
    "params": ["0x1234..."]
}' http://localhost:9933
```

#### chain_getBlockHash
Get block hash by number.

```bash
curl -H "Content-Type: application/json" -d '{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "chain_getBlockHash",
    "params": [100]
}' http://localhost:9933
```

#### chain_getHeader
Get block header.

```bash
curl -H "Content-Type: application/json" -d '{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "chain_getHeader",
    "params": ["0x1234..."]
}' http://localhost:9933
```

#### chain_getFinalizedHead
Get finalized block hash.

```bash
curl -H "Content-Type: application/json" -d '{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "chain_getFinalizedHead"
}' http://localhost:9933
```

### State Methods

#### state_getStorage
Get storage value by key.

```bash
curl -H "Content-Type: application/json" -d '{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "state_getStorage",
    "params": ["0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9"]
}' http://localhost:9933
```

#### state_getMetadata
Get runtime metadata.

```bash
curl -H "Content-Type: application/json" -d '{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "state_getMetadata"
}' http://localhost:9933
```

#### state_getRuntimeVersion
Get runtime version.

```bash
curl -H "Content-Type: application/json" -d '{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "state_getRuntimeVersion"
}' http://localhost:9933
```

### Author Methods

#### author_submitExtrinsic
Submit signed extrinsic.

```bash
curl -H "Content-Type: application/json" -d '{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "author_submitExtrinsic",
    "params": ["0x280403000b63ce64c10c0542"]
}' http://localhost:9933
```

#### author_pendingExtrinsics
Get pending extrinsics.

```bash
curl -H "Content-Type: application/json" -d '{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "author_pendingExtrinsics"
}' http://localhost:9933
```

#### author_rotateKeys
Generate new session keys.

```bash
curl -H "Content-Type: application/json" -d '{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "author_rotateKeys"
}' http://localhost:9933
```

## ORIUM-Specific APIs

### Balance Methods

#### system_account
Get account information including ORM balance.

```bash
curl -H "Content-Type: application/json" -d '{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "system_account",
    "params": ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
}' http://localhost:9933
```

Response:
```json
{
    "jsonrpc": "2.0",
    "result": {
        "nonce": 0,
        "consumers": 0,
        "providers": 1,
        "sufficients": 0,
        "data": {
            "free": "1000000000000000000",
            "reserved": "0",
            "frozen": "0",
            "flags": "0x80000000000000000000000000000000"
        }
    },
    "id": 1
}
```

### Stablecoin Methods

#### Query dUSD Balance

```bash
curl -H "Content-Type: application/json" -d '{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "state_call",
    "params": [
        "AssetsApi_balance",
        "0x01000000" + "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
    ]
}' http://localhost:9933
```

#### Query dEUR Balance

```bash
curl -H "Content-Type: application/json" -d '{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "state_call",
    "params": [
        "AssetsApi_balance",
        "0x02000000" + "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
    ]
}' http://localhost:9933
```

### Collateral Engine Methods

#### Query Vault Information

```bash
curl -H "Content-Type: application/json" -d '{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "state_getStorage",
    "params": ["0x" + blake2_256("CollateralEngine Vaults") + vault_id]
}' http://localhost:9933
```

#### Query Collateral Prices

```bash
curl -H "Content-Type: application/json" -d '{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "state_getStorage",
    "params": ["0x" + blake2_256("CollateralEngine Prices") + asset_pair]
}' http://localhost:9933
```

### Benchmarking Methods

#### Query TPS Measurements

```bash
curl -H "Content-Type: application/json" -d '{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "state_getStorage",
    "params": ["0x" + blake2_256("Benchmarking TpsMeasurements")]
}' http://localhost:9933
```

## WebSocket Subscriptions

### Block Subscriptions

#### Subscribe to New Blocks

```javascript
const WebSocket = require('ws');
const ws = new WebSocket('ws://localhost:9944');

ws.on('open', function() {
    ws.send(JSON.stringify({
        "id": 1,
        "jsonrpc": "2.0",
        "method": "chain_subscribeNewHeads"
    }));
});

ws.on('message', function(data) {
    const response = JSON.parse(data);
    console.log('New block:', response);
});
```

#### Subscribe to Finalized Blocks

```javascript
ws.send(JSON.stringify({
    "id": 2,
    "jsonrpc": "2.0",
    "method": "chain_subscribeFinalizedHeads"
}));
```

### Event Subscriptions

#### Subscribe to System Events

```javascript
ws.send(JSON.stringify({
    "id": 3,
    "jsonrpc": "2.0",
    "method": "state_subscribeStorage",
    "params": [["0x26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7"]]
}));
```

#### Subscribe to Balance Changes

```javascript
const accountId = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
const storageKey = "0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9" + accountId;

ws.send(JSON.stringify({
    "id": 4,
    "jsonrpc": "2.0",
    "method": "state_subscribeStorage",
    "params": [[storageKey]]
}));
```

## Polkadot.js API Examples

### Basic Setup

```javascript
const { ApiPromise, WsProvider } = require('@polkadot/api');

async function main() {
    const wsProvider = new WsProvider('ws://localhost:9944');
    const api = await ApiPromise.create({ provider: wsProvider });
    
    console.log('Connected to ORIUM blockchain');
    
    // Your code here
    
    await api.disconnect();
}

main().catch(console.error);
```

### Query Chain State

```javascript
async function queryChainState() {
    const api = await ApiPromise.create({ provider: wsProvider });
    
    // Get chain info
    const chain = await api.rpc.system.chain();
    const version = await api.rpc.system.version();
    const health = await api.rpc.system.health();
    
    console.log(`Chain: ${chain}`);
    console.log(`Version: ${version}`);
    console.log(`Health: ${JSON.stringify(health)}`);
    
    // Get latest block
    const lastHeader = await api.rpc.chain.getHeader();
    console.log(`Latest block: #${lastHeader.number}`);
}
```

### Account Operations

```javascript
const { Keyring } = require('@polkadot/keyring');

async function accountOperations() {
    const api = await ApiPromise.create({ provider: wsProvider });
    const keyring = new Keyring({ type: 'sr25519' });
    
    // Add accounts
    const alice = keyring.addFromUri('//Alice');
    const bob = keyring.addFromUri('//Bob');
    
    // Get balances
    const aliceBalance = await api.query.system.account(alice.address);
    const bobBalance = await api.query.system.account(bob.address);
    
    console.log(`Alice balance: ${aliceBalance.data.free}`);
    console.log(`Bob balance: ${bobBalance.data.free}`);
    
    // Transfer
    const transfer = api.tx.balances.transfer(bob.address, 1000000000000);
    const hash = await transfer.signAndSend(alice);
    
    console.log(`Transfer hash: ${hash}`);
}
```

### Event Monitoring

```javascript
async function monitorEvents() {
    const api = await ApiPromise.create({ provider: wsProvider });
    
    // Subscribe to system events
    api.query.system.events((events) => {
        console.log(`\nReceived ${events.length} events:`);
        
        events.forEach((record) => {
            const { event, phase } = record;
            const types = event.typeDef;
            
            console.log(`\t${event.section}:${event.method}:: (phase=${phase.toString()})`);
            console.log(`\t\t${event.meta.documentation.toString()}`);
            
            event.data.forEach((data, index) => {
                console.log(`\t\t\t${types[index].type}: ${data.toString()}`);
            });
        });
    });
}
```

### Custom Extrinsics

```javascript
async function customExtrinsics() {
    const api = await ApiPromise.create({ provider: wsProvider });
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');
    
    // Create vault
    const createVault = api.tx.collateralEngine.createVault(
        'ORM',
        1000000000000000 // 1000 ORM
    );
    
    const vaultHash = await createVault.signAndSend(alice);
    console.log(`Vault creation hash: ${vaultHash}`);
    
    // Mint stablecoin
    const mintStablecoin = api.tx.collateralEngine.mintStablecoin(
        1, // vault_id
        'dUSD',
        500000000000000 // 500 dUSD
    );
    
    const mintHash = await mintStablecoin.signAndSend(alice);
    console.log(`Mint hash: ${mintHash}`);
}
```

## Error Handling

### Common Error Codes

| Code | Message | Description |
|------|---------|-------------|
| -32700 | Parse error | Invalid JSON |
| -32600 | Invalid Request | Invalid request object |
| -32601 | Method not found | Method doesn't exist |
| -32602 | Invalid params | Invalid method parameters |
| -32603 | Internal error | Internal JSON-RPC error |
| 1000 | Invalid transaction | Transaction validation failed |
| 1001 | Unknown transaction | Transaction not found |
| 1002 | Invalid block | Block validation failed |

### Error Response Format

```json
{
    "jsonrpc": "2.0",
    "error": {
        "code": -32602,
        "message": "Invalid params",
        "data": "Expected 1 parameter, got 0"
    },
    "id": 1
}
```

### Handling Errors in JavaScript

```javascript
async function handleErrors() {
    try {
        const api = await ApiPromise.create({ provider: wsProvider });
        const result = await api.rpc.chain.getBlock('invalid_hash');
    } catch (error) {
        if (error.code) {
            console.log(`RPC Error ${error.code}: ${error.message}`);
        } else {
            console.log(`Network Error: ${error.message}`);
        }
    }
}
```

## Rate Limiting

### Default Limits

- **HTTP RPC**: 100 requests/minute per IP
- **WebSocket**: 50 connections per IP
- **Subscription**: 10 subscriptions per connection

### Custom Rate Limiting

```bash
# Start node with custom limits
./target/release/solochain-template-node \
    --rpc-max-connections 200 \
    --rpc-max-subscriptions-per-connection 20 \
    --rpc-max-request-size 1048576
```

## Security Considerations

### RPC Security

1. **Network Access**: Restrict RPC access to trusted networks
2. **Method Filtering**: Use `--rpc-methods safe` in production
3. **CORS Policy**: Configure appropriate CORS settings
4. **Rate Limiting**: Implement rate limiting for public endpoints

### Safe vs Unsafe Methods

**Safe Methods** (read-only):
- `system_*`
- `chain_*`
- `state_*`

**Unsafe Methods** (can modify state):
- `author_*`
- Custom transaction methods

### Production Configuration

```bash
# Production RPC configuration
./target/release/solochain-template-node \
    --validator \
    --rpc-methods safe \
    --rpc-cors "https://yourdomain.com" \
    --rpc-max-connections 100 \
    --no-private-ipv4
```

## Performance Optimization

### Connection Pooling

```javascript
// Use connection pooling for high-throughput applications
const { ApiPromise, WsProvider } = require('@polkadot/api');

class ConnectionPool {
    constructor(endpoint, poolSize = 5) {
        this.endpoint = endpoint;
        this.poolSize = poolSize;
        this.connections = [];
        this.currentIndex = 0;
    }
    
    async initialize() {
        for (let i = 0; i < this.poolSize; i++) {
            const provider = new WsProvider(this.endpoint);
            const api = await ApiPromise.create({ provider });
            this.connections.push(api);
        }
    }
    
    getConnection() {
        const connection = this.connections[this.currentIndex];
        this.currentIndex = (this.currentIndex + 1) % this.poolSize;
        return connection;
    }
}
```

### Batch Requests

```javascript
// Batch multiple queries
async function batchQueries() {
    const api = await ApiPromise.create({ provider: wsProvider });
    
    const queries = [
        api.query.system.account.multi(['5GrwvaEF...', '5FHneW46...']),
        api.query.timestamp.now(),
        api.query.system.number()
    ];
    
    const results = await Promise.all(queries);
    console.log('Batch results:', results);
}
```

## Testing and Development

### Local Testing

```javascript
// Mock API for testing
const { MockProvider } = require('@polkadot/api/mock');

async function testWithMock() {
    const provider = new MockProvider();
    const api = await ApiPromise.create({ provider });
    
    // Your test code here
}
```

### Integration Testing

```javascript
// Integration test example
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { Keyring } = require('@polkadot/keyring');

describe('ORIUM API Integration', () => {
    let api;
    let keyring;
    
    beforeAll(async () => {
        const provider = new WsProvider('ws://localhost:9944');
        api = await ApiPromise.create({ provider });
        keyring = new Keyring({ type: 'sr25519' });
    });
    
    afterAll(async () => {
        await api.disconnect();
    });
    
    test('should transfer ORM tokens', async () => {
        const alice = keyring.addFromUri('//Alice');
        const bob = keyring.addFromUri('//Bob');
        
        const transfer = api.tx.balances.transfer(bob.address, 1000000000000);
        const hash = await transfer.signAndSend(alice);
        
        expect(hash).toBeDefined();
    });
});
```

## Support and Resources

- [Polkadot.js Documentation](https://polkadot.js.org/docs/)
- [Substrate RPC Documentation](https://docs.substrate.io/reference/command-line-tools/subxt/)
- [JSON-RPC Specification](https://www.jsonrpc.org/specification)
- [WebSocket API](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)

For API-specific questions, please refer to the [GitHub Issues](https://github.com/bleakhash/orium_blockchain/issues) or join our community discussions.
