# Hypernode Modular Architecture Design

## Overview

Refactoring from monolithic `hypernode-protocol` to 4 independent Solana programs following Nosana/Akash architecture patterns.

---

## 🏗️ Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                         CLIENT                              │
│              (Web App / Node Client / CLI)                  │
└────────────┬────────────────────────────────┬───────────────┘
             │                                │
             ▼                                ▼
┌────────────────────────┐       ┌────────────────────────┐
│   hypernode-jobs       │       │   hypernode-nodes      │
│                        │       │                        │
│ - submit_job()         │◄─────►│ - register()           │
│ - list_job()           │       │ - update()             │
│ - work()               │       │ - heartbeat()          │
│ - claim()              │       │ - deactivate()         │
│ - finish()             │       │                        │
│ - recover()            │       │ Registry only          │
│                        │       │                        │
│ CPI ↓                  │       │                        │
└────────┬───────────────┘       └────────────────────────┘
         │                                    ▲
         │                                    │
         ▼                                    │ CPI
┌────────────────────────┐       ┌────────────────────────┐
│   hypernode-staking    │       │   hypernode-rewards    │
│                        │       │                        │
│ - stake()              │◄─────►│ - enter()              │
│ - unstake()            │       │ - add_fee()            │
│ - extend()             │       │ - claim()              │
│ - topup()              │       │                        │
│ - slash()              │       │ Token Reflection       │
│                        │       │                        │
│ xNOS Calculation       │       │                        │
└────────────────────────┘       └────────────────────────┘
```

---

## 📦 Program Breakdown

### 1. **hypernode-jobs** (Core Job Lifecycle)

**Program ID:** `HYPRjobs11111111111111111111111111111111111`

**Responsibilities:**
- Job creation and management
- Dynamic queue system (Job queue vs Node queue)
- Job-to-node matching
- Escrow management
- Payment settlement

**Key Instructions:**
```rust
// Job Creation
pub fn submit_job(ipfs_hash: [u8; 32], max_price: u64, timeout: i64)
pub fn list_job(ipfs_hash: [u8; 32], timeout: i64)
pub fn assign_job(node: Pubkey)

// Node Operations
pub fn work() // Enter queue or claim job
pub fn claim(job: Pubkey)
pub fn stop() // Exit queue

// Settlement
pub fn finish(result_ipfs: [u8; 32])
pub fn quit(reason: String)
pub fn recover() // Refund expired jobs

// Management
pub fn extend_timeout(additional_time: i64)
pub fn delist()
```

**Accounts:**
```rust
pub struct Market {
    pub authority: Pubkey,
    pub queue_type: QueueType,          // Node, Job, Empty
    pub queue: Vec<Pubkey>,             // Dynamic queue (max 314)
    pub job_price: u64,                 // Base price
    pub job_timeout: i64,               // Default timeout
    pub node_xnos_minimum: u128,        // Minimum xNOS to participate
    pub vault: Pubkey,                  // Payment vault
}

pub struct Job {
    pub market: Pubkey,
    pub client: Pubkey,
    pub ipfs_job: [u8; 32],             // Job definition hash
    pub ipfs_result: [u8; 32],          // Result hash
    pub price: u64,
    pub state: JobState,                // Queued, Done, Stopped
    pub timeout: i64,
    pub node: Option<Pubkey>,
    pub created_at: i64,
    pub started_at: Option<i64>,
    pub completed_at: Option<i64>,
}

pub struct Run {
    pub job: Pubkey,
    pub node: Pubkey,
    pub started_at: i64,
    pub duration: Option<i64>,
}

pub enum QueueType {
    Empty,  // Supply == Demand
    Node,   // Supply > Demand (nodes waiting)
    Job,    // Demand > Supply (jobs waiting)
}

pub enum JobState {
    Queued,
    Done,
    Stopped,
}
```

**CPIs Out:**
- `hypernode-staking::verify_xnos()` - Validate node stake
- `hypernode-rewards::add_fee()` - Distribute network fees

---

### 2. **hypernode-nodes** (Node Registry)

**Program ID:** `HYPRnodes11111111111111111111111111111111111`

**Responsibilities:**
- Node registration and metadata
- Hardware specs tracking
- Reputation management
- Audit system integration

**Key Instructions:**
```rust
pub fn register(
    node_id: String,
    architecture: Architecture,
    country: Country,
    cpu_cores: u16,
    gpu_cores: u16,
    ram_gb: u16,
    iops: u32,
    storage_gb: u32,
    endpoint: String,
)

pub fn update(/* same params */)
pub fn audit(is_audited: bool) // Admin only
pub fn heartbeat()
pub fn deactivate()
```

**Accounts:**
```rust
pub struct Node {
    pub authority: Pubkey,
    pub node_id: String,
    pub is_audited: bool,

    // Hardware specs
    pub architecture: Architecture,  // Amd64, Arm64, etc.
    pub country: Country,            // ISO codes
    pub cpu_cores: u16,
    pub gpu_cores: u16,
    pub ram_gb: u16,
    pub iops: u32,
    pub storage_gb: u32,

    // Stats
    pub jobs_completed: u64,
    pub jobs_failed: u64,
    pub total_earned: u64,
    pub reputation_score: u16,       // 0-1000
    pub uptime_percentage: u8,       // Last 30 days

    // Network
    pub endpoint: String,            // HTTP endpoint for logs
    pub version: u32,
    pub registered_at: i64,
    pub last_heartbeat: i64,
    pub is_active: bool,
}

pub enum Architecture {
    Amd64, Arm64, ArmV7, Arm, I386, Mips64,
    Mips64le, Ppc64, Ppc64le, S390x, Riscv64,
}

pub enum Country {
    US, BR, DE, JP, CN, Unknown, // etc.
}
```

**No CPIs** - Pure registry

---

### 3. **hypernode-staking** (xNOS Stake Management)

**Program ID:** `HYPRstake11111111111111111111111111111111111`

**Responsibilities:**
- Token staking with time multipliers
- xNOS calculation and tracking
- Unstake cooldown management
- Slashing mechanism

**Key Instructions:**
```rust
pub fn stake(amount: u64, duration_days: u16) // 14-365 days
pub fn unstake()
pub fn restake()
pub fn topup(amount: u64)
pub fn extend(additional_days: u16)
pub fn withdraw()
pub fn slash(amount: u64, reason: String) // Admin only
```

**xNOS Calculation:**
```rust
// Nosana formula
xNOS = (stake_duration_days / 365) * token_amount * 4

// Examples:
// 100 HYPER × 14 days = 100 * (14/365) * 4 = 15.34 xNOS
// 100 HYPER × 365 days = 100 * (365/365) * 4 = 400 xNOS (4x multiplier)
```

**Accounts:**
```rust
pub struct Stake {
    pub authority: Pubkey,
    pub amount: u64,                 // Tokens staked (subject to slashing)
    pub xnos: u128,                  // Multiplied value
    pub duration: i64,               // Lock duration
    pub staked_at: i64,
    pub unstake_at: Option<i64>,     // Cooldown timestamp
    pub last_claim_rate: u128,       // For rewards calculation
}

pub struct SlashRecord {
    pub stake: Pubkey,
    pub amount: u64,
    pub reason: String,
    pub authority: Pubkey,
    pub timestamp: i64,
}
```

**Validations:**
- Duration: 14-365 days
- Amount > 0
- Can't unstake with running jobs
- Slashing requires admin authority

---

### 4. **hypernode-rewards** (Fee Distribution)

**Program ID:** `HYPRreward11111111111111111111111111111111111`

**Responsibilities:**
- Network fee collection
- Token reflection distribution
- Automatic reward calculation
- Claim functionality

**Key Instructions:**
```rust
pub fn enter() // Initialize rewards account
pub fn add_fee(amount: u64) // Called by jobs program (CPI)
pub fn claim() // Claim accumulated rewards
```

**Token Reflection Algorithm:**
```rust
// O(1) distribution (no loops!)
pub struct Reflection {
    pub total_xnos: u128,
    pub rate_per_xnos: u128,  // Accumulated: sum(fees) / total_xnos
}

// When fee added (from job completion):
reflection.rate_per_xnos += (fee_amount * PRECISION) / reflection.total_xnos;

// When user claims:
let rate_delta = reflection.rate_per_xnos - stake.last_claim_rate;
let rewards = (stake.xnos * rate_delta) / PRECISION;
stake.last_claim_rate = reflection.rate_per_xnos;
```

**Accounts:**
```rust
pub struct Reflection {
    pub total_xnos: u128,
    pub rate_per_xnos: u128,
    pub vault: Pubkey,
}

pub struct RewardsAccount {
    pub stake: Pubkey,
    pub last_claim_rate: u128,
}
```

**CPIs Out:**
- None (terminal program)

**CPIs In:**
- `hypernode-jobs::finish()` → `add_fee()`

---

## 🔄 Complete Workflows

### Workflow 1: Node Joins Network

```
1. hypernode-nodes::register()
   └─ Create NodeAccount with hardware specs

2. hypernode-staking::stake()
   ├─ Lock tokens for duration
   ├─ Calculate xNOS = (days/365) * amount * 4
   └─ Store StakeAccount

3. hypernode-rewards::enter()
   └─ Initialize RewardsAccount

4. hypernode-jobs::work()
   ├─ CPI: staking.verify_xnos() ✓
   ├─ Check market.node_xnos_minimum
   ├─ If Job Queue: claim job immediately
   └─ Else: enter Node Queue
```

### Workflow 2: Client Submits Job

```
1. hypernode-jobs::submit_job(ipfs_hash, timeout)
   ├─ Create JobAccount
   ├─ Deposit payment + fee to vault
   ├─ Check Queue Type:
   │   ├─ If Node Queue: assign to first node
   │   └─ Else: enter Job Queue
   └─ CPI: rewards.add_fee(network_fee)

2. Node executes in Docker container
   └─ Fetch job definition from IPFS

3. hypernode-jobs::finish(result_ipfs)
   ├─ Store result hash
   ├─ Calculate settlement amount
   ├─ Transfer to node
   ├─ Refund surplus to client
   ├─ CPI: nodes.update_stats()
   └─ CPI: rewards.add_fee(platform_fee)
```

### Workflow 3: Node Claims Rewards

```
1. hypernode-rewards::claim()
   ├─ Fetch stake.xnos
   ├─ Calculate rewards:
   │   rate_delta = global_rate - last_claim_rate
   │   rewards = (xnos * rate_delta) / PRECISION
   ├─ Transfer rewards to node
   └─ Update last_claim_rate
```

---

## 🔐 Security Improvements

### From Nosana/Akash Analysis:

1. **Time-Locked Bidding** (prevent sniping)
   - Jobs have `bid_end_slot`
   - No extensions on late bids

2. **Anti-Colusão Randomization**
   - Multiple nodes at same price → random selection
   - Uses recent blockhash as seed

3. **xNOS Minimum Requirements**
   - Markets can require minimum xNOS
   - Prevents spam/sybil attacks

4. **Slashing with Appeals** (future)
   - Slash for misbehavior
   - Appeal mechanism for disputes

5. **IPFS Content-Addressed Storage**
   - Jobs and results verifiable by hash
   - Immutable and auditable

---

## 📊 Benefits of Modular Design

### Gavin Wood Principles ✅

1. **Independent Upgrades**
   - Fix jobs program without touching staking
   - Add features to rewards without redeploying nodes

2. **Reduced Compute Units**
   - Each instruction smaller
   - More efficient transactions

3. **Composability**
   - Other projects can use hypernode-staking
   - Rewards program integrable with other systems

4. **Clear Boundaries**
   - Jobs = lifecycle
   - Nodes = registry
   - Staking = economics
   - Rewards = distribution

### Cost Savings

**Before (Monolithic):**
- Single 527-line program
- Large account sizes
- High CU usage per instruction

**After (Modular):**
- 4 programs × ~200 lines each
- Smaller account sizes
- Lower CU per transaction
- Parallel development possible

---

## 🚀 Migration Plan

### Phase 1: Create New Programs
1. Implement 4 new programs
2. Deploy to devnet
3. Test independently

### Phase 2: Integration
1. Add CPI calls between programs
2. Test workflows end-to-end
3. Audit security

### Phase 3: Deploy & Migrate
1. Deploy to mainnet
2. Pause old program
3. Migrate data (if needed)
4. Update frontend/backend
5. Deprecate old program

---

## 📁 Directory Structure

```
programs/
├── hypernode-jobs/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── instructions/
│   │   │   ├── mod.rs
│   │   │   ├── submit_job.rs
│   │   │   ├── work.rs
│   │   │   ├── claim.rs
│   │   │   ├── finish.rs
│   │   │   └── recover.rs
│   │   ├── state/
│   │   │   ├── mod.rs
│   │   │   ├── market.rs
│   │   │   ├── job.rs
│   │   │   └── run.rs
│   │   └── error.rs
│   ├── Cargo.toml
│   └── Xargo.toml
│
├── hypernode-nodes/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── instructions/
│   │   │   ├── mod.rs
│   │   │   ├── register.rs
│   │   │   ├── update.rs
│   │   │   └── heartbeat.rs
│   │   ├── state/
│   │   │   ├── mod.rs
│   │   │   └── node.rs
│   │   └── error.rs
│   ├── Cargo.toml
│   └── Xargo.toml
│
├── hypernode-staking/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── instructions/
│   │   │   ├── mod.rs
│   │   │   ├── stake.rs
│   │   │   ├── unstake.rs
│   │   │   └── claim.rs
│   │   ├── state/
│   │   │   ├── mod.rs
│   │   │   └── stake.rs
│   │   └── error.rs
│   ├── Cargo.toml
│   └── Xargo.toml
│
└── hypernode-rewards/
    ├── src/
    │   ├── lib.rs
    │   ├── instructions/
    │   │   ├── mod.rs
    │   │   ├── enter.rs
    │   │   ├── add_fee.rs
    │   │   └── claim.rs
    │   ├── state/
    │   │   ├── mod.rs
    │   │   └── reflection.rs
    │   └── error.rs
    ├── Cargo.toml
    └── Xargo.toml
```

---

## ✅ Next Steps

1. ✅ Architecture designed
2. ⏳ Implement `hypernode-nodes` (simplest, no dependencies)
3. ⏳ Implement `hypernode-staking` (xNOS calculation)
4. ⏳ Implement `hypernode-rewards` (token reflection)
5. ⏳ Implement `hypernode-jobs` (most complex, uses CPIs)
6. ⏳ Integration testing
7. ⏳ Deploy to devnet
8. ⏳ Update frontend/backend
9. ⏳ Mainnet deployment

---

**This architecture follows:**
- ✅ Nick Szabo (trustless, on-chain verification)
- ✅ Gavin Wood (modular, composable)
- ✅ Dario Amodei (safety-first, circuit breakers)
- ✅ Andrej Karpathy (clear, maintainable)

**Learned from:**
- Nosana (4 programs, queue system, xNOS, token reflection)
- Akash (Cosmos SDK modules, escrow settlement, provider bidding)
