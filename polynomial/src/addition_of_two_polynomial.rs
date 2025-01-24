fn add_polynomials(poly1: Vec<i32>, poly2: Vec<i32>) -> Vec<i32> {
    let max_len = std::cmp::max(poly1.len(), poly2.len());
    let mut result = vec![0; max_len];

    for i in 0..max_len {
        if i < poly1.len() {
            result[i] += poly1[i];
        }
        if i < poly2.len() {
            result[i] += poly2[i];
        }
    }

    result
}

fn main() {
    let poly1 = vec![1, 2, 3]; // Represents 1 + 2x + 3x^2
    let poly2 = vec![4, 5, 6, 7]; // Represents 4 + 5x + 6x^2 + 7x^3

    let result = add_polynomials(poly1, poly2);
    println!("{:?}", result);
}