# LeapfrogDAO Governance Program

  <h3>Decentralized Governance for a More Equitable Digital World</h3>
  
  [![Solana](https://img.shields.io/badge/Solana-Blockchain-14F195?style=for-the-badge&logo=solana&logoColor=white)](https://solana.com/)
  [![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
  [![License](https://img.shields.io/badge/License-apache-blue.svg?style=for-the-badge)](LICENSE)
  
</div>

## Overview

The LeapfrogDAO Governance Program is an advanced on-chain governance system built on Solana that brings to life the democratic vision outlined in the LeapfrogDAO whitepaper. This program enables community-driven decision making through a sophisticated suite of voting mechanisms, proposal management, and resource allocation tools.

Designed with inclusivity at its core, our governance program implements quadratic voting, stake-based participation, and timelock mechanisms to ensure equitable representation regardless of token holdings. The system actively prevents plutocracy while maintaining security against governance attacks.

## Core Features

### 🏛️ Decentralized Governance Architecture
- **Realms**: Create isolated governance instances with customizable parameters
- **Proposals**: Multi-stage proposal lifecycle with transparent tracking
- **Instructions Execution**: On-chain execution of approved proposals
- **Council Support**: Optional council structure for specialized decisions

### 🗳️ Advanced Voting Mechanisms
- **Quadratic Voting**: Voting power scales with the square root of staked tokens
- **Multi-Option Voting**: Support for single choice, multiple choice, and weighted voting
- **Vote Types**: Approval, rejection, and abstention with configurable thresholds
- **Denial Quorum**: Protection against low-participation governance attacks

### 🔒 Security & Fairness Systems
- **Staking**: Token-staking requirements for proposal creation and voting
- **Timelocks**: Cooling periods on critical governance actions
- **Vote Weights**: Configurable vote weight calculations to prevent wealth concentration
- **Transparent Execution**: Fully on-chain execution with audit trails

### 📊 Governance Analytics
- **Participation Metrics**: Track voter participation across proposals
- **Vote Distribution**: Analyze voting patterns and community alignment
- **Proposal Success Rate**: Measure governance effectiveness over time
- **Token Distribution**: Monitor stake distribution for centralization risks

## Technical Architecture

The LeapfrogDAO Governance Program is built as a native Solana program that directly interacts with the LeapfrogDAO Token (LFT). The system consists of several key account types that together form a complete governance ecosystem:

```
┌──────────────┐       ┌───────────────┐       ┌─────────────┐
│    Realm     │─┬────▶│   Proposal    │◀─────▶│ Vote Record │
└──────────────┘ │     └───────────────┘       └─────────────┘
                 │             ▲
                 │             │
                 │     ┌───────────────┐       ┌─────────────┐
                 └────▶│ Token Owner   │◀─────▶│   Staking   │
                       │    Record     │       │    Vault    │
                       └───────────────┘       └─────────────┘
```

### Account Types

- **Realm**: The top-level governance container
- **Proposal**: Individual governance proposals with execution instructions
- **Token Owner Record**: Tracks token deposits and voting rights
- **Vote Record**: Records individual votes on proposals
- **Staking Vault**: Securely holds staked tokens

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (1.65+)
- [Solana CLI Tools](https://docs.solana.com/cli/install-solana-cli-tools) (1.14+)
- [Anchor](https://project-serum.github.io/anchor/getting-started/installation.html) (optional, for development)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/leapfrogdao/governance-program.git
cd governance-program

# Build the program
cargo build-bpf

# Run tests
cargo test-bpf
```

### Deployment

```bash
# Generate a program keypair if needed
solana-keygen new -o program-keypair.json

# Deploy to devnet
solana program deploy \
  --program-id program-keypair.json \
  --keypair deployer-keypair.json \
  --url devnet \
  target/deploy/leapfrog_governance.so
```

## Using the Governance Program

### Creating a Realm

```rust
// Create a new governance realm
let create_realm_ix = LeapfrogInstruction::InitializeRealm {
    name: "LeapfrogDAO Main Governance".to_string(),
    min_community_tokens_to_create_proposal: 1000 * 10^9, // 1000 LFT
    community_mint_max_vote_weight_source: MintMaxVoteWeightSource::SupplyFraction { fraction: 100_000_000 },
    use_quadratic_voting: true,
};
```

### Creating a Proposal

```rust
// Create a new proposal
let create_proposal_ix = LeapfrogInstruction::CreateProposal {
    name: "Community Treasury Allocation Q2 2025".to_string(),
    description_link: "ipfs://QmHashLinkToProposalDescription",
    vote_type: VoteType::SingleChoice,
    options: vec!["Approve".to_string(), "Reject".to_string()],
    use_denial_quorum: true,
    voting_period_days: 7,
};
```

### Casting a Vote

```rust
// Cast a vote on a proposal
let cast_vote_ix = LeapfrogInstruction::CastVote {
    vote: Vote::SingleChoice { option_index: 0 }, // Vote for "Approve"
    staked_amount: 100 * 10^9, // 100 LFT
};
```

## Governance Parameters

The LeapfrogDAO Governance Program offers flexible configuration to meet community needs:

| Parameter | Description | Default Value |
|-----------|-------------|---------------|
| Proposal Threshold | Min tokens to create proposal | 1,000 LFT |
| Voting Period | Duration proposals remain active | 7 days |
| Execution Delay | Time between approval and execution | 2 days |
| Quadratic Voting | Whether to use quadratic voting | Enabled |
| Cooldown Period | Time between unstaking cycles | 7 days |

## Security Considerations

The LeapfrogDAO Governance Program incorporates multiple security measures:

- **Formal Verification**: Core voting logic has undergone formal verification
- **Rate Limiting**: Proposal creation is rate-limited to prevent spam
- **Secure RPC**: Protected RPC endpoints for governance transactions
- **Partial Unstaking**: Unstaking requires a cooldown period
- **Timelocks**: Critical operations require timelock periods

## Governance Client

We provide a TypeScript client for interacting with the governance program:

```typescript
import { Connection, PublicKey } from '@solana/web3.js';
import { LeapfrogGovernanceClient } from '@leapfrogdao/governance-client';

// Initialize the client
const connection = new Connection('https://api.mainnet-beta.solana.com');
const programId = new PublicKey('LeapFD1stribuTedgoverNanceTokenXXXXXXXXX');
const governanceClient = new LeapfrogGovernanceClient(connection, programId);

// Get all proposals in a realm
const realmPubkey = new PublicKey('...');
const proposals = await governanceClient.getProposalsForRealm(realmPubkey);
```

## Roadmap

| Phase | Timeline | Features |
|-------|----------|----------|
| V1 | Q1 2025 | Basic governance with quadratic voting |
| V2 | Q2 2025 | Enhanced security features and council support |
| V3 | Q3 2025 | Integration with zero-knowledge proofs for private voting |
| V4 | Q1 2026 | Advanced reputation systems and conviction voting |

## Contributing

We welcome contributions to improve the LeapfrogDAO Governance Program! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Community and Support

- [Discord](https://discord.gg/leapfrog_89
- [GitHub](https://github.com/leapfrogdao)

## Acknowledgments

- Inspired by Solana Program Library (SPL) Governance
- Built with love by the LeapfrogDAO community
- Project 89 for the vibes
- Special thanks to all contributors and early governance participants

---

<div align="center">
  <h3>LeapfrogDAO: Where Code Meets Courage</h3>
  <p>Reclaiming the digital commons through decentralized governance</p>
</div>
