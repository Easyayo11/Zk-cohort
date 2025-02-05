use ark_ff::PrimeField;
// use ark_bls12_381::Fr;

pub(crate) fn lagrange_interpolation<F: PrimeField>(points: &[(F, F)], x: F) -> F {
    let mut result=F::zero();

    for i in 0..points.len() {
        let mut li=F::one();
        for j in 0..points.len() {
            if i != j {
                let (x_i, _) = points[i];
                let (x_j, _) = points[j];
                li *= (x - x_j) * (x_i- x_j).inverse().unwrap();
            }
        }
        result += points[i].1 * li;
    }

    result
}

fn main() { 

    // use ark_bls12_381::Fr;
    use ark_bn254::Fr;


    let points = vec![(Fr::from(0u64), Fr::from(1u64)),(Fr::from(1u64), Fr::from(3u64)), (Fr::from(2u64), Fr::from(2u64)), (Fr::from(3u64), Fr::from(5u64))];
    let x = Fr::from(1u64);
    let interpolated_value = lagrange_interpolation(&points, x);

    println!("The interpolated value at x = {} is {}", x, interpolated_value);
}
