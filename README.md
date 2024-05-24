# zk-SNARK Simple Multiplication

This repository contains an implementation of a simple zero-knowledge Succinct Non-interactive ARgument of Knowledge (zk-SNARK) circuit for proving the knowledge of values `a` and `b` such that `a * b = c`, where `c` is a public input.

## Description

The circuit is implemented using the [bellman](https://github.com/zkcrypto/bellman) library, which is a zk-SNARK library for Rust. The circuit enforces the constraint `a * b = c`, where `a` and `b` are private inputs, and `c` is a public input.

The repository includes the following files:

- `simple_circuit.rs`: Contains the implementation of the `SimpleCircuit` struct, which defines the circuit constraints.
- `main.rs`: Contains the code for generating circuit parameters, creating and verifying the proof.

## Usage

To run the code, follow these steps:

1. Clone the repository:

```
https://github.com/CodeMongerrr/zkSNARK-multiplication-circuit.git
```

2. Navigate to the project directory:

```
cd zkSNARK-multiplication-circuit
```

3. Build and run the code:

```
cargo run
```

This will generate the circuit parameters, create a proof for the given values of `a`, `b`, and `c`, and then verify the proof.

## Example Output

```
Proving a simple circuit...
Creating parameters...
Creating proofs...
Verifying proof...
Proof verified successfully!
```
## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.


