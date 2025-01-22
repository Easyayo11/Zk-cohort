fn scalar_multiplication(poly:Vec<i32>, scalar:i32) -> Vec<i32> {
    poly.iter().map(|&coef| coef *scalar).collect()
}

fn main(){
    let poly = vec![1,2,3];// ascending order p(x)=1+2x+3x^2
    let scalar =0;
    let result  = scalar_multiplication(poly, scalar);
    println!("{:?}", result);
}