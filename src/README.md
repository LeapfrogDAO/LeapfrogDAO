# LeapfrogDAO Token (LFT)

<div align="center">
  <img src="https://via.placeholder.com/200x200.png?text=LeapfrogDAO" alt="LeapfrogDAO Logo" width="200" height="200">
  <h3>Building a Decentralized World of Freedom, Justice, and Empowerment</h3>
  
  [![Solana](https://img.shields.io/badge/Solana-Blockchain-14F195?style=for-the-badge&logo=solana&logoColor=white)](https://solana.com/)
  [![TypeScript](https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
  [![License](https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge)](LICENSE)
  
</div>

## Overview

The LeapfrogDAO Token (LFT) is the governance and utility token powering the LeapfrogDAO ecosystem—a decentralized autonomous organization committed to dismantling oppressive systems and replacing them with technologies that empower individuals and communities.

Built on Solana for unparalleled speed, security, and low transaction costs, LFT enables a comprehensive governance system with quadratic voting, stake-based participation, and equitable resource allocation. The token serves as the backbone for our mission to create a digital world defined by freedom, justice, and equity.

## Key Features

### 🔐 Advanced Governance
- Quadratic voting system that prevents wealth concentration
- Proposal creation and management with flexible voting types
- Time-locked execution for transparent governance

### 🌐 Equitable Distribution
- 40% allocated to community treasury
- 25% for ecosystem contributors
- 20% for development fund
- 10% for founding team (time-locked)
- 5% for initial liquidity provision

### ⚙️ Technical Implementation
- SPL token on Solana
- 9 decimal precision
- 1 billion total supply
- Time-locked allocations using on-chain programs
- Integration with EquiVote governance system

### 💡 Use Cases
- Create and vote on governance proposals
- Stake to earn rewards and governance rights
- Access premium services within LeapfrogDAO ecosystem
- Fund community-driven initiatives

## Token Distribution

The initial token distribution embodies our commitment to community ownership and long-term sustainability:

```
┌────────────────────┬────────┬────────────────┐
│ Allocation         │ %      │ Amount (LFT)   │
├────────────────────┼────────┼────────────────┤
│ Community Treasury │ 40%    │ 400,000,000    │
│ Contributor Rewards│ 25%    │ 250,000,000    │
│ Development Fund   │ 20%    │ 200,000,000    │
│ Founding Team      │ 10%    │ 100,000,000    │
│ Liquidity Provision│ 5%     │ 50,000,000     │
└────────────────────┴────────┴────────────────┘
```

## Getting Started

### Prerequisites
- [Node.js](https://nodejs.org/) (v16 or higher)
- [Solana CLI Tools](https://docs.solana.com/cli/install-solana-cli-tools)
- Solana wallet with SOL for deployment

### Installation

```bash
# Clone the repository
git clone https://github.com/leapfrogdao/leapfrog-token.git
cd leapfrog-token

# Install dependencies
npm install
```

### Deployment

```bash
# Create a new keypair (if you don't have one)
solana-keygen new -o deployer-keypair.json

# Fund your wallet
solana airdrop 2 $(solana-keygen pubkey deployer-keypair.json) --url devnet

# Deploy the token
npm run deploy:devnet  # For testnet deployment
npm run deploy:mainnet # For mainnet deployment
```

## Development

```bash
# Build the project
npm run build

# Run tests
npm test

# Lint code
npm run lint
```

## Governance Implementation

The LeapfrogDAO token is deeply integrated with our governance program, providing:

- **Stake-based Voting**: Token holders can stake LFT to participate in governance
- **Quadratic Influence**: Voting power scales with the square root of tokens staked
- **Proposal Creation**: Create proposals for resource allocation and ecosystem decisions
- **Delegation**: Delegate voting power to trusted community members

## Security Features

- **Time-locking**: Founding team tokens are locked for 12 months with linear vesting
- **Multi-signature**: Treasury requires multiple signers for any fund allocation
- **Cooldown Periods**: Staking and unstaking include cooldown periods to prevent attacks
- **Audited Code**: Professional security audits by leading blockchain security firms

## Token Contract

**Mainnet:** `LFTxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx`  
**Decimals:** 9  
**Explorer:** [Solana Explorer](https://explorer.solana.com/address/LFTxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx)

## Roadmap

| Phase | Timeline | Milestone |
|-------|----------|-----------|
| Alpha | Q1 2025 | Token deployment and initial governance |
| Beta | Q2 2025 | Full governance features and ecosystem integration |
| V1.0 | Q3 2025 | Complete DAO functionality with expanded use cases |
| V2.0 | Q1 2026 | Enhanced governance with reputation systems |

## Contributing

We welcome contributions to the LeapfrogDAO ecosystem! Please see our [Contributing Guidelines](CONTRIBUTING.md) for more informatio

## Connect With Us

- [Twitter](https://twitter.com/leapfrog_89
- [GitHub](https://github.com/leapfrogdao)

---

<div align="center">
  <h3>LeapfrogDAO: Where Code Meets Courage</h3>
  <p>Building a future where technology serves humanity, not the powerful.</p>
</div>
