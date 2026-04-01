# Tribe Protocol

Solana programs (Anchor) for decentralized social identity and graph.

Tribe is a fully-owned, open social protocol on Solana. This repo contains the on-chain programs — the foundation layer that everything else builds on.

## Programs

| Program | Description | Key Accounts |
|---------|-------------|--------------|
| **fid-registry** | Unique numeric identity (FID) per user | `GlobalState`, `FidRecord`, `CustodyLookup` |
| **app-key-registry** | Scoped delegation keys for apps | `AppKeyRecord` |
| **username-registry** | Human-readable names (.cast) bound to FIDs | `UsernameRecord`, `FidUsername` |
| **social-graph** | Follow/unfollow with PDA-per-relationship | `SocialProfile`, `Link` |

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                  Client / SDK                        │
└──────────┬──────────────────┬───────────────────────┘
           │                  │
   ┌───────▼────────┐  ┌─────▼──────────────────────┐
   │  fid-registry   │  │  social-graph               │
   │  app-key-registry│  │  (PDA-per-relationship)    │
   │  username-registry│  │  SocialProfile + Link PDAs │
   └─────────────────┘  └────────────────────────────┘
```

### Social Graph Design (PDA-per-Relationship)

Instead of storing followers in a `Vec<u64>` (which hits Solana's 10MB account limit), each follow relationship is its own PDA:

- **`SocialProfile`** — one per user, stores only counters (25 bytes)
- **`Link`** — one per follow relationship (33 bytes)
  - Seeds: `["link", follower_fid, following_fid]`
  - O(1) follow/unfollow (create/close PDA)
  - O(1) existence check (derive PDA address)
  - Unlimited follows per user
  - Unfollow reclaims rent (~0.001 SOL)

## Related Repos

| Repo | Description |
|------|-------------|
| [tribe-sdk](../tribe-sdk) | TypeScript SDK, network config, protobuf definitions |
| [tribe-cast-server](../tribe-cast-server) | Cast server (message storage + validation) |
| [tribe-indexer](../tribe-indexer) | Event indexer + read API |

## Development

### Prerequisites

- Rust + Cargo (nightly)
- Solana CLI 2.x+
- Anchor CLI 0.30.1+
- Node.js 18+
- pnpm

### Setup

```bash
# Install dependencies
pnpm install

# Build all programs
anchor build

# Run tests (requires local validator)
anchor test

# Deploy to devnet
anchor deploy --provider.cluster devnet

# Deploy to mainnet
anchor deploy --provider.cluster mainnet
```

### Program IDs

Program IDs are placeholders until first deploy. After deploying, update `Anchor.toml` and `declare_id!()` in each program's `lib.rs`.

| Program | Localnet/Devnet | Mainnet |
|---------|----------------|---------|
| fid-registry | TBD | TBD |
| app-key-registry | TBD | TBD |
| username-registry | TBD | TBD |
| social-graph | TBD | TBD |

## Project Structure

```
tribe-protocol/
├── Anchor.toml                 # Anchor workspace config
├── Cargo.toml                  # Rust workspace
├── programs/
│   ├── fid-registry/           # FID identity program
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── state/          # FidRecord, GlobalState, CustodyLookup
│   │       ├── instructions/   # initialize, register, transfer, recover
│   │       ├── errors.rs
│   │       └── events.rs
│   ├── app-key-registry/       # App key delegation program
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── state/          # AppKeyRecord
│   │       ├── instructions/   # add_key, revoke_key, rotate_key
│   │       ├── errors.rs
│   │       └── events.rs
│   ├── username-registry/      # Username program
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── state/          # UsernameRecord, FidUsername
│   │       ├── instructions/   # register, renew, transfer, release
│   │       ├── errors.rs
│   │       └── events.rs
│   └── social-graph/           # Social graph program
│       └── src/
│           ├── lib.rs
│           ├── state/          # SocialProfile, Link
│           ├── instructions/   # init_profile, follow, unfollow
│           ├── errors.rs
│           └── events.rs
├── tests/                      # Anchor integration tests
├── migrations/
└── scripts/
```

## License

MIT
