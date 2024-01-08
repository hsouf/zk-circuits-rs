mod circuits;

use bellman::groth16::{
    create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof,
};
use bls12_381::{Bls12, Scalar};
use circuits::sqrt::SquareRootCircuit;
use rand::rngs::OsRng;

fn main() {
    let rng = &mut OsRng;
    // Generate parameters based on our circuit
    let params = {
        let empty_circuit = SquareRootCircuit::<Scalar> {
            x: None,
            root: None,
        };
        generate_random_parameters::<Bls12, _, _>(empty_circuit, rng)
            .expect("Parameter generation failed")
    };

    // proof generation
    let prover_circuit = SquareRootCircuit {
        x: Some(Scalar::from(81u64)),
        root: Some(Scalar::from(9u64)),
    };

    let proof = create_random_proof(prover_circuit, &params, rng).expect("Proof generation failed");

    //proof verification
    let pvk = prepare_verifying_key(&params.vk);

    let public_input = vec![Scalar::from(81u64)]; // defining the public input (our x value)

    verify_proof(&pvk, &proof, &public_input).expect("Proof verification failed");
}
