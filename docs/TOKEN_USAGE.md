# Token Usage Guide

This guide explains how to use ORIUM's native token (ORM) and stablecoins (dUSD, dEUR) through RPC calls and the Polkadot.js interface.

## Overview

ORIUM blockchain features three main tokens:

- **ORM**: Native token used for fees, governance, and collateral
- **dUSD**: USD-pegged stablecoin backed by ORM collateral
- **dEUR**: EUR-pegged stablecoin backed by ORM collateral

## ORIUM (ORM) Native Token

### Token Properties

- **Symbol**: ORM
- **Decimals**: 12
- **Address Prefix**: "or" (SS58 format: 111)
- **Total Supply**: Configurable via governance

### Basic Operations

#### Check Balance

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

#### Transfer ORM Tokens

```javascript
// Using Polkadot.js API
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');

async function transferORM() {
    const wsProvider = new WsProvider('ws://localhost:9944');
    const api = await ApiPromise.create({ provider: wsProvider });
    
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');
    const bob = keyring.addFromUri('//Bob');
    
    // Transfer 1 ORM (1 * 10^12 units)
    const transfer = api.tx.balances.transfer(bob.address, 1000000000000);
    
    const hash = await transfer.signAndSend(alice);
    console.log('Transfer hash:', hash.toHex());
}
```

#### Mint ORM Tokens (Sudo Only)

```javascript
async function mintORM() {
    const api = await ApiPromise.create({ provider: wsProvider });
    const keyring = new Keyring({ type: 'sr25519' });
    const sudo = keyring.addFromUri('//Alice'); // Sudo account
    
    const recipient = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
    const amount = 1000000000000000; // 1000 ORM
    
    const mint = api.tx.oriumToken.mint(recipient, amount);
    const sudoMint = api.tx.sudo.sudo(mint);
    
    const hash = await sudoMint.signAndSend(sudo);
    console.log('Mint hash:', hash.toHex());
}
```

## Stablecoins (dUSD & dEUR)

### Collateral System Overview

The stablecoin system uses a MakerDAO-style approach:

1. **Collateral Deposit**: Users deposit ORM tokens
2. **Vault Creation**: System creates a Collateralized Debt Position (CDP)
3. **Stablecoin Minting**: Users mint dUSD/dEUR against collateral
4. **Liquidation**: Under-collateralized positions are liquidated

### Key Parameters

- **Minimum Collateral Ratio**: 150% (configurable)
- **Liquidation Ratio**: 130% (configurable)
- **Stability Fee**: 2% annually (configurable)
- **Liquidation Penalty**: 10% (configurable)

### Creating a Collateral Vault

#### Step 1: Deposit Collateral

```javascript
async function createVault() {
    const api = await ApiPromise.create({ provider: wsProvider });
    const keyring = new Keyring({ type: 'sr25519' });
    const user = keyring.addFromUri('//Alice');
    
    // Deposit 1000 ORM as collateral
    const collateralAmount = 1000000000000000; // 1000 ORM
    
    const createVault = api.tx.collateralEngine.createVault(
        'ORM',  // Collateral asset
        collateralAmount
    );
    
    const hash = await createVault.signAndSend(user);
    console.log('Vault creation hash:', hash.toHex());
}
```

#### Step 2: Mint Stablecoins

```javascript
async function mintStablecoin() {
    const api = await ApiPromise.create({ provider: wsProvider });
    const keyring = new Keyring({ type: 'sr25519' });
    const user = keyring.addFromUri('//Alice');
    
    const vaultId = 1; // Your vault ID
    const stablecoinAmount = 500000000000000; // 500 dUSD
    
    const mint = api.tx.collateralEngine.mintStablecoin(
        vaultId,
        'dUSD',
        stablecoinAmount
    );
    
    const hash = await mint.signAndSend(user);
    console.log('Mint hash:', hash.toHex());
}
```

### Managing Your Vault

#### Check Vault Status

```javascript
async function getVaultInfo(vaultId) {
    const api = await ApiPromise.create({ provider: wsProvider });
    
    const vault = await api.query.collateralEngine.vaults(vaultId);
    
    if (vault.isSome) {
        const vaultData = vault.unwrap();
        console.log('Vault Info:', {
            owner: vaultData.owner.toString(),
            collateralAmount: vaultData.collateralAmount.toString(),
            debtAmount: vaultData.debtAmount.toString(),
            collateralRatio: calculateCollateralRatio(vaultData)
        });
    }
}
```

#### Add More Collateral

```javascript
async function addCollateral(vaultId, amount) {
    const api = await ApiPromise.create({ provider: wsProvider });
    const keyring = new Keyring({ type: 'sr25519' });
    const user = keyring.addFromUri('//Alice');
    
    const addCollateral = api.tx.collateralEngine.addCollateral(vaultId, amount);
    const hash = await addCollateral.signAndSend(user);
    
    console.log('Add collateral hash:', hash.toHex());
}
```

#### Repay Debt

```javascript
async function repayDebt(vaultId, amount) {
    const api = await ApiPromise.create({ provider: wsProvider });
    const keyring = new Keyring({ type: 'sr25519' });
    const user = keyring.addFromUri('//Alice');
    
    const repay = api.tx.collateralEngine.repayDebt(vaultId, amount);
    const hash = await repay.signAndSend(user);
    
    console.log('Repay debt hash:', hash.toHex());
}
```

#### Withdraw Collateral

```javascript
async function withdrawCollateral(vaultId, amount) {
    const api = await ApiPromise.create({ provider: wsProvider });
    const keyring = new Keyring({ type: 'sr25519' });
    const user = keyring.addFromUri('//Alice');
    
    const withdraw = api.tx.collateralEngine.withdrawCollateral(vaultId, amount);
    const hash = await withdraw.signAndSend(user);
    
    console.log('Withdraw collateral hash:', hash.toHex());
}
```

### Stablecoin Operations

#### Transfer dUSD/dEUR

```javascript
async function transferStablecoin() {
    const api = await ApiPromise.create({ provider: wsProvider });
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');
    const bob = keyring.addFromUri('//Bob');
    
    // Transfer 100 dUSD
    const transfer = api.tx.dusd.transfer(bob.address, 100000000000000);
    const hash = await transfer.signAndSend(alice);
    
    console.log('dUSD transfer hash:', hash.toHex());
}
```

#### Check Stablecoin Balance

```javascript
async function checkStablecoinBalance(address) {
    const api = await ApiPromise.create({ provider: wsProvider });
    
    // Check dUSD balance
    const dusdBalance = await api.query.dusd.account(address);
    console.log('dUSD Balance:', dusdBalance.data.free.toString());
    
    // Check dEUR balance
    const deurBalance = await api.query.deur.account(address);
    console.log('dEUR Balance:', deurBalance.data.free.toString());
}
```

## Price Oracle Integration

### Get Current Prices

```javascript
async function getCurrentPrices() {
    const api = await ApiPromise.create({ provider: wsProvider });
    
    // Get ORM/USD price
    const ormUsdPrice = await api.query.collateralEngine.prices('ORM', 'USD');
    console.log('ORM/USD Price:', ormUsdPrice.toString());
    
    // Get ORM/EUR price
    const ormEurPrice = await api.query.collateralEngine.prices('ORM', 'EUR');
    console.log('ORM/EUR Price:', ormEurPrice.toString());
}
```

### Update Prices (Oracle Only)

```javascript
async function updatePrice() {
    const api = await ApiPromise.create({ provider: wsProvider });
    const keyring = new Keyring({ type: 'sr25519' });
    const oracle = keyring.addFromUri('//Oracle'); // Oracle account
    
    const newPrice = 1500000000; // $15.00 (with 8 decimals)
    
    const updatePrice = api.tx.collateralEngine.updatePrice('ORM', 'USD', newPrice);
    const hash = await updatePrice.signAndSend(oracle);
    
    console.log('Price update hash:', hash.toHex());
}
```

## Liquidation System

### Check Liquidation Status

```javascript
async function checkLiquidationRisk(vaultId) {
    const api = await ApiPromise.create({ provider: wsProvider });
    
    const vault = await api.query.collateralEngine.vaults(vaultId);
    const ormUsdPrice = await api.query.collateralEngine.prices('ORM', 'USD');
    
    if (vault.isSome && ormUsdPrice.isSome) {
        const vaultData = vault.unwrap();
        const price = ormUsdPrice.unwrap();
        
        const collateralValue = vaultData.collateralAmount * price;
        const collateralRatio = collateralValue / vaultData.debtAmount;
        
        console.log('Collateral Ratio:', collateralRatio);
        
        if (collateralRatio < 1.3) {
            console.log('⚠️ VAULT AT RISK OF LIQUIDATION!');
        } else if (collateralRatio < 1.5) {
            console.log('⚠️ Vault below minimum collateral ratio');
        } else {
            console.log('✅ Vault is healthy');
        }
    }
}
```

### Liquidate Vault (Liquidator)

```javascript
async function liquidateVault(vaultId) {
    const api = await ApiPromise.create({ provider: wsProvider });
    const keyring = new Keyring({ type: 'sr25519' });
    const liquidator = keyring.addFromUri('//Liquidator');
    
    const liquidate = api.tx.collateralEngine.liquidateVault(vaultId);
    const hash = await liquidate.signAndSend(liquidator);
    
    console.log('Liquidation hash:', hash.toHex());
}
```

## Advanced Features

### Batch Operations

```javascript
async function batchOperations() {
    const api = await ApiPromise.create({ provider: wsProvider });
    const keyring = new Keyring({ type: 'sr25519' });
    const user = keyring.addFromUri('//Alice');
    
    const calls = [
        api.tx.collateralEngine.addCollateral(1, 100000000000000),
        api.tx.collateralEngine.mintStablecoin(1, 'dUSD', 50000000000000),
        api.tx.dusd.transfer('5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY', 25000000000000)
    ];
    
    const batch = api.tx.utility.batch(calls);
    const hash = await batch.signAndSend(user);
    
    console.log('Batch hash:', hash.toHex());
}
```

### Event Monitoring

```javascript
async function monitorEvents() {
    const api = await ApiPromise.create({ provider: wsProvider });
    
    // Subscribe to all events
    api.query.system.events((events) => {
        events.forEach((record) => {
            const { event, phase } = record;
            
            if (event.section === 'collateralEngine') {
                console.log(`Event: ${event.section}.${event.method}`);
                console.log(`Data: ${event.data.toString()}`);
            }
        });
    });
}
```

## CLI Tools

### Using Substrate CLI

```bash
# Check ORM balance
./target/release/solochain-template-node key inspect //Alice

# Generate new account
./target/release/solochain-template-node key generate

# Insert key into keystore
./target/release/solochain-template-node key insert \
    --base-path /tmp/orium-data \
    --chain local \
    --scheme Sr25519 \
    --suri "//Alice" \
    --key-type babe
```

## Best Practices

### Security

1. **Never share private keys** in production
2. **Use hardware wallets** for large amounts
3. **Monitor collateral ratios** regularly
4. **Set up alerts** for liquidation risks

### Performance

1. **Batch transactions** when possible
2. **Use WebSocket** for real-time updates
3. **Cache frequently accessed data**
4. **Monitor gas fees** and optimize

### Risk Management

1. **Maintain high collateral ratios** (>200%)
2. **Diversify collateral** when possible
3. **Monitor price feeds** for accuracy
4. **Have emergency procedures** for market volatility

## Troubleshooting

### Common Errors

**Error: Insufficient balance**
```javascript
// Check balance before operations
const balance = await api.query.system.account(address);
console.log('Free balance:', balance.data.free.toString());
```

**Error: Vault not found**
```javascript
// Verify vault exists
const vault = await api.query.collateralEngine.vaults(vaultId);
if (vault.isNone) {
    console.log('Vault does not exist');
}
```

**Error: Collateral ratio too low**
```javascript
// Calculate required collateral
const requiredCollateral = debtAmount * 1.5; // 150% ratio
console.log('Required collateral:', requiredCollateral);
```

## Support

For additional help:

1. Check the [API Reference](API_REFERENCE.md)
2. Review [Architecture documentation](ARCHITECTURE.md)
3. Join our community discussions
4. Report issues on GitHub
