# ORIUM Blockchain API Reference

Complete API reference for the ORIUM blockchain, including RPC methods, extrinsics, storage queries, and events.

## Table of Contents

- [Connection & Setup](#connection--setup)
- [System APIs](#system-apis)
- [Chain APIs](#chain-apis)
- [State APIs](#state-apis)
- [ORIUM Token APIs](#orium-token-apis)
- [Collateral Engine APIs](#collateral-engine-apis)
- [Stablecoin APIs](#stablecoin-apis)
- [Events](#events)
- [Error Codes](#error-codes)
- [Examples](#examples)

## Connection & Setup

### WebSocket Connection
```javascript
import { ApiPromise, WsProvider } from '@polkadot/api';

// Connect to local node
const wsProvider = new WsProvider('ws://localhost:9944');
const api = await ApiPromise.create({ provider: wsProvider });

// Connect to remote node
const wsProvider = new WsProvider('wss://rpc.orium.network');
const api = await ApiPromise.create({ provider: wsProvider });
```

### HTTP Connection
```bash
# Using curl
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "system_health", "params":[]}' \
     http://localhost:9933
```

## System APIs

### system_health
Get node health status.

**Method**: `system_health`  
**Parameters**: None  
**Returns**: `SystemHealth`

```javascript
const health = await api.rpc.system.health();
console.log(health.toHuman());
// Response: { "isSyncing": false, "peers": 12, "shouldHavePeers": true }
```

### system_name
Get node implementation name.

```javascript
const name = await api.rpc.system.name();
console.log(name.toString()); // "ORIUM Node"
```

### system_version
Get node version.

```javascript
const version = await api.rpc.system.version();
console.log(version.toString()); // "1.0.0"
```

## Chain APIs

### chain_getBlock
Get block by hash.

```javascript
// Get latest block
const block = await api.rpc.chain.getBlock();

// Get specific block
const blockHash = '0x1234...';
const block = await api.rpc.chain.getBlock(blockHash);
```

### chain_getBlockHash
Get block hash by number.

```javascript
// Get latest block hash
const hash = await api.rpc.chain.getBlockHash();

// Get specific block hash
const hash = await api.rpc.chain.getBlockHash(12345);
```

## ORIUM Token APIs

### Storage Queries

#### balances
```javascript
const balance = await api.query.oriumToken.balances('orAccountAddress...');
console.log(balance.toString());
```

#### totalSupply
```javascript
const totalSupply = await api.query.oriumToken.totalSupply();
console.log(`Total Supply: ${totalSupply.toString()}`);
```

#### allowances
```javascript
const allowance = await api.query.oriumToken.allowances(
  'orOwnerAddress...',
  'orSpenderAddress...'
);
```

### Extrinsics

#### transfer
```javascript
const transfer = api.tx.oriumToken.transfer(
  'orRecipientAddress...',
  1000 * 10**18  // 1000 ORM
);
await transfer.signAndSend(senderAccount);
```

#### approve
```javascript
const approve = api.tx.oriumToken.approve(
  'orSpenderAddress...',
  500 * 10**18  // 500 ORM allowance
);
await approve.signAndSend(ownerAccount);
```

#### mint
```javascript
const mint = api.tx.oriumToken.mint(
  'orRecipientAddress...',
  1000 * 10**18  // 1000 ORM
);
await mint.signAndSend(sudoAccount);
```

## Collateral Engine APIs

### Storage Queries

#### cdps
```javascript
const cdp = await api.query.collateralEngine.cdps('orAccountAddress...');
if (cdp.isSome) {
  const cdpData = cdp.unwrap();
  console.log(`Collateral: ${cdpData.collateral.toString()}`);
  console.log(`dUSD Debt: ${cdpData.dusdDebt.toString()}`);
  console.log(`dEUR Debt: ${cdpData.deurDebt.toString()}`);
}
```

#### ormUsdPrice
```javascript
const ormUsdPrice = await api.query.collateralEngine.ormUsdPrice();
console.log(`ORM/USD Price: $${(ormUsdPrice / 10**18).toFixed(4)}`);
```

### Extrinsics

#### createCdp
```javascript
const createCdp = api.tx.collateralEngine.createCdp(
  10000 * 10**18  // 10,000 ORM collateral
);
await createCdp.signAndSend(account);
```

#### mintDusd
```javascript
const mintDusd = api.tx.collateralEngine.mintDusd(
  5000 * 10**18  // 5,000 dUSD
);
await mintDusd.signAndSend(account);
```

#### liquidate
```javascript
const liquidate = api.tx.collateralEngine.liquidate('orCdpOwnerAddress...');
await liquidate.signAndSend(liquidatorAccount);
```

## Stablecoin APIs

### dUSD APIs
```javascript
// Transfer dUSD
const transferDusd = api.tx.dusd.transfer(
  'orRecipientAddress...',
  1000 * 10**18
);
await transferDusd.signAndSend(account);

// Get dUSD balance
const dusdBalance = await api.query.dusd.balances('orAccountAddress...');
```

### dEUR APIs
```javascript
// Transfer dEUR
const transferDeur = api.tx.deur.transfer(
  'orRecipientAddress...',
  500 * 10**18
);
await transferDeur.signAndSend(account);

// Get dEUR balance
const deurBalance = await api.query.deur.balances('orAccountAddress...');
```

## Events

### ORIUM Token Events

#### Transferred
```javascript
api.query.system.events((events) => {
  events.forEach((record) => {
    const { event } = record;
    if (event.method === 'Transferred' && event.section === 'oriumToken') {
      const [from, to, amount] = event.data;
      console.log(`Transfer: ${from} -> ${to}, Amount: ${amount}`);
    }
  });
});
```

#### Minted
```javascript
// Event: Minted { to: AccountId, amount: Balance }
```

#### Burned
```javascript
// Event: Burned { from: AccountId, amount: Balance }
```

### Collateral Engine Events

#### CdpCreated
```javascript
// Event: CdpCreated { account: AccountId, collateral: Balance }
```

#### DusdMinted
```javascript
// Event: DusdMinted { account: AccountId, amount: Balance }
```

#### CdpLiquidated
```javascript
// Event: CdpLiquidated { account: AccountId, liquidator: AccountId }
```

## Error Codes

### ORIUM Token Errors
- `InsufficientBalance`: Not enough tokens for operation
- `InsufficientAllowance`: Not enough allowance for transfer
- `Overflow`: Arithmetic overflow in calculation

### Collateral Engine Errors
- `CdpAlreadyExists`: CDP already exists for account
- `CdpNotFound`: No CDP found for account
- `InsufficientCollateral`: Not enough collateral for operation
- `CollateralRatioTooLow`: Collateral ratio below minimum
- `NotLiquidatable`: CDP cannot be liquidated

## Examples

### Complete CDP Workflow
```javascript
import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';

async function cdpWorkflow() {
  // Connect to node
  const wsProvider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({ provider: wsProvider });
  
  // Create account
  const keyring = new Keyring({ type: 'sr25519' });
  const account = keyring.addFromUri('//Alice');
  
  // 1. Create CDP with collateral
  const createCdp = api.tx.collateralEngine.createCdp(10000 * 10**18);
  await createCdp.signAndSend(account);
  
  // 2. Mint stablecoins
  const mintDusd = api.tx.collateralEngine.mintDusd(5000 * 10**18);
  await mintDusd.signAndSend(account);
  
  // 3. Check CDP status
  const cdp = await api.query.collateralEngine.cdps(account.address);
  console.log('CDP Status:', cdp.toHuman());
  
  // 4. Transfer stablecoins
  const transferDusd = api.tx.dusd.transfer('orRecipient...', 1000 * 10**18);
  await transferDusd.signAndSend(account);
  
  await api.disconnect();
}
```

### Batch Operations
```javascript
async function batchOperations() {
  const operations = [
    api.tx.oriumToken.transfer('orAddr1...', 100 * 10**18),
    api.tx.dusd.transfer('orAddr2...', 50 * 10**18),
    api.tx.collateralEngine.depositCollateral(1000 * 10**18)
  ];
  
  const batch = api.tx.utility.batch(operations);
  await batch.signAndSend(account);
}
```

### Event Monitoring
```javascript
async function monitorEvents() {
  api.query.system.events((events) => {
    events.forEach((record) => {
      const { event, phase } = record;
      
      console.log(`Event: ${event.section}.${event.method}`);
      console.log(`Phase: ${phase.toString()}`);
      console.log(`Data: ${event.data.toString()}`);
    });
  });
}
```

For complete examples and integration guides, see [TOKEN_USAGE.md](TOKEN_USAGE.md).
