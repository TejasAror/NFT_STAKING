# NFT Staking Smart Contract (Solana + Anchor)

This repository contains a simple NFT Staking smart contract built using the Solana Anchor framework (Rust).

The contract demonstrates how NFT staking logic can be represented using on-chain program state, focusing on clarity, correctness and Anchor fundamentals.



## Overview

The NFT staking program allows users to:
- Stake an NFT by recording ownership information on-chain
- Unstake the NFT after ownership verification

This implementation is state based and does not integrate with SPL Token or Metaplex standards, as it is intended for learning and assignment purposes.


## Features

- Stake an NFT by storing owner address and token ID
- Verify ownership during unstaking
- Simple and secure state transitions
- Custom error handling


## Instructions

### stake
Creates a staking account that stores:
- NFT owner public key
- NFT token ID

### unstake
Allows unstaking only if the caller is the original staker.
Fails if the caller is not the owner.


## Error Handling

- NotOwner 
  Thrown when an unauthorized user attempts to unstake an NFT.



## Build Instructions

To build the program locally:

```bash
   anchor build
