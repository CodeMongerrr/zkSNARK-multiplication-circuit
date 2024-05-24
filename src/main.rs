#![allow(unused_imports)]
#![allow(unused_variables)]
extern crate bellman;
extern crate pairing;
extern crate rand;

use bellman::{Circuit, ConstraintSystem, SynthesisError};
use pairing::{Engine, Field, PrimeField};

mod simple_circuit;

fn main() {
    use pairing::bls12_381::{Bls12, Fr};
    use rand::thread_rng;
    use bellman::groth16::{
        create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof,
        Proof,
    };

    println!("Proving a simple circuit...");
    let rng = &mut thread_rng();

    println!("Creating parameters...");
    let params = {
        let c = simple_circuit::SimpleCircuit::<Bls12> {
            a: None,
            b: None,
            c: None,
        };
        generate_random_parameters(c, rng).unwrap()
    };

    let pvk = prepare_verifying_key(&params.vk);

    println!("Creating proofs...");
    let a = Fr::from_str("3").unwrap();
    let b = Fr::from_str("5").unwrap();
    let c = Fr::from_str("15").unwrap();

    let c_proof = simple_circuit::SimpleCircuit::<Bls12> {
        a: Some(a),
        b: Some(b),
        c: Some(c),
    };

    let proof = create_random_proof(c_proof, &params, rng).unwrap();

    let c_verify = simple_circuit::SimpleCircuit::<Bls12> {
        a: Some(a),
        b: Some(b),
        c: Some(c),
    };

    println!("Verifying proof...");
    assert!(verify_proof(&pvk, &proof, &[c_verify.c.unwrap()]).unwrap());
    println!("Proof verified successfully!");
}