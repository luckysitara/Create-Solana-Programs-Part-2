
---

# Hello Solana Steel

**Hello Solana Steel** is a minimal example of a Solana program that logs "Hello, Solana from Steel!" when executed. This example demonstrates how to set up a Solana program using the **Steel** framework, with testing support via the **Bankrun** framework and **@solana/web3.js**.

## Features

- **Hello, Solana!**: Logs a message when the program is invoked.
- **Steel Framework**: For streamlined Solana program development.
- **Bankrun Framework**: For efficient testing of Solana programs in a simulated environment.
- **TypeScript Tests**: Interaction testing through **@solana/web3.js**.

## Table of Contents

1. [File Structure](#file-structure)
2. [Prerequisites](#prerequisites)
3. [Installation](#installation)
4. [Building the Program](#building-the-program)
5. [Running Tests](#running-tests)
6. [File Explanations](#file-explanations)
7. [Additional Resources](#additional-resources)

---

## File Structure

```plaintext
hello-solana-steel/
├── program/
│   ├── Cargo.toml             # Rust dependencies for program
│   └── src/
│       └── lib.rs             # Main program logic
├── tests/
│   ├── test.rs                # Rust-based integration tests using Bankrun
│   └── test.ts                # TypeScript-based tests using @solana/web3.js
├── Cargo.toml                 # Workspace configuration (optional)
├── package.json               # Node.js dependencies for TypeScript tests
├── pnpm-lock.yaml             # PNPM lock file (optional)
└── tsconfig.json              # TypeScript configuration
```

---

## Prerequisites

- **Rust**: Ensure Rust and its toolchain are installed. Follow [Rust installation instructions](https://rustup.rs/).
- **Solana CLI**: Install Solana CLI tools by following [Solana CLI installation](https://docs.solana.com/cli/install-solana-cli-tools).
- **Node.js & PNPM**: Install Node.js and [PNPM](https://pnpm.io/installation) for TypeScript-based testing.

## Installation

1. **Clone the Repository**:
   ```bash
   git clone <github.com/luckysitara/Create-Solana-Programs-Part-2>
   cd Create-Solana-Programs-Part-2/hello-solana-steel
   ```

2. **Install Node.js Dependencies** (if using TypeScript tests):
   ```bash
   pnpm install
   ```

3. **Build Rust Program**:
   Navigate to the program folder and build it using the Solana toolchain.
   ```bash
   cd program
   cargo build-bpf
   ```

---

## Building the Program

To compile the Solana program to BPF (Berkeley Packet Filter) format:

```bash
cd program
cargo build-bpf
```

This will output a `.so` file in `target/deploy`, which can be deployed to a Solana cluster.

---

## Running Tests

### Rust-Based Tests

1. **Run Rust Unit Tests**:
   Unit tests for the Hello Solana program are in `test.rs`, using the **Bankrun** framework to simulate a Solana environment.

   ```bash
   cargo test -- --nocapture
   ```

2. **Run Rust Integration Tests**:
   The integration test in `test.rs` simulates a transaction to invoke the Hello Solana program and checks for a successful response.

   ```bash
   cargo test --test test -- --nocapture
   ```

### TypeScript-Based Tests (optional)

The TypeScript tests in `test.ts` allow you to interact with the program using **@solana/web3.js**.

1. **Run TypeScript Tests**:
   ```bash
   pnpm test
   ```

   This test invokes the Hello Solana program, sending a transaction to the local Solana environment and confirming the result.

---

## File Explanations

### 1. `program/Cargo.toml`

Defines dependencies for the Solana program, including **Steel** and **Bankrun**.

```toml
[package]
name = "hello-solana-steel"
version = "0.1.0"
edition = "2021"

[dependencies]
solana-program = "1.14.0"
steel = { git = "https://github.com/steel-programming/steel.git" }

[dev-dependencies]
solana-program-test = "1.14.0"
solana-sdk = "1.14.0"
bankrun = "0.2.0"
```

### 2. `program/src/lib.rs`

This file contains the main program logic. When invoked, it logs "Hello, Solana from Steel!" to the console.

```rust
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult,
    pubkey::Pubkey, msg,
};
use steel::{SteelProcessor, ProgramContext};

pub struct HelloSolana;

impl SteelProcessor for HelloSolana {
    fn process_instruction(ctx: ProgramContext) -> ProgramResult {
        msg!("Hello, Solana from Steel!");
        Ok(())
    }
}

entrypoint!(HelloSolana::process_instruction);
```

### 3. `tests/test.rs`

This Rust test file uses **Bankrun** to simulate a Solana environment and test the program.

```rust
use solana_sdk::{pubkey::Pubkey, transaction::Transaction};
use bankrun::{Bankrun, ProgramTestContext};
use hello_solana_steel::HelloSolana;

#[tokio::test]
async fn test_hello_solana_with_bankrun() {
    let program_id = Pubkey::new_unique();

    let mut context = Bankrun::new(
        "hello_solana_steel",
        program_id,
        HelloSolana::process_instruction
    ).start().await;

    let payer = context.payer.clone();
    let recent_blockhash = context.last_blockhash;

    let mut transaction = Transaction::new_with_payer(
        &[solana_sdk::instruction::Instruction {
            program_id,
            accounts: vec![],
            data: vec![],
        }],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);

    context.process_transaction(transaction).await.unwrap();
}
```

### 4. `tests/test.ts`

This TypeScript file provides a test case using **@solana/web3.js** to interact with the Hello Solana program.

```typescript
import { Connection, Keypair, PublicKey, SystemProgram, Transaction } from '@solana/web3.js';

async function main() {
    const connection = new Connection("http://localhost:8899", 'confirmed');
    const payer = Keypair.generate();

    const programId = new PublicKey("YOUR_PROGRAM_ID_HERE");

    const transaction = new Transaction().add({
        keys: [],
        programId,
        data: Buffer.alloc(0),
    });

    const signature = await connection.sendTransaction(transaction, [payer]);
    await connection.confirmTransaction(signature, 'confirmed');

    console.log("Transaction confirmed with signature:", signature);
}

main().catch(console.error);
```

### 5. `package.json`

Defines dependencies for Node.js and TypeScript-based tests.

```json
{
  "name": "hello-solana-steel",
  "version": "1.0.0",
  "scripts": {
    "test": "ts-node tests/test.ts"
  },
  "dependencies": {
    "@solana/web3.js": "^1.41.0"
  },
  "devDependencies": {
    "ts-node": "^10.0.0"
  }
}
```

### 6. `tsconfig.json`

Configures TypeScript for compatibility with **@solana/web3.js**.

```json
{
  "compilerOptions": {
    "target": "es2020",
    "module": "commonjs",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true
  },
  "include": ["tests/**/*.ts"]
}
```

---

## Additional Resources

- [Solana Documentation](https://docs.solana.com/)
- [Steel GitHub Repository](https://github.com/steel-programming/steel)
- [Bankrun GitHub Repository](https://github.com/project-serum/bankrun)

---

