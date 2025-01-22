fn polynomial_mul(poly1: Vec<i32>, poly2: Vec<i32>) -> Vec<i32>{
    let degree1 = poly1.len();
    let degree2 =poly2.len();
    let mut result = vec![0; degree1 + degree2 -1];

    for i in 0..degree1{
        for j in 0..degree2{
            result[i+j] += poly1[i] * poly2[j];
        }
    }
 result
}

fn main(){
    let poly1 = vec![1,2,3];
    let poly2 = vec![4,3,1,2];
    let result = polynomial_mul(poly1, poly2);
    println!("{:?}", result);
}