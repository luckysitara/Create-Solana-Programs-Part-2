#Prgram Examples
---

# Program Examples

## This repo contains Solana onchain programs (referred to as 'Smart Contracts' in other blockchains) for the [Superteam Bounty - Create Solana Programs](https://earn.superteam.fun/listings/bounty/create-solana-programs-2/).


 If you're new to Solana, you don't need to create your own programs to perform basic tasks like account creation, token transactions, or NFT minting. These common actions are handled by existing programs, like the System Program (for creating accounts or transferring SOL) or the Token Program (for creating tokens and NFTs). For more information, see the [Solana Developer site](https://solana.com/developers).

Each folder includes examples for one or more of the following frameworks:

- **`anchor`**: Uses [Anchor](https://www.anchor-lang.com/), a widely-used framework for Solana development with Rust. Build & deploy with `anchor build && anchor deploy`. Test with `anchor run test`.
- **`native`**: Built with Solana's native Rust crates. Use `cicd.sh` to build & deploy, and `yarn run test` to test.
- **`poseidon`**: Written with [Poseidon](https://turbin3.github.io/poseidon), converting TypeScript to Anchor Rust.
- **`seahorse`**: Uses [Seahorse](https://seahorse-lang.org/), converting Python code to Anchor Rust. Build & deploy with `seahorse build && anchor deploy`. Test with `anchor run test`.

**Want to contribute?** If an example is missing, send us a PR! Our goal is to provide each example across all frameworks. We also welcome examples on staking, wrapped tokens, oracles, compression, and VRF. See our [contributing guidelines](./CONTRIBUTING.md) to ensure consistency.

---

## The example programs

<details>
  <summary>Basics</summary>

### Hello World

[Minimal "Hello World" program](./basics/hello-solana/README.md)

Completed as part of the bounty.

Available in: [anchor](./basics/hello-solana/anchor) | [native](./basics/hello-solana/native) | [seahorse](./basics/hello-solana/seahorse)

### Favorites

Save and update per-user state on the blockchain, ensuring users can only update their own information.

Completed as part of the bounty.

Available in: [anchor](./basics/favorites/anchor)

... (continue similar structure for each section)

</details>

<details>
  <summary>Tokens</summary>

### Escrow

Allow two users to swap digital assets with each other, each getting 100% of what the other has offered due to the power of decentralization!

Completed as part of the bounty.

Available in: [anchor](./tokens/escrow/anchor)

</details>

---

Thanks
