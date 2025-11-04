# Hypernode Core Protocol

**Modular Solana programs for decentralized compute marketplace**

![Solana](https://img.shields.io/badge/Solana-Program-9945FF?logo=solana)
![Anchor](https://img.shields.io/badge/Anchor-0.29.0-512DA8)
![License](https://img.shields.io/badge/License-MIT-green)

## 🎯 Overview

Hypernode Core Protocol is a suite of modular Solana programs that power a trustless, decentralized compute marketplace. Built with Anchor framework and inspired by Nosana's architecture, it enables:

- **Trustless job-to-node matching** via on-chain dynamic queue
- **Time-locked staking** with xNOS rewards and multipliers
- **O(1) reward distribution** using token reflection
- **IPFS content-addressing** for verifiable job definitions and results

---

## 🏗️ Architecture

### 4 Modular Programs (100% Complete)

```
hypernode-core-protocol/
├── hypernode-nodes      ✅  Node registry with hardware specs
├── hypernode-jobs       ✅  Job marketplace with dynamic queue
├── hypernode-staking    ✅  xNOS calculation with time multipliers
└── hypernode-rewards    ✅  Token reflection distribution (O(1))
```

### Program Interaction Flow

```
┌─────────────────────┐
│   hypernode-nodes   │  Register → Track hardware & reputation
└──────────┬──────────┘
           │
           ├──────────────────────────────────┐
           │                                  │
┌──────────▼──────────┐          ┌───────────▼──────────┐
│  hypernode-jobs     │  ◄─────  │  hypernode-staking   │
│  • Dynamic queue    │  CPI     │  • xNOS calculation  │
│  • Escrow payments  │  ─────►  │  • Time multipliers  │
│  • IPFS integration │          │  • Tier system       │
└──────────┬──────────┘          └───────────┬──────────┘
           │                                  │
           │  Job fees                   xNOS rewards
           │                                  │
           └──────────────┬───────────────────┘
                          │
                ┌─────────▼──────────┐
                │ hypernode-rewards  │
                │ • O(1) distribution│
                │ • Proportional     │
                └────────────────────┘
```

---

## 📦 Programs

### 1. hypernode-nodes

**Purpose:** Node registry and reputation tracking

```rust
Instructions:
- register(hardware_specs)  // Register new compute node
- update(hardware_specs)    // Update node specs
- heartbeat()              // Keep-alive signal
```

**Features:**
- 11 architecture types (Amd64, Arm64, Riscv64, etc.)
- Country-based tracking (ISO codes)
- Reputation scoring (0-1000)
- Uptime percentage tracking
- Audit system for trusted nodes

**Program ID:** `HYPRnodes11111111111111111111111111111111111`

---

### 2. hypernode-jobs

**Purpose:** Trustless job marketplace with dynamic queue

```rust
Instructions:
- initialize_market(params)     // Create new market
- submit_job(ipfs, price)       // Client submits job
- work()                        // Node claims job
- finish(ipfs_result, success) // Complete job
- recover()                    // Refund expired jobs
```

**Features:**
- **Dynamic Queue System** (Nosana pattern):
  - `QueueType::Empty` → Balanced supply/demand
  - `QueueType::Node` → Nodes waiting for jobs
  - `QueueType::Job` → Jobs waiting for nodes
- SPL token escrow with automatic settlement
- IPFS content-addressed storage (32-byte hashes)
- Permissionless recovery (anti-DoS)
- Event emission for off-chain tracking

**Program ID:** `HYPRjobs11111111111111111111111111111111111`

---

### 3. hypernode-staking

**Purpose:** Time-locked staking with xNOS rewards

```rust
Instructions:
- initialize_config()           // Setup staking params
- stake(amount, duration)       // Lock HYPER → Earn xNOS
- unstake()                     // Withdraw after unlock
```

**xNOS Formula:**
```
xNOS = staked_amount × multiplier

Multipliers (based on lock duration):
• < 1 month    → 1x   (100 bps)
• 1-3 months   → 1.5x (150 bps)
• 3-6 months   → 2x   (200 bps)
• 6-12 months  → 3x   (300 bps)
• >= 1 year    → 4x   (400 bps)
```

**Tier System:**
| Tier | xNOS Required | Benefits |
|------|---------------|----------|
| Starter | 0-999 | Basic access |
| Bronze | 1,000-9,999 | Priority queue |
| Silver | 10,000-49,999 | Reduced fees |
| Gold | 50,000-99,999 | Premium markets |
| Diamond | 100,000+ | All benefits |

**Program ID:** `HYPRstake1111111111111111111111111111111111`

---

### 4. hypernode-rewards

**Purpose:** O(1) reward distribution via token reflection

```rust
Instructions:
- initialize_pool(rate_bps)  // Setup reward pool
- claim_rewards()           // Claim proportional share
```

**Distribution Formula (O(1)):**
```
user_reward = (user_xnos / total_xnos) × accumulated_rewards

No iteration needed! Instant calculation for all users.
```

**Program ID:** `HYPRreward111111111111111111111111111111111`

---

## 🚀 Quick Start

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Install Anchor
cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked
```

### Build

```bash
# Clone repository
git clone https://github.com/Hypernode-sol/hypernode-core-protocol.git
cd hypernode-core-protocol

# Build all programs
anchor build

# Run tests
anchor test
```

### Deploy

```bash
# Deploy to devnet
anchor deploy --provider.cluster devnet

# Deploy to mainnet (after audits)
anchor deploy --provider.cluster mainnet
```

---

## 📚 Documentation

- [**MODULAR_ARCHITECTURE.md**](./MODULAR_ARCHITECTURE.md) - Complete architecture design
- [**IMPLEMENTATION_STATUS.md**](./IMPLEMENTATION_STATUS.md) - Current implementation status
- [**API Documentation**](./docs/api.md) - Instruction reference (coming soon)
- [**Integration Guide**](./docs/integration.md) - How to integrate (coming soon)

---

## 🔧 Development

### Project Structure

```
hypernode-core-protocol/
├── programs/
│   ├── hypernode-nodes/
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── state/
│   │       └── instructions/
│   ├── hypernode-jobs/
│   ├── hypernode-staking/
│   └── hypernode-rewards/
├── tests/
├── Anchor.toml
└── README.md
```

### Code Principles

This project follows principles from industry leaders:

- **🔐 Trustless**: On-chain queue, cryptographic verification
- **🧩 Modular**: Independent programs, composable design
- **🛡️ Safe**: Circuit breakers, extensive validations
- **✨ Clear**: One instruction per file, simple formulas

---

## 🧪 Testing

```bash
# Run all tests
anchor test

# Test specific program
anchor test --skip-build -- --test hypernode-jobs

# Local validator
solana-test-validator
```

---

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](./CONTRIBUTING.md) for details.

### Development Workflow

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open Pull Request

---

## 🔒 Security

- **Audits:** Pending (planned Q2 2025)
- **Bug Bounty:** Coming soon
- **Responsible Disclosure:** contact@hypernodesolana.org

---

## 📊 Comparison

| Feature | Hypernode | Nosana | Akash |
|---------|-----------|---------|-------|
| **Blockchain** | Solana | Solana | Cosmos |
| **Queue System** | On-chain ✅ | On-chain ✅ | Off-chain |
| **Staking** | Time multipliers | xNOS | AKT tokens |
| **Distribution** | O(1) reflection | O(1) reflection | Block-based |
| **Content Storage** | IPFS ✅ | IPFS ✅ | Custom |
| **Modular Design** | 4 programs ✅ | 4 programs ✅ | 6 modules |

---

## 📈 Roadmap

- [x] **Q4 2024** - Architecture design
- [x] **Q1 2025** - Core programs implementation (4/4 complete)
- [ ] **Q2 2025** - Security audits
- [ ] **Q2 2025** - Mainnet deployment
- [ ] **Q3 2025** - SDK and tooling
- [ ] **Q4 2025** - Governance integration

---

## 📞 Links

- **Website:** [hypernodesolana.org](https://hypernodesolana.org)
- **Twitter:** [@HypernodeSol](https://twitter.com/HypernodeSol)
- **GitHub:** [github.com/Hypernode-sol](https://github.com/Hypernode-sol)

---

## ⚖️ License

MIT License - see [LICENSE](./LICENSE) file for details

---

## 🙏 Acknowledgments

- Inspired by [Nosana](https://nosana.io) architecture
- Built with [Anchor Framework](https://www.anchor-lang.com/)
- Powered by [Solana](https://solana.com/)

---

**Built with ❤️ by the Hypernode Team**
