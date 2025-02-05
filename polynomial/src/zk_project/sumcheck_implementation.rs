use ark_bn254::Fq;
use ark_ff::{BigInteger, Field, PrimeField};
use ark_std::rand::Rng;
use sha2::{Sha256, Digest};
use ark_crypto_primitives::sponge::{CryptographicSponge, FieldBasedCryptographicSponge};

// type F = Fr;
const PRIME: u64 = 23; 
type Polynomial<F> = Vec<F>;

fn prover<F : PrimeField>(f: &Polynomial<F>, num_vars: usize) -> (F, Vec<Vec<F>>) {
    let h: F = compute_sum(f); // Step 1: Compute the sum H
    //change the proof to round proof
    // I need to add the initia poly(f) and the total sum(h) to the transcript In both the verifier and prover
    let mut proof = vec![Vec::new()];
    let mut challenges = Vec::new();
    
    for round in 0..num_vars {
        let g_i = compute_partial_sum(f, &challenges, round);
        proof.push(g_i);

        let r_i = hash(&h, &proof[round]);
        challenges.push(r_i);
    }
    
    // let final_eval = evaluate_Polynomial(f, &challenges);
    // proof.push(final_eval);
    
    (h, proof)
    
}


fn verifier<F : PrimeField>(h: F, proof: &Vec<Vec<F>>, f: &Polynomial<F>, num_vars: usize) -> bool {
    let mut challenges = Vec::new();
    let g_i = proof[round];
        if !verify_partial_sum(&g_i, &challenges, round) {
            return false;
        }
    
    for round in 0..num_vars {
        let r_i = hash(&h, &proof[round]);
        challenges.push(r_i);

        // let g_i = proof[round];
        // if !verify_partial_sum(&g_i, &challenges, round) {
        //     return false;
        // }
    }
    
    let final_eval = proof.last().unwrap();
    // I will evaluate the final final_eval at the last challenge
    if *final_eval != evaluate_Polynomial(f, &challenges) {
        return false;
    }
    //if h != compute_sum(f) {
       // return false;
   // }
    true
}

fn compute_sum<F : PrimeField>(f: &Polynomial<F>) -> F {
    f.iter().fold(F::zero(), |acc, &x| acc + x)
}

fn compute_partial_sum<F : PrimeField>(f: &Polynomial<F>, challenges: &[F], round: usize) -> Vec<F> {
    let mut sum = F::zero();
    
    for (i, &coeff) in f.iter().enumerate() {
            if round == 0 {
                sum += coeff;
            } else{
              sum += coeff * challenges[0].pow([i as u64]);
        }
    }
    sum
}



fn evaluate_Polynomial<F : PrimeField>(f: &Polynomial<F>, challenges: &[F]) -> F {
 let x = challenges[0];
  f.iter()
  .enumerate()
  .fold(F::zero(), |acc, (i, &coeff)| acc + coeff * x.pow([i as u64]))
}

fn hash<F : PrimeField>(h: &F, proof: &[F]) -> F {
    let mut hasher = Sha256::new();
    hasher.update(h.into_bigint().to_bytes_be().as_slice());
    for p in proof {
        hasher.update(p.into_bigint().to_bytes_be().as_slice());
    }
    let result = hasher.finalize();
    F::from(result[0] as u64)
}

fn verify_partial_sum<F : PrimeField>(f: &Polynomial<F>, challenges: &[F], round: usize) -> bool{
    todo!()
}

fn main() {
    let f: Polynomial<Fq> = vec![2.into(), 5.into(),6.into()];
  let  num_vars =1; 
    
    let (h, proof) = prover(&f, num_vars);
    println!("Prover's sum H: {:?}", h);
    println!("Prover's proof: {:?}", proof);
    
    let is_valid = verifier(h, &proof, &f, num_vars);
    println!("Verifier result: {}", is_valid);
}


// fn sum_to_n<F: PrimeField>(n : u32) -> F{
//     let mut sum = F::from(0);
//     for i in 1..=n{
//         sum = sum + F::from(i);
//     }
//     sum
// }


// #[cfg(test)]

// mod tests{
//     use super::*;

//     #[test]
//     fn test_sum_to_n(){
//         let answer:Fq = sum_to_n(10);
//         assert_eq!(answer,Fq::from(55));
//     }
// }