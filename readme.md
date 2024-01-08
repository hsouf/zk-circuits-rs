# ZK-SNARKS Circuits In Rust

The `bellman` crate is a Rust library for building zk-SNARKs, providing a modular and user-friendly API for constructing and verifying zero-knowledge proofs. It employs circuits as abstract representations of computations, uses R1CS for circuit representation, requires specific parameters for proof generation and verification, and works with pairing-friendly elliptic curves like BLS12-381.

## Overall process
![image](https://github.com/hsouf/zk-circuits-rs/assets/37840702/7fee8215-5549-4f15-b60c-3e5835b56123)


# Implemented circuits

- `SquareRootCircuit`: to generate a proof that we know the square root of a value `x` without disclosing the root value.
