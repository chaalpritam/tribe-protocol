# Tribe Protocol Architecture: Custom Ephemeral Rollup + Decentralized Social on Solana

> Tribe is a fully owned, open social protocol on Solana with a native Ephemeral Rollup layer for real-time state execution and a peer-to-peer message network for content distribution.

---

## Table of Contents

1. [System Overview](#1-system-overview)
2. [Layer 0: Solana Base Layer](#2-layer-0-solana-base-layer)
3. [Layer 1: Ephemeral Rollup (ER)](#3-layer-1-ephemeral-rollup-er)
4. [Layer 2: P2P Cast Network](#4-layer-2-p2p-cast-network)
5. [Layer 3: Router + API](#5-layer-3-router--api)
6. [Social Protocol Primitives](#6-social-protocol-primitives)
7. [Data Flow Diagrams](#7-data-flow-diagrams)
8. [Security Model](#8-security-model)
9. [Tech Stack](#9-tech-stack)
10. [Build Roadmap](#10-build-roadmap)

---

## 1. System Overview

The protocol is composed of three execution environments stacked on Solana:

```
┌─────────────────────────────────────────────────────────────────┐
│                    CLIENT APPS / MINI APPS                      │
│         (Web, Mobile, CLI — any Solana-compatible client)       │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                    ┌──────▼──────┐
                    │   ROUTER    │  ← routes txns to right layer
                    └──────┬──────┘
          ┌────────────────┼──────────────────┐
          ▼                ▼                  ▼
  ┌──────────────┐  ┌─────────────┐  ┌──────────────────┐
  │  BASE LAYER  │  │     ER      │  │  P2P CAST NODES  │
  │  (Solana)    │  │  VALIDATOR  │  │  (libp2p gossip) │
  │              │  │  NETWORK    │  │                  │
  │ - Identity   │  │             │  │ - Casts          │
  │ - App Keys   │◄─┤- Soc. Graph │  │ - Replies        │
  │ - Storage    │  │- Channel    │  │ - Reactions      │
  │   Rent       │  │  State      │  │ - Profile data   │
  │ - Usernames  │  │- Sessions   │  │                  │
  └──────────────┘  └─────────────┘  └──────────────────┘
          ▲                │
          └────── commit() ┘
```

### Design Philosophy

| Principle | Implementation |
|---|---|
| User owns identity | FID is a PDA the user controls, not a platform account |
| User owns social graph | Graph settles to Solana; survives any node going down |
| No gas for social actions | Social graph + reactions processed free on ER |
| Content is portable | Casts are signed messages; any node can replicate |
| Composable by default | Onchain PDAs readable by any Solana program |

---

## 2. Layer 0: Solana Base Layer

### 2.1 Programs (Anchor)

Four Anchor programs form the onchain foundation. Each handles only what must be permanent and trustless.

#### A. FID Registry Program

Manages Farcaster-ID equivalent — a unique numeric identity anchored to a wallet.

```rust
// Account: FID Record
#[account]
pub struct FidRecord {
    pub fid: u64,                    // auto-incremented unique ID
    pub custody_address: Pubkey,     // primary wallet (owns the FID)
    pub recovery_address: Pubkey,    // can reclaim if custody key lost
    pub registered_at: i64,         // unix timestamp
    pub storage_units: u16,         // paid storage slots
    pub bump: u8,
}

// Seeds: ["fid", fid.to_le_bytes()]
// Seeds: ["custody", custody_address] → reverse lookup

// Instructions:
// - register(recovery: Pubkey) → creates FID, charges registration fee
// - transfer(new_custody: Pubkey) → moves FID to new wallet
// - recover(new_custody: Pubkey) → recovery address initiates transfer
// - change_recovery(new_recovery: Pubkey)
```

#### B. App Key Registry Program

Delegation layer — apps sign on behalf of users without holding the custody key.

```rust
#[account]
pub struct AppKeyRecord {
    pub fid: u64,
    pub app_pubkey: Pubkey,          // ephemeral key app uses for signing
    pub scope: AppKeyScope,          // what the key is authorized to do
    pub created_at: i64,
    pub expires_at: Option<i64>,     // optional TTL
    pub revoked: bool,
    pub metadata_uri: Option<String>, // app info URI
    pub bump: u8,
}

#[repr(u8)]
pub enum AppKeyScope {
    Full       = 0,   // all message types
    CastsOnly  = 1,   // casts + replies only
    SocialOnly = 2,   // follows/unfollows only
    ReadOnly   = 3,   // no writes, for indexers
}

// Seeds: ["app_key", fid.to_le_bytes(), app_pubkey]

// Instructions:
// - add_app_key(app_pubkey, scope, expires_at, metadata_uri)
// - revoke_app_key(app_pubkey)
// - rotate_app_key(old_pubkey, new_pubkey)
```

#### C. Storage Rent Program

Users pay for storage capacity. This enforces protocol-level storage limits without running centralized servers.

```rust
#[account]
pub struct StorageRecord {
    pub fid: u64,
    pub units: u16,                  // 1 unit = 5000 casts/year + 2500 reactions + 1000 follows
    pub paid_until: i64,             // subscription expiry
    pub bump: u8,
}

// Storage unit pricing: denominated in SOL
// Nodes enforce limits when serving messages — reject from FIDs with expired storage

// Instructions:
// - purchase_storage(fid, units, duration_days) → charges lamports
// - extend_storage(fid, duration_days)
// - get_storage_status(fid) → returns remaining capacity
```

#### D. Username Registry Program

Human-readable names bound to FIDs. Protocol-native TLD (e.g., `.cast`). Optional SNS/ANS integration.

```rust
#[account]
pub struct UsernameRecord {
    pub username: String,            // max 20 chars, alphanumeric + underscore
    pub fid: u64,
    pub registered_at: i64,
    pub expiry: i64,                 // annual renewal
    pub bump: u8,
}

// Seeds: ["username", username_hash]
// Seeds: ["fid_username", fid.to_le_bytes()] → reverse lookup

// Instructions:
// - register_username(username, fid)
// - renew_username(username)
// - transfer_username(username, new_fid)
// - release_username(username)
```

#### E. Delegation Program

The trust anchor for the ER layer. This is the most critical program — it locks PDAs during ER sessions and verifies state commits from ER validators.

```rust
#[account]
pub struct DelegationRecord {
    pub pda: Pubkey,                 // the account being delegated
    pub original_owner: Pubkey,      // program that owns the PDA
    pub delegated_to: Pubkey,        // ER validator pubkey
    pub state_hash: [u8; 32],        // blake3 hash of state at delegation time
    pub delegated_at: i64,
    pub last_commit_slot: u64,
    pub commit_count: u64,
    pub ttl_slots: u64,              // auto-expire; ER must return before this
    pub validator_set: Pubkey,       // which validator set governs this delegation
    pub bump: u8,
}

#[account]
pub struct ValidatorSet {
    pub set_id: u64,
    pub validators: Vec<Pubkey>,     // authorized ER validator pubkeys
    pub threshold: u8,               // M in M-of-N multisig
    pub bump: u8,
}

// Instructions:
// - delegate(pda, validator, ttl_slots)      → locks PDA, records state hash
// - commit(pda, new_state_hash, signatures)  → verifies M-of-N, applies diff
// - undelegate(pda, final_state, signatures) → final commit, returns ownership
// - force_undelegate(pda)                    → callable after TTL expiry
// - register_validator_set(validators, threshold)
// - update_validator_set(set_id, add, remove)
```

---

## 3. Layer 1: Ephemeral Rollup (ER)

The ER is a lightweight SVM execution environment that borrows state from Solana, processes it at high frequency for zero fees, and settles back.

### 3.1 ER Validator Node

Each ER validator is an independent process — not a Solana validator. It uses Solana's extracted `solana-svm` crate to run the exact same execution environment.

```
┌──────────────────────────────────────────────────────┐
│                  ER Validator Node                   │
│                                                      │
│  ┌─────────────────────────────────────────────┐     │
│  │              RPC Server                     │     │
│  │   (SVM-compatible JSON-RPC endpoint)        │     │  ← clients talk here
│  └───────────────────┬─────────────────────────┘     │
│                      │                               │
│  ┌───────────────────▼─────────────────────────┐     │
│  │           Transaction Processor             │     │
│  │   solana-svm crate (same as base layer)     │     │  ← runs Anchor programs
│  └───────────────────┬─────────────────────────┘     │
│                      │                               │
│  ┌───────────────────▼─────────────────────────┐     │
│  │            State Cache (RAM)                │     │
│  │   HashMap<Pubkey, AccountState>             │     │  ← delegated PDAs live here
│  │   Dirty tracking (modified accounts)        │     │
│  └───────────────────┬─────────────────────────┘     │
│                      │                               │
│  ┌───────────────────▼─────────────────────────┐     │
│  │            Commit Engine                    │     │
│  │   - builds state diff                       │     │
│  │   - computes merkle root (blake3)           │     │  ← prepares settlement
│  │   - coordinates M-of-N signing              │     │
│  │   - submits to Delegation Program           │     │
│  └───────────────────┬─────────────────────────┘     │
│                      │                               │
│  ┌───────────────────▼─────────────────────────┐     │
│  │           P2P Sync (libp2p)                 │     │
│  │   - gossip state with peer ER validators    │     │  ← multi-node ER network
│  │   - leader election for commit coordination │     │
│  └─────────────────────────────────────────────┘     │
└──────────────────────────────────────────────────────┘
```

### 3.2 ER Session Lifecycle

```
   CLIENT                BASE LAYER           ER VALIDATOR
      │                      │                     │
      │   delegate(pda)       │                     │
      │──────────────────────►│                     │
      │                      │ lock PDA             │
      │                      │ emit DelegationEvent │
      │                      │──────────────────────►
      │                      │              load state into cache
      │                      │                     │
      │   send txn            │                     │
      │────────────────────────────────────────────►│
      │                      │         process in SVM, update cache
      │◄────────────────────────────────────────────│
      │   confirmation (sub-100ms)                  │
      │                      │                     │
      │   [many txns...]      │                     │
      │                      │                     │
      │   undelegate()        │                     │
      │────────────────────────────────────────────►│
      │                      │    build state diff  │
      │                      │    sign merkle root  │
      │                      │    commit(diff, sigs)│
      │                      │◄─────────────────────│
      │                      │  verify M-of-N sigs  │
      │                      │  apply state diff    │
      │                      │  return PDA ownership│
      │◄──────────────────────│                     │
      │   undelegate confirmed│                     │
```

### 3.3 Commit Protocol

Commits are the trust boundary between ER and base layer.

```
Commit Trigger (any of):
  - Explicit: client calls commit()
  - Periodic: every N slots (configurable, default 1000)
  - Threshold: when dirty accounts > X bytes
  - Session end: on undelegate()

Commit Steps:
  1. ER leader builds merkle tree of all dirty accounts
     merkle_root = blake3(sorted(account_pubkey + account_data))

  2. Leader broadcasts merkle_root to peer ER validators
     Peers independently verify against their local state copy

  3. M-of-N validators sign: sign(merkle_root + slot_number + validator_pubkey)

  4. Leader submits CommitInstruction to Delegation Program:
     {
       pda_updates: Vec<(Pubkey, AccountData)>,
       merkle_root: [u8; 32],
       slot: u64,
       signatures: Vec<(Pubkey, Signature)>   // must be >= threshold
     }

  5. Delegation Program:
     a. verifies each signature is from a registered validator
     b. verifies count >= threshold
     c. recomputes merkle root from pda_updates
     d. verifies matches submitted root
     e. applies account diffs
     f. emits CommitEvent
```

### 3.4 Validator Selection & Staking

```rust
// Validators stake SOL to join a ValidatorSet
// Slashing: if validator submits invalid commit that passes M-of-N, 
// challenge period allows fraud proof → stake slashed

// For Phase 1 (PoA): known validators, no slashing, governance-controlled set
// For Phase 2: open validator set with staking + challenge window (optimistic)
// For Phase 3: ZK validity proofs per commit (full trustlessness)
```

### 3.5 Social PDAs in the ER

These accounts are delegated to ER during active social sessions:

```rust
// Social Graph PDA — lives in ER during active use, committed to base layer
#[account]
pub struct SocialGraph {
    pub fid: u64,
    pub following: Vec<u64>,         // ordered list of followed FIDs
    pub following_count: u32,
    pub followers_count: u32,        // maintained by incoming follow events
    pub last_updated_slot: u64,
    pub bump: u8,
}

// Channel State PDA
#[account]
pub struct ChannelState {
    pub channel_id: Pubkey,
    pub name: String,
    pub creator_fid: u64,
    pub member_count: u32,
    pub pinned_cast_hash: Option<[u8; 32]>,
    pub last_updated_slot: u64,
    pub bump: u8,
}

// Reaction Aggregate PDA — batches likes/recasts before settling
#[account]
pub struct ReactionAggregate {
    pub target_hash: [u8; 32],       // hash of the target cast
    pub likes: u64,
    pub recasts: u64,
    pub quotes: u64,
    pub last_updated_slot: u64,
    pub bump: u8,
}
```

---

## 4. Layer 2: P2P Cast Network

Message content (casts, replies, profile updates) never needs to settle onchain. This is the Snapchain equivalent — a decentralized gossip network of nodes that store and replicate signed messages.

### 4.1 Cast Node Architecture

```
┌──────────────────────────────────────────────────────┐
│                   Cast Node (Go)                     │
│                                                      │
│  ┌─────────────────────────────────────────────┐     │
│  │           libp2p Gossip Layer               │     │
│  │   - GossipSub for message propagation       │     │
│  │   - Kademlia DHT for peer discovery         │     │
│  │   - Noise protocol for encryption           │     │
│  └───────────────────┬─────────────────────────┘     │
│                      │                               │
│  ┌───────────────────▼─────────────────────────┐     │
│  │           Message Validator                 │     │
│  │   1. Decode protobuf message                │     │
│  │   2. Look up signer in App Key Registry     │     │  ← Solana RPC call
│  │      (on Solana base layer)                 │     │
│  │   3. Verify ed25519 signature               │     │
│  │   4. Check FID storage limit                │     │  ← Solana RPC call
│  │   5. Check for duplicate hash              │     │
│  └───────────────────┬─────────────────────────┘     │
│                      │                               │
│  ┌───────────────────▼─────────────────────────┐     │
│  │           Message Store (RocksDB)           │     │
│  │   - keyed by: fid + type + timestamp        │     │
│  │   - pruned per storage unit limits          │     │
│  │   - content-addressed by message hash       │     │
│  └───────────────────┬─────────────────────────┘     │
│                      │                               │
│  ┌───────────────────▼─────────────────────────┐     │
│  │              HTTP/gRPC API                  │     │
│  │   - getCastsByFid(fid, pageSize, cursor)    │     │
│  │   - getCast(hash)                           │     │
│  │   - submitMessage(msg)                      │     │
│  │   - getFollowersByFid(fid)                  │     │  ← reads from ER/base layer
│  └─────────────────────────────────────────────┘     │
└──────────────────────────────────────────────────────┘
```

### 4.2 Message Format

All messages are protobuf-encoded, ed25519-signed by an app key.

```protobuf
message Message {
  MessageData data        = 1;
  bytes hash              = 2;  // blake3(data)
  HashScheme hash_scheme  = 3;  // BLAKE3
  bytes signature         = 4;  // ed25519 signature over hash
  SignatureScheme sig_scheme = 5;
  bytes signer            = 6;  // app key pubkey (must exist in App Key Registry)
}

message MessageData {
  MessageType type        = 1;
  uint64 fid              = 2;
  uint32 timestamp        = 3;  // seconds since epoch
  Network network         = 4;  // mainnet / testnet
  oneof body {
    CastAddBody        cast_add_body        = 5;
    CastRemoveBody     cast_remove_body     = 6;
    ReactionBody       reaction_body        = 7;
    LinkBody           link_body            = 8;  // follow/unfollow
    UserDataBody       user_data_body       = 9;  // profile updates
    ChannelBody        channel_body         = 10;
  }
}

message CastAddBody {
  string text                     = 1;  // max 320 chars
  repeated uint64 mentions        = 2;  // mentioned FIDs
  repeated Embed embeds           = 3;  // urls, media hashes
  ParentCast parent               = 4;  // null if top-level
  string channel_id               = 5;  // optional channel
}
```

### 4.3 Storage Enforcement

Nodes enforce per-FID storage limits derived from the onchain StorageRecord:

```
1 storage unit = 5,000 casts + 2,500 reactions + 1,000 links + 100 userdata
Oldest messages are pruned when limit is reached (FIFO per type)
Nodes reject new messages from FIDs with expired storage rent
```

### 4.4 Node Sync Protocol

```
New node joining the network:
  1. Bootstrap from known peers (hardcoded + DNS seed)
  2. Kademlia peer discovery
  3. Request sync from peers: getSyncStatus()
  4. Download messages in batches using merkle trie comparison
  5. Validate all messages against Solana base layer
  6. Mark self as synced, start accepting submissions
```

---

## 5. Layer 3: Router + API

### 5.1 Transaction Router

A stateless proxy that inspects transactions and routes to the correct execution layer. Similar to MagicBlock's Magic Router, but protocol-owned.

```
Routing Rules:

  - Identity instructions (register_fid, add_app_key, etc.)
      → Solana base layer (non-negotiable, permanent state)

  - Social graph operations (follow, unfollow, channel join/leave)
      → ER validator network (if PDA is delegated)
      → Solana base layer (if PDA not yet delegated)

  - Reaction aggregates (like, recast)
      → ER validator network

  - Cast content (cast_add, cast_remove, reaction_add)
      → P2P Cast Nodes (never touches chain)

  - Reads (getFid, getCast, getSocialGraph)
      → Indexer / Cache layer
```

```typescript
// Router logic (simplified)
async function routeTransaction(tx: Transaction): Promise<string> {
  const programId = tx.instructions[0].programId;
  const data = tx.instructions[0].data;
  const discriminator = data.slice(0, 8);

  if (isIdentityInstruction(programId, discriminator)) {
    return sendToSolana(tx);
  }

  if (isSocialGraphInstruction(programId, discriminator)) {
    const pda = tx.instructions[0].keys[0].pubkey;
    const delegated = await checkDelegationStatus(pda);
    return delegated
      ? sendToER(tx, delegated.validatorEndpoint)
      : sendToSolana(tx);
  }

  throw new Error("Unknown instruction type");
}
```

### 5.2 Indexer

An off-the-shelf Postgres + event listener that indexes onchain events and P2P messages into queryable tables for fast API reads.

```sql
-- Core tables
CREATE TABLE fids (
  fid BIGINT PRIMARY KEY,
  custody_address TEXT NOT NULL,
  recovery_address TEXT NOT NULL,
  registered_at TIMESTAMP NOT NULL,
  storage_units INT DEFAULT 0,
  username TEXT
);

CREATE TABLE app_keys (
  fid BIGINT,
  app_pubkey TEXT,
  scope INT,
  created_at TIMESTAMP,
  revoked BOOLEAN DEFAULT false,
  PRIMARY KEY (fid, app_pubkey)
);

CREATE TABLE social_graph (
  follower_fid BIGINT,
  following_fid BIGINT,
  created_at TIMESTAMP,
  deleted_at TIMESTAMP,
  PRIMARY KEY (follower_fid, following_fid)
);

CREATE TABLE casts (
  hash TEXT PRIMARY KEY,
  fid BIGINT,
  parent_hash TEXT,
  channel_id TEXT,
  text TEXT,
  timestamp TIMESTAMP,
  deleted BOOLEAN DEFAULT false
);

CREATE TABLE reactions (
  hash TEXT PRIMARY KEY,
  fid BIGINT,
  type INT,               -- 1=like, 2=recast
  target_hash TEXT,
  timestamp TIMESTAMP,
  deleted BOOLEAN DEFAULT false
);
```

---

## 6. Social Protocol Primitives

### 6.1 Full Primitive Map

| Primitive | Storage Location | Execution Layer | Settlement |
|---|---|---|---|
| FID registration | Solana | Solana | Permanent |
| App key add/revoke | Solana | Solana | Permanent |
| Storage rent payment | Solana | Solana | Permanent |
| Username register | Solana | Solana | Permanent |
| Follow / Unfollow | ER (delegated PDA) | ER | Committed periodically |
| Channel join/leave | ER (delegated PDA) | ER | Committed periodically |
| Reaction aggregates | ER (delegated PDA) | ER | Committed periodically |
| Cast content | P2P nodes | Off-chain | Never — storage enforced by rent |
| Profile updates | P2P nodes | Off-chain | Never — verified by app key |
| Media/embeds | P2P + IPFS/Arweave | Off-chain | Content-addressed |

### 6.2 Identity Recovery

```
User loses custody key:
  1. Recovery address calls recover(new_custody) on FID Registry
  2. Recovery requires timelock (e.g., 48h) to prevent griefing
  3. After timelock: FID transferred to new_custody
  4. All existing app keys remain valid
  5. Old custody key loses control

User loses both keys:
  → Protocol has no recovery path (self-custody responsibility)
  → Best practice: use a multisig as recovery address
```

---

## 7. Data Flow Diagrams

### 7.1 User Registration Flow

```
User
 │
 ├─1─► FID Registry Program (Solana)
 │       register(recovery_address)
 │       ← emits FID = 12345
 │
 ├─2─► Username Registry Program (Solana)
 │       register_username("alice", fid=12345)
 │
 ├─3─► Storage Rent Program (Solana)
 │       purchase_storage(fid=12345, units=1, duration=365days)
 │
 └─4─► App Key Registry Program (Solana)
         add_app_key(app_pubkey, scope=Full)

→ User is now fully registered. Can post casts + follow others.
```

### 7.2 Social Session Flow (Follow / Unfollow)

```
User opens client app
 │
 ├─1─► Delegation Program: delegate(social_graph_pda, er_validator)
 │       PDA locked on Solana, ownership transferred to ER
 │
 ├─2─► ER Validator: follow(fid=12345, target=67890)
 │       Processed in SVM, state updated in memory
 │       Confirmation: ~50ms
 │
 ├─3─► ER Validator: follow(fid=12345, target=11111)
 ├─4─► ER Validator: unfollow(fid=12345, target=67890)
 │       (all free, all instant)
 │
 └─5─► [session end OR periodic trigger]
         ER builds state diff
         M-of-N validators sign merkle root
         CommitInstruction submitted to Solana
         PDA updated on base layer
         Ownership returned to social program
```

### 7.3 Cast Publish Flow

```
User writes cast
 │
 ├─1─► Client signs CastAddBody with app key (ed25519)
 │
 ├─2─► Client submits to Cast Node via HTTP
 │       POST /v1/submitMessage { message }
 │
 ├─3─► Cast Node validates:
 │       - app key exists in App Key Registry (Solana RPC)
 │       - signature verifies against app key
 │       - FID has storage units (Solana RPC)
 │       - no duplicate hash
 │
 ├─4─► Cast Node stores in RocksDB
 │
 └─5─► Cast Node gossips to peers via GossipSub
         Peers validate + store
         Message propagates across P2P network (~1-2s full propagation)
```

---

## 8. Security Model

### 8.1 Trust Hierarchy

```
Solana Validators (L1 consensus)
    └── Delegation Program (verifies ER commits)
            └── ER Validator Set (M-of-N multisig over state diffs)
                    └── Cast Nodes (verify signatures against base layer)
                            └── App Keys (user-revokable, scoped)
                                    └── Client Apps
```

### 8.2 Threat Analysis

| Threat | Mitigation |
|---|---|
| ER validator submits fraudulent state | M-of-N threshold; single validator can't commit alone |
| ER validators collude (< M compromise) | Commit rejected by Delegation Program |
| ER validators collude (>= M compromise) | Optimistic challenge window (Phase 2); ZK proofs (Phase 3) |
| Cast node censors messages | P2P gossip; any node can receive + propagate |
| Cast node stores wrong data | Signature verification; clients re-validate |
| App key compromised | User revokes key on Solana; nodes stop accepting signatures |
| FID stolen | Recovery address can reclaim with timelock |
| Storage spam | Storage rent enforces hard limits per FID |
| Delegation never returned (ER offline) | TTL expiry on delegation → force_undelegate after timeout |

### 8.3 ER Validity Progression

```
Phase 1 (Launch): Proof of Authority
  - Known, permissioned validator set
  - Honest: published identities, reputational stake
  - Simple, fast to implement

Phase 2 (Growth): Optimistic with Fraud Proofs
  - Open validator set with SOL stake
  - 24h challenge window per commit
  - Anyone can submit fraud proof → slashes stake
  - Requires fraud proof program on Solana

Phase 3 (Maturity): ZK Validity Proofs
  - Each commit accompanied by SNARK proof
  - Delegation Program verifies proof on-chain
  - Fully trustless, no challenge window needed
  - Higher compute cost at commit time
```

---

## 9. Tech Stack

### Base Layer
| Component | Technology |
|---|---|
| Smart contracts | Rust + Anchor Framework |
| Solana version | Solana 2.x |
| Testing | anchor-bankrun, solana-program-test |
| Client SDK | TypeScript (@solana/web3.js or @solana/kit) |

### ER Validator Node
| Component | Technology |
|---|---|
| Language | Rust |
| SVM Execution | solana-svm crate |
| State storage | In-memory HashMap + RocksDB persistence |
| Merkle tree | blake3 + custom merkle implementation |
| P2P sync | libp2p-rs (gossipsub + kademlia) |
| RPC server | JSON-RPC compatible with Solana RPC spec |
| Commit signing | ed25519-dalek |

### P2P Cast Node
| Component | Technology |
|---|---|
| Language | Go |
| P2P layer | go-libp2p |
| Message format | Protocol Buffers (protobuf3) |
| Message store | RocksDB (via grocksdb bindings) |
| Solana client | github.com/gagliardetto/solana-go |
| API | gRPC + HTTP/JSON gateway |

### Transaction Router
| Component | Technology |
|---|---|
| Language | TypeScript / Go |
| Deployment | Cloudflare Workers or lightweight Go binary |
| State check | Solana RPC (delegation status) |

### Indexer
| Component | Technology |
|---|---|
| Database | PostgreSQL |
| Chain listener | Custom Solana log subscriber (WebSocket) |
| Cast listener | Cast Node gRPC stream |
| API | REST + GraphQL |
| Cache | Redis |

### Client SDK
| Component | Technology |
|---|---|
| Core | TypeScript |
| Wallet adapter | @solana/wallet-adapter |
| Message signing | tweetnacl (ed25519) |
| Protobuf | protobufjs |
| Distribution | npm package |

---

## 10. Build Roadmap

### Phase 1 — Foundation (Months 1–2)
**Goal:** Core Solana programs + minimal ER on devnet

- [ ] FID Registry Program (Anchor)
- [ ] App Key Registry Program (Anchor)
- [ ] Storage Rent Program (Anchor)
- [ ] Username Registry Program (Anchor)
- [ ] Delegation Program (Anchor) — PoA, trusted validator set
- [ ] ER Validator Node v0 — single node, no P2P sync
- [ ] ER counter/graph test (equivalent to MagicBlock's counter example)
- [ ] Devnet deployment + integration tests
- [ ] TypeScript SDK v0 (register FID, add app key, delegate)

### Phase 2 — P2P Cast Layer (Months 3–4)
**Goal:** Full message network; clients can post + read casts

- [ ] Cast Node in Go (libp2p, RocksDB, protobuf)
- [ ] Message validation against Solana App Key Registry
- [ ] Storage limit enforcement
- [ ] Node sync protocol (merkle trie diff)
- [ ] Cast Node gRPC + HTTP API
- [ ] Indexer (Postgres, chain listener, cast listener)
- [ ] Transaction Router v0
- [ ] TypeScript SDK v1 (full cast + social primitives)

### Phase 3 — ER Multi-Node + Social Programs (Months 5–6)
**Goal:** Production-grade ER; social graph fully delegated

- [ ] ER Validator P2P sync (libp2p-rs)
- [ ] M-of-N commit protocol
- [ ] Social Graph PDA with ER delegation hooks
- [ ] Channel State PDA with ER delegation hooks
- [ ] Reaction Aggregate PDA with ER delegation hooks
- [ ] Post-commit Magic Actions (reputation, storage credit hooks)
- [ ] Mainnet deployment

### Phase 4 — Ecosystem (Month 7+)
**Goal:** Developer adoption

- [ ] Mini App framework (iframe + SDK, Farcaster-compatible subset)
- [ ] Sign In with Tribe (auth flow, analogous to Sign In with Farcaster)
- [ ] Reference client (web app)
- [ ] Developer docs + quickstart
- [ ] Validator onboarding guide (run your own ER node)
- [ ] Optimistic fraud proofs (Phase 2 security upgrade)

---

## Appendix A: Key Design Decisions

### Why not use an existing ER (MagicBlock)?
Building Tribe's own ER means the protocol does not depend on a third-party infrastructure provider. The Tribe ER validator network becomes its own ecosystem — other protocols (games, DeFi, other social apps) can use it. The ER is a moat, not a vendor relationship.

### Why Solana over Ethereum/OP?
- Native ed25519 support (same key type used for message signing — no conversion layer)
- Account model is a natural fit for PDA-per-user identity
- Solana's `solana-svm` crate is designed for exactly this ER use case
- Cost: identity registration + storage rent in fractions of a cent

### Why Go for Cast Nodes?
- go-libp2p is the most mature libp2p implementation (same used by IPFS, Filecoin)
- Goroutine model handles high P2P concurrency well
- gagliardetto/solana-go is production-grade
- Easy for the community to run nodes (single binary)

### Why not put casts onchain or on the ER?
Cast content has no need for canonical settlement. The ER is optimized for state that must eventually be owned and composable (social graph, channel membership). Cast content is ephemeral in the sense that storage limits prune old messages — putting it through the ER/Solana pipeline adds cost with no benefit.

### Commit frequency for social graph
Default: periodic (every 1000 slots ~6.4min) + explicit on session end. This means:
- Social graph is eventually consistent (base layer lags ~6 min max)
- Real-time for all apps reading from the ER or indexer
- Base layer holds truth for composability with other protocols

---

## Appendix B: Message Type Reference

| Type ID | Name | Description | Layer |
|---|---|---|---|
| 1 | CAST_ADD | New cast or reply | P2P |
| 2 | CAST_REMOVE | Soft-delete a cast | P2P |
| 3 | REACTION_ADD | Like or recast | P2P + ER aggregate |
| 4 | REACTION_REMOVE | Remove reaction | P2P + ER aggregate |
| 5 | LINK_ADD | Follow a user | P2P + ER graph |
| 6 | LINK_REMOVE | Unfollow a user | P2P + ER graph |
| 7 | USER_DATA_ADD | Update profile field | P2P |
| 8 | USERNAME_PROOF | Verify username ownership | P2P |
| 9 | CHANNEL_ADD | Create channel | Solana |
| 10 | CHANNEL_JOIN | Join channel | ER |
| 11 | CHANNEL_LEAVE | Leave channel | ER |

---

*Tribe Protocol — Architecture version 0.1 — subject to revision as implementation progresses.*