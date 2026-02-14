# Getting Started

## Prerequisites

Ensure you have the following installed:

- **Rust (latest stable)** (Required for building the operator)
- **mise** (Required for task management and environment handling)
- **Docker & Docker Compose** (For containerized deployment)

## Installation

1.  **Clone the repository:**

    ```sh
    git clone <your-repo-url>
    cd paradapp-operator-service
    ```

2.  **Configure Environment:**

    The application uses environment variables for configuration. Copy the example file:

    ```sh
    cp .env.example .env
    ```

    Modify `.env` to set your RPC endpoints, private keys, etc

3.  **Install Tools:**

    Install the project tools managed by `mise`:

    ```sh
    mise install
    ```

## Running Locally

The operator is managed via `mise` tasks. You can view all currently defined tasks and their usage formats by running:

```sh
# View all available tasks
mise tasks
```

The general usage format for operator tasks is:
`[ENGINE=<type>] mise run <task-name> [watch_sources...]`

### 1. Run Hedera Operator

Run the Hedera operator and optionally watch other chains for bridge intents.

```sh
# Run with all engines, no watch sources
mise run run-hedera

# Run and watch Ethereum (enable hedera to listen to ethereum)
mise run run-hedera eth
```

### 2. Run Ethereum Operator

Run the Ethereum operator and optionally watch other chains.

```sh
# Run and watch Hedera (enable ethereum to listen to hedera)
mise run run-ethereum hedera
```

### 3. Run Custom or Future Networks

Use this task to run any network supported by the codebase that doesn't have a specific shortcut task yet.

```sh
# Format: mise run run-custom <src_network> [watch_sources...]
mise run run-custom eth hedera polygon
```

## Configuration (Engines)

You can specify which logic to run using the `ENGINE` environment variable.

| Engine        | Description                                             |
| :------------ | :------------------------------------------------------ |
| **all**       | (Default) Runs Approver, Streamer, and Converter logic. |
| **approver**  | Handles transaction approving and open tunnel.          |
| **streamer**  | Handle stream blocks to contract.                       |
| **converter** | Executes the final conversion transactions.             |

**Example:**

```sh
ENGINE=streamer mise run run-ethereum
```

---

### Troubleshooting

**Unexpected Argument Error:**
If you see `error: unexpected argument '...' found`, ensure you are using the `mise` task correctly. These tasks automatically handle the `--watch-sources` flag for you.

**Shell Syntax Error:**
If you encounter `sh: Syntax error`, ensure your environment supports `bash`. The tasks use `bash -c` to ensure arguments are passed into the Rust binary correctly.
