# ORIUM Token Usage Guide

This guide covers the usage of ORIUM (ORM) native token and the dUSD/dEUR stablecoins, including basic operations, advanced features, and integration examples.

## Table of Contents

- [ORIUM (ORM) Token](#orium-orm-token)
- [Stablecoins Overview](#stablecoins-overview)
- [Basic Operations](#basic-operations)
- [Advanced Features](#advanced-features)
- [Integration Examples](#integration-examples)
- [Security Considerations](#security-considerations)
- [Troubleshooting](#troubleshooting)

## ORIUM (ORM) Token

### Overview

ORIUM (ORM) is the native token of the ORIUM blockchain with the following characteristics:

- **Symbol**: ORM
- **Decimals**: 18
- **Address Prefix**: "or" (SS58 format: 111)
- **Total Supply**: Governed by on-chain governance
- **Use Cases**: 
  - Transaction fees
  - Staking for validators
  - Collateral for stablecoins
  - Governance participation

### Address Format

ORIUM uses the "or" prefix for addresses:
```
Example: orX1Y2Z3... (SS58 format with prefix 111)
```

### Token Economics

#### Initial Distribution
- **Validators**: 40% (for staking and network security)
- **Treasury**: 30% (for development and ecosystem growth)
- **Community**: 20% (for airdrops and incentives)
- **Team**: 10% (vested over 4 years)

#### Inflation Model
- **Target Inflation**: 7% annually
- **Validator Rewards**: 80% of inflation
- **Treasury**: 20% of inflation
- **Adjustment**: Based on staking participation rate

## Stablecoins Overview

### dUSD (USD-Pegged Stablecoin)

- **Symbol**: dUSD
- **Peg**: 1 dUSD = 1 USD
- **Collateral**: ORM tokens
- **Minimum Collateral Ratio**: 150%
- **Liquidation Threshold**: 130%
- **Stability Fee**: 2% annually

### dEUR (EUR-Pegged Stablecoin)

- **Symbol**: dEUR
- **Peg**: 1 dEUR = 1 EUR
- **Collateral**: ORM tokens
- **Minimum Collateral Ratio**: 150%
- **Liquidation Threshold**: 130%
- **Stability Fee**: 2% annually

### Collateralization Mechanism

The stablecoins use a MakerDAO-style CDP (Collateralized Debt Position) system:

1. **Deposit Collateral**: Lock ORM tokens as collateral
2. **Mint Stablecoins**: Generate dUSD/dEUR against collateral
3. **Maintain Ratio**: Keep collateral ratio above 150%
4. **Repay Debt**: Return stablecoins to unlock collateral

## Basic Operations

### Using Polkadot.js API

#### Setup
```javascript
import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';

// Connect to ORIUM node (development or devnet)
const wsProvider = new WsProvider('ws://localhost:9944');
const api = await ApiPromise.create({ provider: wsProvider });

// Create keyring with ORIUM's "or" address prefix
const keyring = new Keyring({ type: 'sr25519', ss58Format: 111 });
const account = keyring.addFromUri('//Alice');
```

### ORM Token Operations

#### Check Balance
```javascript
// Get ORM balance
const balance = await api.query.oriumToken.balances(account.address);
console.log(`ORM Balance: ${balance.toString()}`);

// Get free balance (transferable)
const accountInfo = await api.query.system.account(account.address);
console.log(`Free Balance: ${accountInfo.data.free.toString()}`);
```

#### Transfer ORM Tokens
```javascript
// Transfer 1000 ORM tokens
const transfer = api.tx.oriumToken.transfer(
  'orRecipientAddress...', 
  1000n * 10n**18n  // 1000 ORM (18 decimals)
);

const hash = await transfer.signAndSend(account);
console.log(`Transfer hash: ${hash.toString()}`);

// Via Polkadot-JS Apps UI:
// 1. Navigate to http://localhost:9944
// 2. Go to Extrinsics tab
// 3. Select oriumToken.transfer
// 4. Enter recipient address and amount
```

#### Approve Spending
```javascript
// Approve another account to spend tokens
const approve = api.tx.oriumToken.approve(
  'orSpenderAddress...', 
  500 * 10**18  // 500 ORM allowance
);

await approve.signAndSend(account);

// Check allowance
const allowance = await api.query.oriumToken.allowances(
  account.address, 
  'orSpenderAddress...'
);
console.log(`Allowance: ${allowance.toString()}`);
```

#### Transfer From (Delegated Transfer)
```javascript
// Transfer from approved account
const transferFrom = api.tx.oriumToken.transferFrom(
  'orOwnerAddress...', 
  'orRecipientAddress...', 
  100 * 10**18  // 100 ORM
);

await transferFrom.signAndSend(spenderAccount);
```

### Stablecoin Operations

#### Create CDP (Collateralized Debt Position)
```javascript
// Deposit 10,000 ORM as collateral
const createCdp = api.tx.collateralEngine.createCdp(
  10000 * 10**18  // 10,000 ORM collateral
);

const hash = await createCdp.signAndSend(account);
console.log(`CDP created: ${hash.toString()}`);
```

#### Check CDP Status
```javascript
// Get CDP information
const cdp = await api.query.collateralEngine.cdps(account.address);

if (cdp.isSome) {
  const cdpData = cdp.unwrap();
  console.log(`Collateral: ${cdpData.collateral.toString()}`);
  console.log(`dUSD Debt: ${cdpData.dusdDebt.toString()}`);
  console.log(`dEUR Debt: ${cdpData.deurDebt.toString()}`);
  
  // Calculate collateral ratio
  const ormPrice = await api.query.collateralEngine.ormUsdPrice();
  const collateralValue = cdpData.collateral * ormPrice / 10**18;
  const totalDebt = cdpData.dusdDebt + cdpData.deurDebt;
  const ratio = (collateralValue * 100) / totalDebt;
  console.log(`Collateral Ratio: ${ratio.toFixed(2)}%`);
}
```

#### Mint Stablecoins
```javascript
// Mint 5,000 dUSD (requires sufficient collateral)
const mintDusd = api.tx.collateralEngine.mintDusd(
  5000 * 10**18  // 5,000 dUSD
);

await mintDusd.signAndSend(account);

// Mint 3,000 dEUR
const mintDeur = api.tx.collateralEngine.mintDeur(
  3000 * 10**18  // 3,000 dEUR
);

await mintDeur.signAndSend(account);
```

#### Manage Collateral
```javascript
// Deposit additional collateral
const depositCollateral = api.tx.collateralEngine.depositCollateral(
  2000 * 10**18  // 2,000 ORM
);

await depositCollateral.signAndSend(account);

// Withdraw collateral (if ratio allows)
const withdrawCollateral = api.tx.collateralEngine.withdrawCollateral(
  1000 * 10**18  // 1,000 ORM
);

await withdrawCollateral.signAndSend(account);
```

#### Repay Debt
```javascript
// Repay dUSD debt
const repayDusd = api.tx.collateralEngine.repayDusd(
  1000 * 10**18  // 1,000 dUSD
);

await repayDusd.signAndSend(account);

// Repay dEUR debt
const repayDeur = api.tx.collateralEngine.repayDeur(
  500 * 10**18  // 500 dEUR
);

await repayDeur.signAndSend(account);
```

### Stablecoin Transfers

#### Transfer dUSD
```javascript
// Transfer dUSD tokens
const transferDusd = api.tx.dusd.transfer(
  'orRecipientAddress...', 
  1000 * 10**18  // 1,000 dUSD
);

await transferDusd.signAndSend(account);
```

#### Transfer dEUR
```javascript
// Transfer dEUR tokens
const transferDeur = api.tx.deur.transfer(
  'orRecipientAddress...', 
  500 * 10**18  // 500 dEUR
);

await transferDeur.signAndSend(account);
```

## Advanced Features

### Batch Operations

#### Multiple Transfers
```javascript
// Batch multiple transfers
const transfers = [
  api.tx.oriumToken.transfer('orAddress1...', 100 * 10**18),
  api.tx.dusd.transfer('orAddress2...', 50 * 10**18),
  api.tx.deur.transfer('orAddress3...', 25 * 10**18)
];

const batchTx = api.tx.utility.batch(transfers);
await batchTx.signAndSend(account);
```

#### CDP Management Batch
```javascript
// Deposit collateral and mint stablecoins in one transaction
const cdpOperations = [
  api.tx.collateralEngine.depositCollateral(5000 * 10**18),
  api.tx.collateralEngine.mintDusd(2000 * 10**18),
  api.tx.collateralEngine.mintDeur(1000 * 10**18)
];

const batchCdp = api.tx.utility.batch(cdpOperations);
await batchCdp.signAndSend(account);
```

### Event Monitoring

#### Listen for Token Events
```javascript
// Subscribe to ORM token events
api.query.system.events((events) => {
  events.forEach((record) => {
    const { event } = record;
    
    if (event.section === 'oriumToken') {
      console.log(`ORM Event: ${event.method}`);
      console.log(`Data: ${event.data.toString()}`);
    }
    
    if (event.section === 'collateralEngine') {
      console.log(`CDP Event: ${event.method}`);
      console.log(`Data: ${event.data.toString()}`);
    }
  });
});
```

#### Filter Specific Events
```javascript
// Listen for transfer events
api.query.system.events((events) => {
  events.forEach((record) => {
    const { event } = record;
    
    if (event.method === 'Transferred' && event.section === 'oriumToken') {
      const [from, to, amount] = event.data;
      console.log(`Transfer: ${from} -> ${to}, Amount: ${amount}`);
    }
    
    if (event.method === 'CdpCreated' && event.section === 'collateralEngine') {
      const [account, collateral] = event.data;
      console.log(`CDP Created: ${account}, Collateral: ${collateral}`);
    }
  });
});
```

### Price Oracle Integration

#### Get Current Prices
```javascript
// Get ORM/USD price
const ormUsdPrice = await api.query.collateralEngine.ormUsdPrice();
console.log(`ORM/USD: $${(ormUsdPrice / 10**18).toFixed(4)}`);

// Get ORM/EUR price
const ormEurPrice = await api.query.collateralEngine.ormEurPrice();
console.log(`ORM/EUR: €${(ormEurPrice / 10**18).toFixed(4)}`);
```

#### Monitor Price Updates
```javascript
// Subscribe to price updates
api.query.collateralEngine.ormUsdPrice((price) => {
  console.log(`New ORM/USD Price: $${(price / 10**18).toFixed(4)}`);
});

api.query.collateralEngine.ormEurPrice((price) => {
  console.log(`New ORM/EUR Price: €${(price / 10**18).toFixed(4)}`);
});
```

### Liquidation Monitoring

#### Check Liquidation Risk
```javascript
async function checkLiquidationRisk(accountAddress) {
  const cdp = await api.query.collateralEngine.cdps(accountAddress);
  
  if (cdp.isNone) {
    console.log('No CDP found');
    return;
  }
  
  const cdpData = cdp.unwrap();
  const ormUsdPrice = await api.query.collateralEngine.ormUsdPrice();
  const ormEurPrice = await api.query.collateralEngine.ormEurPrice();
  
  // Calculate collateral value in USD
  const collateralValueUsd = (cdpData.collateral * ormUsdPrice) / 10**18;
  
  // Calculate total debt in USD
  const dusdDebtUsd = cdpData.dusdDebt;
  const deurDebtUsd = (cdpData.deurDebt * ormEurPrice) / ormUsdPrice;
  const totalDebtUsd = dusdDebtUsd + deurDebtUsd;
  
  // Calculate collateral ratio
  const collateralRatio = (collateralValueUsd * 100) / totalDebtUsd;
  
  console.log(`Collateral Ratio: ${collateralRatio.toFixed(2)}%`);
  
  if (collateralRatio < 130) {
    console.log('⚠️ LIQUIDATION RISK: Ratio below 130%');
  } else if (collateralRatio < 150) {
    console.log('⚠️ WARNING: Ratio below safe threshold');
  } else {
    console.log('✅ CDP is safe');
  }
  
  return collateralRatio;
}

// Check liquidation risk
await checkLiquidationRisk('orAccountAddress...');
```

#### Perform Liquidation
```javascript
// Liquidate undercollateralized CDP
const liquidate = api.tx.collateralEngine.liquidate('orCdpOwnerAddress...');

try {
  const hash = await liquidate.signAndSend(liquidatorAccount);
  console.log(`Liquidation successful: ${hash.toString()}`);
} catch (error) {
  console.error('Liquidation failed:', error.message);
}
```

## Integration Examples

### Web Application Integration

#### React Hook for Token Balance
```javascript
import { useEffect, useState } from 'react';
import { ApiPromise, WsProvider } from '@polkadot/api';

function useTokenBalance(address) {
  const [balance, setBalance] = useState('0');
  const [loading, setLoading] = useState(true);
  
  useEffect(() => {
    let unsubscribe;
    
    async function subscribeToBalance() {
      const wsProvider = new WsProvider('ws://localhost:9944');
      const api = await ApiPromise.create({ provider: wsProvider });
      
      unsubscribe = await api.query.oriumToken.balances(
        address, 
        (balance) => {
          setBalance(balance.toString());
          setLoading(false);
        }
      );
    }
    
    subscribeToBalance();
    
    return () => {
      if (unsubscribe) {
        unsubscribe();
      }
    };
  }, [address]);
  
  return { balance, loading };
}

// Usage in component
function TokenBalance({ address }) {
  const { balance, loading } = useTokenBalance(address);
  
  if (loading) return <div>Loading...</div>;
  
  return (
    <div>
      Balance: {(parseInt(balance) / 10**18).toFixed(4)} ORM
    </div>
  );
}
```

#### CDP Management Component
```javascript
import React, { useState, useEffect } from 'react';

function CDPManager({ account, api }) {
  const [cdp, setCdp] = useState(null);
  const [collateralAmount, setCollateralAmount] = useState('');
  const [mintAmount, setMintAmount] = useState('');
  
  useEffect(() => {
    async function loadCdp() {
      const cdpData = await api.query.collateralEngine.cdps(account.address);
      if (cdpData.isSome) {
        setCdp(cdpData.unwrap());
      }
    }
    
    loadCdp();
  }, [account, api]);
  
  const createCdp = async () => {
    const tx = api.tx.collateralEngine.createCdp(
      parseFloat(collateralAmount) * 10**18
    );
    await tx.signAndSend(account);
  };
  
  const mintStablecoin = async () => {
    const tx = api.tx.collateralEngine.mintDusd(
      parseFloat(mintAmount) * 10**18
    );
    await tx.signAndSend(account);
  };
  
  return (
    <div>
      <h3>CDP Management</h3>
      
      {!cdp ? (
        <div>
          <input
            type="number"
            placeholder="Collateral Amount (ORM)"
            value={collateralAmount}
            onChange={(e) => setCollateralAmount(e.target.value)}
          />
          <button onClick={createCdp}>Create CDP</button>
        </div>
      ) : (
        <div>
          <p>Collateral: {(cdp.collateral / 10**18).toFixed(4)} ORM</p>
          <p>dUSD Debt: {(cdp.dusdDebt / 10**18).toFixed(4)} dUSD</p>
          <p>dEUR Debt: {(cdp.deurDebt / 10**18).toFixed(4)} dEUR</p>
          
          <input
            type="number"
            placeholder="Mint Amount (dUSD)"
            value={mintAmount}
            onChange={(e) => setMintAmount(e.target.value)}
          />
          <button onClick={mintStablecoin}>Mint dUSD</button>
        </div>
      )}
    </div>
  );
}
```

### Backend Service Integration

#### Node.js Service
```javascript
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { Keyring } = require('@polkadot/keyring');

class OriumService {
  constructor() {
    this.api = null;
    this.keyring = new Keyring({ type: 'sr25519' });
  }
  
  async connect() {
    const wsProvider = new WsProvider('ws://localhost:9944');
    this.api = await ApiPromise.create({ provider: wsProvider });
    console.log('Connected to ORIUM blockchain');
  }
  
  async getTokenBalance(address) {
    const balance = await this.api.query.oriumToken.balances(address);
    return balance.toString();
  }
  
  async transferTokens(fromSeed, toAddress, amount) {
    const fromAccount = this.keyring.addFromUri(fromSeed);
    const transfer = this.api.tx.oriumToken.transfer(toAddress, amount);
    
    return new Promise((resolve, reject) => {
      transfer.signAndSend(fromAccount, ({ status, events }) => {
        if (status.isInBlock) {
          console.log(`Transfer included in block ${status.asInBlock}`);
          resolve(status.asInBlock.toString());
        } else if (status.isFinalized) {
          console.log(`Transfer finalized in block ${status.asFinalized}`);
        }
        
        events.forEach(({ event }) => {
          if (this.api.events.system.ExtrinsicFailed.is(event)) {
            reject(new Error('Transfer failed'));
          }
        });
      });
    });
  }
  
  async monitorCdpHealth(address, callback) {
    return this.api.query.collateralEngine.cdps(address, (cdp) => {
      if (cdp.isSome) {
        const cdpData = cdp.unwrap();
        // Calculate health and call callback
        callback(cdpData);
      }
    });
  }
}

// Usage
const oriumService = new OriumService();
await oriumService.connect();

// Get balance
const balance = await oriumService.getTokenBalance('orAddress...');
console.log(`Balance: ${balance}`);

// Transfer tokens
await oriumService.transferTokens(
  '//Alice', 
  'orRecipientAddress...', 
  1000 * 10**18
);
```

### Python Integration

#### Using py-substrate-interface
```python
from substrateinterface import SubstrateInterface, Keypair
from substrateinterface.exceptions import SubstrateRequestException

class OriumClient:
    def __init__(self, url="ws://127.0.0.1:9944"):
        self.substrate = SubstrateInterface(url=url)
        
    def get_token_balance(self, address):
        """Get ORM token balance for an address"""
        result = self.substrate.query(
            module='OriumToken',
            storage_function='Balances',
            params=[address]
        )
        return result.value
        
    def transfer_tokens(self, keypair, recipient, amount):
        """Transfer ORM tokens"""
        call = self.substrate.compose_call(
            call_module='OriumToken',
            call_function='transfer',
            call_params={
                'dest': recipient,
                'value': amount
            }
        )
        
        extrinsic = self.substrate.create_signed_extrinsic(
            call=call,
            keypair=keypair
        )
        
        try:
            receipt = self.substrate.submit_extrinsic(
                extrinsic, 
                wait_for_inclusion=True
            )
            return receipt.extrinsic_hash
        except SubstrateRequestException as e:
            print(f"Transfer failed: {e}")
            return None
            
    def get_cdp_info(self, address):
        """Get CDP information for an address"""
        result = self.substrate.query(
            module='CollateralEngine',
            storage_function='Cdps',
            params=[address]
        )
        return result.value

# Usage
client = OriumClient()

# Create keypair from seed
keypair = Keypair.create_from_uri('//Alice')

# Get balance
balance = client.get_token_balance(keypair.ss58_address)
print(f"Balance: {balance}")

# Transfer tokens
hash = client.transfer_tokens(
    keypair, 
    'orRecipientAddress...', 
    1000 * 10**18
)
print(f"Transfer hash: {hash}")
```

## Security Considerations

### Private Key Management

#### Best Practices
1. **Never hardcode private keys** in source code
2. **Use environment variables** for sensitive data
3. **Implement key rotation** for production systems
4. **Use hardware wallets** for large amounts
5. **Encrypt stored keys** with strong passwords

#### Secure Key Storage
```javascript
// Use environment variables
const PRIVATE_KEY = process.env.ORIUM_PRIVATE_KEY;

// Or use encrypted keystore
const keystore = JSON.parse(fs.readFileSync('keystore.json'));
const account = keyring.addFromJson(keystore);
account.decodePkcs8(password);
```

### Transaction Security

#### Verify Transaction Parameters
```javascript
// Always verify transaction details before signing
const transfer = api.tx.oriumToken.transfer(recipient, amount);

console.log(`Transferring ${amount} to ${recipient}`);
console.log(`Transaction fee: ${transfer.paymentInfo(sender).partialFee}`);

// Confirm before signing
const confirmed = await confirmTransaction();
if (confirmed) {
  await transfer.signAndSend(sender);
}
```

#### Use Batch Transactions Carefully
```javascript
// Verify all calls in batch
const calls = [
  api.tx.oriumToken.transfer(addr1, amount1),
  api.tx.dusd.transfer(addr2, amount2)
];

// Check each call
calls.forEach((call, index) => {
  console.log(`Call ${index}: ${call.method.section}.${call.method.method}`);
  console.log(`Args: ${call.method.args.toString()}`);
});

const batch = api.tx.utility.batch(calls);
await batch.signAndSend(sender);
```

### CDP Security

#### Monitor Collateral Ratios
```javascript
// Set up automated monitoring
setInterval(async () => {
  const ratio = await checkCollateralRatio(account.address);
  
  if (ratio < 140) {
    // Send alert
    await sendAlert(`Low collateral ratio: ${ratio}%`);
    
    // Auto-deposit more collateral if configured
    if (autoManage) {
      await depositMoreCollateral();
    }
  }
}, 60000); // Check every minute
```

#### Liquidation Protection
```javascript
// Implement liquidation protection
async function protectFromLiquidation(account) {
  const cdp = await api.query.collateralEngine.cdps(account.address);
  const ratio = await calculateCollateralRatio(cdp);
  
  if (ratio < 135) {
    // Emergency: Repay some debt
    const repayAmount = calculateRepayAmount(cdp, 160); // Target 160% ratio
    await api.tx.collateralEngine.repayDusd(repayAmount).signAndSend(account);
  }
}
```

## Troubleshooting

### Common Issues

#### 1. Transaction Failures
```javascript
// Check account balance before transaction
const balance = await api.query.oriumToken.balances(account.address);
if (balance < amount) {
  throw new Error('Insufficient balance');
}

// Check for proper error handling
try {
  await transfer.signAndSend(account);
} catch (error) {
  if (error.message.includes('InsufficientBalance')) {
    console.log('Not enough tokens');
  } else if (error.message.includes('InvalidSignature')) {
    console.log('Invalid signature');
  }
}
```

#### 2. CDP Creation Failures
```javascript
// Verify minimum collateral requirements
const minCollateral = 1000 * 10**18; // 1000 ORM minimum
if (collateralAmount < minCollateral) {
  throw new Error('Insufficient collateral amount');
}

// Check if CDP already exists
const existingCdp = await api.query.collateralEngine.cdps(account.address);
if (existingCdp.isSome) {
  throw new Error('CDP already exists for this account');
}
```

#### 3. Price Oracle Issues
```javascript
// Check if price is stale
const lastUpdate = await api.query.collateralEngine.lastPriceUpdate();
const now = Date.now();
const maxAge = 3600000; // 1 hour

if (now - lastUpdate > maxAge) {
  console.warn('Price data may be stale');
}

// Fallback to alternative price source
if (!ormPrice || ormPrice === 0) {
  ormPrice = await getBackupPrice();
}
```

### Performance Optimization

#### Batch Multiple Operations
```javascript
// Instead of multiple individual transactions
await api.tx.oriumToken.transfer(addr1, amount1).signAndSend(account);
await api.tx.oriumToken.transfer(addr2, amount2).signAndSend(account);
await api.tx.oriumToken.transfer(addr3, amount3).signAndSend(account);

// Use batch transaction
const transfers = [
  api.tx.oriumToken.transfer(addr1, amount1),
  api.tx.oriumToken.transfer(addr2, amount2),
  api.tx.oriumToken.transfer(addr3, amount3)
];

await api.tx.utility.batch(transfers).signAndSend(account);
```

#### Use Subscriptions for Real-time Data
```javascript
// Instead of polling
setInterval(async () => {
  const balance = await api.query.oriumToken.balances(address);
  updateUI(balance);
}, 5000);

// Use subscription
const unsubscribe = await api.query.oriumToken.balances(
  address, 
  (balance) => {
    updateUI(balance);
  }
);
```

### Getting Help

1. **Documentation**: [API Reference](API_REFERENCE.md)
2. **Community**: [Discord](https://discord.gg/orium)
3. **GitHub Issues**: [Report bugs](https://github.com/your-org/orium-blockchain/issues)
4. **Examples**: [GitHub Examples](https://github.com/your-org/orium-examples)

## Next Steps

- [API Reference](API_REFERENCE.md) - Complete API documentation
- [Architecture Guide](ARCHITECTURE.md) - Technical architecture details
- [Development Guide](DEVELOPMENT.md) - Building on ORIUM
- [Validator Guide](VALIDATOR_GUIDE.md) - Running a validator node
