use ark_bn254::Fq;
use ark_ff::{BigInteger, Field, PrimeField};
use ark_std::rand::Rng;
use sha2::{Sha256, Digest};
use super::{circuit::SumPoly, multilinear_polynomial::MultilinearPoly, transcript::{HashTrait, Transcript}};
const PRIME: u64 = 23; 



fn prover<F : PrimeField, K: HashTrait>(initial_poly:SumPoly<F>, num_vars: usize, hash_function: K) -> (F, Vec<Vec<F>>) {

    let claimed_sum: F = compute_sum(&initial_poly);

    let mut proof = vec![];
    let mut transcript = Transcript::init(hash_function);

    transcript.absorb(&claimed_sum.into_bigint().to_bytes_be());

    transcript.absorb(&to_bytes(&initial_poly.polys));

    let mut poly = initial_poly;
    
    for round in 0..num_vars {
        
        let round_poly = generate_round_poly(&poly);
        
        transcript.absorb(&to_bytes(&round_poly));
        
        proof.push(round_poly);

        let challenge = transcript.squeeze();
        dbg!(&challenge);

       poly = poly.partial_evaluate((0, challenge));


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



  
   fn generate_round_poly<F:PrimeField>(poly: &SumPoly<F>) ->Vec<F>{
    let eval_zero = poly.partial_evaluate(F::zero()).reduce().iter().sum();
    let eval_one = poly.partial_evaluate(F::one()).reduce().iter().sum();
    let eval_two = poly.partial_evaluate(F::from(2_u64)).reduce().iter().sum();

    vec![eval_zero,eval_one, eval_two]
 }

 fn compute_sum<F: PrimeField>(poly: &MultilinearPoly<F>) -> F {
    poly.coefficients.reduce().iter().sum()
 }


fn verifier<F : PrimeField, K: HashTrait>(claimed_sum: F, proof: &Vec<Vec<F>>, num_vars: usize , initial_poly: &SumPoly<F>, hash_function: K) -> bool {
    
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