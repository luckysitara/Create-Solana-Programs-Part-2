
---

# Program Examples


## This repository contains Solana onchain programs (referred to as 'Smart Contracts' in other blockchains) completed as part of the [Superteam Bounty - Create Solana Programs](https://earn.superteam.fun/listings/bounty/create-solana-programs-2/).

> [!NOTE]
> For beginners: You don’t need custom programs for basics like creating accounts, transferring tokens, or minting NFTs. These actions are handled by existing Solana programs, such as the System Program or Token Program. Visit the [Solana Developer site](https://solana.com/developers) for more information.

Each folder includes examples using one or more of the following frameworks:

- **`anchor`**: Uses [Anchor](https://www.anchor-lang.com/) with Rust. Build & deploy with `anchor build && anchor deploy`. Test with `anchor run test`.
- **`native`**: Built with Solana's native Rust crates. Use `cicd.sh` to build & deploy; test with `yarn run test`.
- **`steel`**: A framework for streamlined Rust-based development.
- **`seahorse`**: Uses [Seahorse](https://seahorse-lang.org/) for Python. Build & deploy with `seahorse build && anchor deploy`. Test with `anchor run test`.

Contributions are welcome! If you’d like to add an example, please follow the [contributing guidelines](./CONTRIBUTING.md).

---

## Example Programs

<details>
  <summary>Basics</summary>

### Hello World

A minimal "Hello World" program for Solana. This program logs a simple greeting to the blockchain.

Completed as part of the bounty.

Available in: [steel](./hello-solana-steel)

### Favorites

A program to save and update per-user state, ensuring users can only update their own information.

Completed as part of the bounty.

Available in: [steel](./favorite-steel)

</details>

<details>
  <summary>Tokens</summary>

### Escrow

A program allowing two users to swap digital assets, ensuring each user receives the full value of the other’s offer through a decentralized transaction.

Completed as part of the bounty.

Available in: [anchor](./escrow-anchor) | [native](./escrow-native) | [steel](./escrow-steel)

</details>

---

