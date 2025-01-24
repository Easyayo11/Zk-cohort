pub mod sum_of_an_nth_term;
pub mod shamirr_secret_sharing;
pub mod Addition_of_two_array;
pub mod interpolation;

fn main() {
    let polynomial = vec![(3,2), (2,1), (5,0)];
    let degree = degree(polynomial.clone());

    println!("The degree of the polynomial {:?} = {}", polynomial, degree);

    sum_of_an_nth_term::main();
    Addition_of_two_array::main();
    // shamirr_secret_sharing::main();

}

fn degree(polynomial: Vec<(u32, u32)>) -> u32 {
    let mut current_degree = 0;

    for i in polynomial.iter() {
        if i.1 > current_degree {
            current_degree = i.1
        }
    }

    current_degree
}
