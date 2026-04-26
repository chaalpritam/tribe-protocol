# tribe-protocol

Solana programs (Anchor) for decentralized social identity and graph.

Tribe is a fully-owned, open social protocol on Solana. This repo contains eight on-chain programs covering identity, app keys, usernames, the social graph (with ER delegation), hub discovery, on-chain tip receipts, crowdfund campaigns with escrow, and local tasks with optional reward escrow.

## Programs

| Program | Program ID | Instructions |
|---------|-----------|--------------|
| **tid-registry** | `4BSmJmRGQWKgioP9DG2bUuRS9U3V6soRauU7Nv6yGvHD` | `initialize`, `register`, `transfer`, `recover`, `change_recovery` |
| **app-key-registry** | `5LtbFUeAoXWRovGpyWnRJhiCS62XsTYKVErT9kPpv4hN` | `add_key`, `revoke_key`, `rotate_key` |
| **username-registry** | `65oKjSjcGYR61ASzDYczbodz6H8TARtJyQGvb5V9y9W1` | `register`, `renew`, `transfer`, `release` |
| **social-graph** | `8kKnWvbmTjWq5uPePk79RRbQMAXCszNFzHdRwUS4N74w` | `init_profile`, `follow`, `unfollow`, `init_sequencer`, `init_profile_delegated`, `follow_delegated`, `unfollow_delegated` |
| **hub-registry** | `HubReg1111111111111111111111111111111111111` | `register_hub`, `update_hub`, `heartbeat`, `deactivate_hub` |
| **tip-registry** | `TipReg1111111111111111111111111111111111111` | `init_sender_state`, `send_tip` |
| **crowdfund-registry** | `CrowdF11111111111111111111111111111111111111` | `init_creator_state`, `create_crowdfund`, `pledge`, `claim_funds`, `refund` |
| **task-registry** | `TaskReg111111111111111111111111111111111111` | `init_creator_state`, `create_task`, `claim_task`, `complete_task`, `cancel_task` |

## Architecture

### TID System

Every user gets a unique auto-incrementing 64-bit numeric identity (TID). The `GlobalState` account holds the counter and governance authority. Each TID maps to a `TidRecord` containing the custody address (primary wallet) and a recovery address that can reclaim the TID if the custody key is lost. A `CustodyLookup` account provides reverse lookup from wallet address to TID.

Key operations:
- **register** -- allocates the next TID and binds it to the caller's wallet
- **transfer** -- moves custody to a new address (signed by current custody)
- **recover** -- reclaims the TID using the recovery address
- **change_recovery** -- updates the recovery address (signed by custody)

### Social Graph (PDA-per-Relationship)

Instead of storing followers in a `Vec<u64>` (which would hit Solana's 10 MB account limit), each follow relationship is its own PDA:

- **SocialProfile** (25 bytes) -- one per user, stores only counters (`following_count`, `followers_count`)
- **Link** (33 bytes) -- one per follow relationship
  - Seeds: `["link", follower_tid, following_tid]`
  - O(1) follow/unfollow (create/close a single small PDA)
  - O(1) existence check (derive the PDA address, check if it exists)
  - Unlimited follows per user
  - Unfollow reclaims rent (~0.001 SOL returned to follower)

#### ER Delegation (instant follows)

The base instructions (`init_profile`, `follow`, `unfollow`) cost SOL and require user signatures. To make follows instant and free at the UX layer, the program also exposes a delegated path used by [tribe-er-server](../tribe-er-server):

- **`init_sequencer`** -- one-time setup; admin authorizes a single ER server pubkey via the `SequencerConfig` PDA.
- **`init_profile_delegated` / `follow_delegated` / `unfollow_delegated`** -- same effect as the base instructions, but signed by the ER server on behalf of the user. The server collects signed user intents off-chain and settles them on L1 in batches.

The non-delegated path remains available — any wallet can still call `follow` directly without the sequencer.

### App Keys (Scoped Delegation)

Apps can request a delegation key that lets them sign messages on behalf of a user without holding the user's main wallet key. Each `AppKeyRecord` stores:

- The TID it belongs to
- An ephemeral app pubkey
- A permission scope: `0` = Full, `1` = TweetsOnly, `2` = SocialOnly, `3` = ReadOnly
- An optional expiry timestamp (`0` = no expiry)
- A revocation flag

Users can revoke keys at any time. Keys can also be rotated to a new pubkey.

### Usernames (.tribe)

Human-readable names (up to 20 characters) bound to TIDs. Each `UsernameRecord` stores the name, the bound TID, registration time, and an expiry timestamp. Usernames require annual renewal. A `TidUsername` reverse lookup maps each TID to the hash of its current username.

### Hub Registry

A discovery layer for the gossip network. Anyone can run a hub; the registry lets clients find healthy peers without a central directory:

- **register_hub** -- operator publishes a `HubRecord` (URL + gossip pubkey) anchored to their wallet.
- **update_hub** -- rotate the URL or gossip key.
- **heartbeat** -- refresh `last_heartbeat` to prove liveness; clients filter on this when choosing peers.
- **deactivate_hub** -- mark a hub inactive (e.g., before retiring it).

### Tip Registry

On-chain tip receipts plus the SOL transfer in a single instruction. Each sender has a `SenderTipState` (PDA seeded by their wallet) holding a monotonic `next_tip_id`; every tip becomes a `TipRecord` PDA seeded by `["tip", sender_pubkey, tip_id_le]`.

- **init_sender_state(sender_tid)** -- one-time per sender. Creates the counter PDA so subsequent tips have deterministic addresses.
- **send_tip(recipient_tid, amount, target_hash, has_target)** -- transfers `amount` lamports from the signer to `recipient` via System Program CPI, then writes the immutable `TipRecord`. Rejects zero amounts and self-tips. `target_hash` optionally anchors the tip to a piece of content (e.g. the blake3 hash of a tweet).

### Crowdfund Registry

On-chain crowdfunding campaigns with escrow. Each creator has a `CreatorCrowdfundState` (PDA seeded by their wallet) tracking `next_crowdfund_id`; each campaign is a `Crowdfund` PDA at `["crowdfund", creator_pubkey, crowdfund_id_le]` that doubles as the lamport vault. Per-backer pledges are tracked in `Pledge` PDAs seeded by `["pledge", crowdfund_pubkey, backer_pubkey]`.

- **init_creator_state(creator_tid)** -- one-time per creator.
- **create_crowdfund(goal_amount, deadline_at, metadata_hash)** -- starts a campaign. `metadata_hash` anchors the off-chain `CROWDFUND_ADD` envelope (title / description / image / currency).
- **pledge(backer_tid, amount)** -- transfers lamports into the Crowdfund PDA, updates the backer's `Pledge` (creating it on first pledge, accumulating on repeat). Rejected after the deadline or on a non-active campaign.
- **claim_funds()** -- creator only. After the deadline, if `total_pledged >= goal_amount`, sweeps the campaign vault into the creator's wallet and flips status to `Succeeded`.
- **refund()** -- backer only. After the deadline, if the goal wasn't met, returns the backer's pledge from the vault and closes the `Pledge` PDA (rent goes back to the backer). First refund flips status `Active` → `Failed`; subsequent refunds are idempotent.

### Task Registry

Local tasks with an optional reward escrow. Each creator has a `CreatorTaskState` (PDA seeded by their wallet) tracking `next_task_id`; each task is a `Task` PDA at `["task", creator_pubkey, task_id_le]` that doubles as the reward vault when `reward_amount > 0`.

State machine: `Open` → `Claimed` → `Completed` (or `Open` → `Cancelled`). Once a task has been claimed, the creator can't cancel it; the only path forward is `complete_task` (which releases the escrow to the claimer).

- **init_creator_state(creator_tid)** -- one-time per creator.
- **create_task(reward_amount, metadata_hash)** -- creates an open task; if `reward_amount > 0`, escrows that many lamports into the Task PDA. `metadata_hash` anchors the off-chain `TASK_ADD` envelope (title / description / reward_text).
- **claim_task(claimer_tid)** -- claimer signs to lock the task to themselves. Creator can't claim their own task.
- **complete_task()** -- creator-only. Releases the escrowed reward (if any) to the claimer and flips status to `Completed`.
- **cancel_task()** -- creator-only. Refunds any escrow back to the creator. Only valid while the task is still `Open`.

## Account Structures

### tid-registry

| Account | Size | Fields |
|---------|------|--------|
| `GlobalState` | 49 bytes | `tid_counter: u64`, `authority: Pubkey`, `bump: u8` |
| `TidRecord` | 89 bytes | `tid: u64`, `custody_address: Pubkey`, `recovery_address: Pubkey`, `registered_at: i64`, `bump: u8` |
| `CustodyLookup` | 17 bytes | `tid: u64`, `bump: u8` |

### app-key-registry

| Account | Size | Fields |
|---------|------|--------|
| `AppKeyRecord` | 67 bytes | `tid: u64`, `app_pubkey: Pubkey`, `scope: u8`, `created_at: i64`, `expires_at: i64`, `revoked: bool`, `bump: u8` |

### username-registry

| Account | Size | Fields |
|---------|------|--------|
| `UsernameRecord` | 54 bytes | `username: [u8; 20]`, `username_len: u8`, `tid: u64`, `registered_at: i64`, `expiry: i64`, `bump: u8` |
| `TidUsername` | 41 bytes | `username_hash: [u8; 32]`, `bump: u8` |

### social-graph

| Account | Size | Fields |
|---------|------|--------|
| `SocialProfile` | 25 bytes | `tid: u64`, `following_count: u32`, `followers_count: u32`, `bump: u8` |
| `Link` | 33 bytes | `follower_tid: u64`, `following_tid: u64`, `created_at: i64`, `bump: u8` |
| `SequencerConfig` | 73 bytes | `authority: Pubkey`, `admin: Pubkey`, `bump: u8` |

### hub-registry

| Account | Size | Fields |
|---------|------|--------|
| `HubRecord` | 219 bytes | `operator: Pubkey`, `url: [u8; 128]`, `url_len: u8`, `gossip_key: Pubkey`, `registered_at: i64`, `last_heartbeat: i64`, `active: bool`, `bump: u8` |

### tip-registry

| Account | Size | Fields |
|---------|------|--------|
| `SenderTipState` | 57 bytes | `sender: Pubkey`, `sender_tid: u64`, `next_tip_id: u64`, `bump: u8` |
| `TipRecord` | 146 bytes | `sender: Pubkey`, `recipient: Pubkey`, `sender_tid: u64`, `recipient_tid: u64`, `amount: u64`, `tip_id: u64`, `created_at: i64`, `target_hash: [u8; 32]`, `has_target: bool`, `bump: u8` |

### crowdfund-registry

| Account | Size | Fields |
|---------|------|--------|
| `CreatorCrowdfundState` | 57 bytes | `creator: Pubkey`, `creator_tid: u64`, `next_crowdfund_id: u64`, `bump: u8` |
| `Crowdfund` | 126 bytes | `creator: Pubkey`, `creator_tid: u64`, `crowdfund_id: u64`, `goal_amount: u64`, `total_pledged: u64`, `pledge_count: u32`, `deadline_at: i64`, `created_at: i64`, `status: u8`, `bump: u8`, `metadata_hash: [u8; 32]` |
| `Pledge` | 97 bytes | `crowdfund: Pubkey`, `backer: Pubkey`, `backer_tid: u64`, `amount: u64`, `pledged_at: i64`, `bump: u8` |

### task-registry

| Account | Size | Fields |
|---------|------|--------|
| `CreatorTaskState` | 57 bytes | `creator: Pubkey`, `creator_tid: u64`, `next_task_id: u64`, `bump: u8` |
| `Task` | 171 bytes | `creator: Pubkey`, `creator_tid: u64`, `task_id: u64`, `status: u8`, `reward_amount: u64`, `claimer: Pubkey`, `claimer_tid: u64`, `has_claimer: bool`, `created_at: i64`, `claimed_at: i64`, `completed_at: i64`, `metadata_hash: [u8; 32]`, `bump: u8` |

Note: all sizes include the 8-byte Anchor discriminator.

## Prerequisites

- Rust (nightly toolchain)
- Solana CLI 2.x+
- Anchor CLI 0.31.1
- Node.js 18+
- pnpm

## Build and Test

```bash
# Install JS dependencies
pnpm install

# Build all five programs
anchor build

# Run the full test suite (uses a local validator)
anchor test

# Deploy to devnet
anchor deploy --provider.cluster devnet

# Deploy to mainnet
anchor deploy --provider.cluster mainnet
```

## Project Structure

```
tribe-protocol/
├── Anchor.toml                   # Anchor workspace config + program IDs
├── Cargo.toml                    # Rust workspace
├── programs/
│   ├── tid-registry/
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── state/            # GlobalState, TidRecord, CustodyLookup
│   │       ├── instructions/     # initialize, register, transfer, recover, change_recovery
│   │       ├── errors.rs
│   │       └── events.rs
│   ├── app-key-registry/
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── state/            # AppKeyRecord
│   │       ├── instructions/     # add_key, revoke_key, rotate_key
│   │       ├── errors.rs
│   │       └── events.rs
│   ├── username-registry/
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── state/            # UsernameRecord, TidUsername
│   │       ├── instructions/     # register, renew, transfer, release
│   │       ├── errors.rs
│   │       └── events.rs
│   ├── social-graph/
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── state/            # SocialProfile, Link, SequencerConfig
│   │       ├── instructions/     # init_profile, follow, unfollow,
│   │       │                     # init_sequencer, init_profile_delegated,
│   │       │                     # follow_delegated, unfollow_delegated
│   │       ├── errors.rs
│   │       └── events.rs
│   ├── hub-registry/
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── state/            # HubRecord
│   │       ├── instructions/     # register_hub, update_hub, heartbeat, deactivate_hub
│   │       ├── errors.rs
│   │       └── events.rs
│   ├── tip-registry/
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── state/            # SenderTipState, TipRecord
│   │       ├── instructions/     # init_sender_state, send_tip
│   │       ├── errors.rs
│   │       └── events.rs
│   ├── crowdfund-registry/
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── state/            # CreatorCrowdfundState, Crowdfund, Pledge
│   │       ├── instructions/     # init_creator_state, create_crowdfund,
│   │       │                     # pledge, claim_funds, refund
│   │       ├── errors.rs
│   │       └── events.rs
│   └── task-registry/
│       └── src/
│           ├── lib.rs
│           ├── state/            # CreatorTaskState, Task
│           ├── instructions/     # init_creator_state, create_task,
│           │                     # claim_task, complete_task, cancel_task
│           ├── errors.rs
│           └── events.rs
├── tests/                        # Anchor integration tests (TypeScript)
│   └── tribe-protocol.ts
├── migrations/
└── scripts/
```

## Related Repos

| Repo | Description |
|------|-------------|
| [tribe-sdk](../tribe-sdk) | TypeScript SDK -- identity, social graph, tweet clients |
| [tribe-hub](../tribe-hub) | Decentralized hub -- tweet storage, indexing, gossip sync |
| [tribe-er-server](../tribe-er-server) | Ephemeral Rollup sequencer -- instant follows, batched L1 settlement |
| [tribe-app](../tribe-app) | Next.js frontend -- 10 pages with multi-node failover |

## License

MIT
