# ZK-SNARKS Circuits in Rust

The `bellman` crate is a Rust library for building zk-SNARKs, providing a modular and user-friendly API for constructing and verifying zero-knowledge proofs. It employs circuits as abstract representations of computations, uses R1CS for circuit representation, requires specific parameters for proof generation and verification, and works with pairing-friendly elliptic curves like BLS12-381.

In this repository, we will create a simple SquareRootCircuit to generate a proof that we know the square root of a value without disclosing the root value.
