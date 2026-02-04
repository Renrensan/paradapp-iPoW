# Paradapp Operator Service

A high-performance, modular blockchain operator built in Rust. This service facilitates cross-chain bridge intents by providing automated streaming, approving, and converting services across networks by leveraging Bitcoin networks.

## Architecture Overview

The system is built on a **Trait-Based Adapter Architecture**, separating the core business logic from chain-specific implementations.

- **Core**: Defines the state machine and cross-chain traits.
- **Chains**: Concrete implementations for EVM (Ethereum, Hedera, etc.), Solana, or other supported chains.
- **Operator**: The orchestration layer that manages transaction lifecycles.

For a deep dive into the design, see [ARCHITECTURE.md](./docs/ARCHITECTURE.md).

## Quick Start

### Prerequisites

- **Rust** (Latest Stable)
- **mise** (Task runner)
- **Docker** (For local infrastructure)

### Installation

```sh
# Clone and enter repo
git clone <repo-url>
cd paradapp-operator-service

# Setup local data directory for SQLite
mkdir data

# Install toolchain and dependencies via mise
mise install

# Configure environment
cp .env.example .env
```

## Local Development

### Running the Operator

The service is managed via `mise` tasks. You can run specific network operators or all-in-one nodes.

```sh
# View all available tasks
mise tasks

# Run Hedera operator watching Ethereum intents
mise run run-hedera eth

# Run with a specific engine (e.g., only the Streamer)
ENGINE=streamer mise run run-ethereum
```

### Running Tests

```sh
# Run all workspace tests
cargo test

# Run specific crate tests
cargo test -p paradapp-core
```

## Deployment

### Docker Build

The service is containerized for consistent deployment across environments.

```sh
# Build the production image
docker build -t paradapp-operator .
```

### Production Configuration

Production environments should leverage the `ENGINE` environment variable to scale components independently:

## Project Structure

```text
.
├── crates/
│   ├── core/         # Logic, Traits, and Models
│   ├── chains/       # EVM, Solana, and other networks adapters
│   └── operator/     # Main entrypoint and task spawner
├── data/             # Local SQLite storage (Git ignored)
├── docs/             # Technical documentation
└── mise.toml         # Task definitions
```

## Documentation

- [Getting Started Guide](./docs/GETTING_STARTED.md) - Detailed setup and usage.
- [Architecture & Design](./docs/ARCHITECTURE.md) - In-depth system design and data flow.

---

### Next Step

Would you like me to generate a **`CONTRIBUTING.md`** file that outlines the process for adding support for a new blockchain network?
