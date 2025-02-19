use ark_bn254::Fq;
use ark_ff::{BigInteger, Field, PrimeField};
use ark_std::rand::Rng;
use sha2::{Sha256, Digest};
use super::{multilinear_polynomial::MultilinearPoly, transcript::{HashTrait, Transcript}};
const PRIME: u64 = 23; 
type Polynomial<F> = MultilinearPoly<F>;



fn prover<F : PrimeField, K: HashTrait>(initial_poly: Polynomial<F>, num_vars: usize, hash_function: K) -> (F, Vec<Vec<F>>) {

    let claimed_sum: F = compute_sum(&initial_poly);

    let mut proof = vec![];
    let mut transcript = Transcript::init(hash_function);

    transcript.absorb(&claimed_sum.into_bigint().to_bytes_be());

    transcript.absorb(&to_bytes(&initial_poly.coefficients));

    let mut poly = initial_poly;
    
    for round in 0..num_vars {
        
        let round_poly = generate_round_poly(&poly);
        
        transcript.absorb(&to_bytes(&round_poly));
        
        proof.push(round_poly);

        let challenge = transcript.squeeze();
        dbg!(&challenge);

       poly = poly.partial_evaluate((0, challenge));

       dbg!(&poly.coefficients);
    }
    
    (claimed_sum, proof)
}






 fn to_bytes<F:PrimeField>(values: &Vec<F>) -> Vec<u8>{
    let mut result = vec![];
    for value in values{
       result.extend(value.into_bigint().to_bytes_be());
    }

    result
 }



 fn generate_round_poly<F:PrimeField>(poly: &Polynomial<F>) ->Vec<F>{
    let eval_zero = poly.partial_evaluate((0,F::zero())).coefficients.iter().sum();
    let eval_one = poly.partial_evaluate((0,F::one())).coefficients.iter().sum();
    vec![eval_zero,eval_one]
 }

 fn compute_sum<F: PrimeField>(poly: &MultilinearPoly<F>) -> F {
    poly.coefficients.iter().sum()
 }


fn verifier<F : PrimeField, K: HashTrait>(claimed_sum: F, proof: &Vec<Vec<F>>, num_vars: usize , initial_poly: &Polynomial<F>, hash_function: K) -> bool {
    
    dbg!("Verifier");

    let mut transcript = Transcript:: init(hash_function);
    transcript.absorb(&claimed_sum.into_bigint().to_bytes_be());
    transcript.absorb(&to_bytes(&initial_poly.coefficients));

    let mut running_claim= claimed_sum;
    let mut challenges =vec![];

    for round_poly in proof {
        if round_poly.len() !=2 {
           return false;
        } 
        assert!(running_claim == round_poly.iter().sum());
        transcript.absorb(&to_bytes(round_poly));

        let challenge = transcript.squeeze();
        challenges.push(challenge);

        dbg!(&challenge);

         running_claim = round_poly[0] + challenge * (round_poly[1] - round_poly[0]);

         dbg!(&running_claim);
        
    }
    let final_eval = initial_poly.evaluate(&challenges);

    dbg!(&final_eval);
    dbg!(&running_claim);

    running_claim == final_eval
}




#[cfg(test)]
mod test {
use crate::zk_project::sumcheck_implementation::{prover, verifier};

// Test the Sumcheck protocol
use super::MultilinearPoly;
use ark_bn254::Fq;
use sha3::{Keccak256, Digest};

// Define a simple multilinear polynomial: f(x1, x2) = 2x1 + 3x2
fn create_test_polynomial() -> MultilinearPoly<Fq> {
    let coefficients = vec![
        Fq::from(0),  // f(0, 0) = 0
        Fq::from(3),  // f(0, 1) = 3
        Fq::from(2),  // f(1, 0) = 2
        Fq::from(5),  // f(1, 1) = 5
    ];
    MultilinearPoly::new(coefficients)
}

#[test]
fn test_sumcheck_protocol() {
    // Create a test polynomial
    let poly = create_test_polynomial();

    // Number of variables in the polynomial
    let num_vars = 2;

    // Initialize the hash function for the transcript
    let hash_function = Keccak256::new();

    // Run the prover
    let (claimed_sum, proof) = prover(poly.clone(), num_vars, hash_function.clone());

    dbg!(&proof);
    dbg!(&claimed_sum);

    // Verify the proof
    let is_valid = verifier(claimed_sum, &proof, num_vars, &poly, hash_function);

    // Check that the proof is valid
    assert!(is_valid, "Sumcheck protocol verification failed");

    // Print the results
    println!("Claimed sum: {}", claimed_sum);
    println!("Proof: {:?}", proof);
    println!("Verification result: {}", is_valid);
}

}


