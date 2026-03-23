# Architecture & Design

This project follows a **Trait-Based Adapter Architecture**. The system is split into independent crates to enforce a strict "Core vs. Implementation" separation, allowing the operator to support multiple blockchain families (EVM, Solana, BTC) with shared business logic.

## Crate Structure

The workspace is divided into three primary layers:

### 1. Core (`crates/core`)

The "Brain" of the system. It defines the rules, data types, and interfaces that all other crates must follow.

- **Traits**: Defines `ChainStack`, `StreamingAdapter`, `ApprovingAdapter`, and `ConvertingAdapter`.
- **BTC Service**: Specialized logic for Bitcoin-specific operations and relaying which is one of the core of the app.
- **Models**: Defines the `Conversion` entity, `TransactionPhase`, and `TransactionType`.

### 2. Chains (`crates/chains`)

The "Execution" layer. This contains the concrete implementations of the core traits for specific networks.

- **EVM**: A comprehensive implementation for Ethereum-like chains (Ethereum, Hedera, etc.).
  - **Adapters**: Concrete logic for state-based approving and converting.
  - **Dependencies**: Network-specific configurations.

### 3. Operator (`crates/operator`)

The "Orchestrator." This brings the adapters together into a running service.

- **Registry**: Maps network names to their specific `ChainStack`.
- **ChainOperator**: Manages the lifecycle of a transaction from spawn to settlement.

---

## Transaction Lifecycle

The operator moves transactions through a state machine to ensure security and cross-chain consistency:

1.  **Spawn**: A transaction is initiated straight to the corresponding smart contract.
2.  **Streaming (BTC Specific)**: For BTC-related flows, the `StreamingAdapter` syncs block headers or specific data to the target contract to provide the necessary context for verification.
3.  **Approving**: The `ApprovingAdapter` runs a series of security and business-logic checks. If the transaction passes, it is signed/authorized.
4.  **Converting**: The `ConvertingAdapter` picks up the approved intent and executes the final movement of funds or state change on the destination network.

## Scalability & Extensibility

- **Pluggable Chains**: To support a new network (e.g., adding `crates/chains/polkadot`), one simply implements the adapters defined in `core/traits`.
- **Engine Decoupling**: By setting the `ENGINE` environment variable, an operator instance can be dedicated solely to **Streaming**  or **Converting** or any other engine that might be later added.
