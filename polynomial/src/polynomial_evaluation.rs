fn evaluate_polynomial(coefficients: &[f64], x: f64) -> f64{
    coefficients.iter().rev().fold(0.0,  |acc,&coef| acc *x + coef)
}

fn main(){
    let coefficients = [3.0, 2.0, 1.0]; //ascending order
    let x = 2.0;
    let result = evaluate_polynomial(&coefficients, x);
    println!("The vaue of the polynomial at x={} os {}", x, result);
}