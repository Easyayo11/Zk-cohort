fn lagrange_interpolation(points: &[(f64, f64)], x: f64) -> f64 {
    let mut result = 0.0;

    for i in 0..points.len() {
        let mut li = 1.0;
        for j in 0..points.len() {
            if i != j {
                li *= (x - points[j].0) / (points[i].0 - points[j].0);
            }
        }
        result += points[i].1 * li;
    }

    result
}

fn main() {

    let points = [(0.0, 1.0), (1.0, 3.0), (2.0, 2.0), (3.0, 5.0)];
    let x = 1.5; 

    let interpolated_value = lagrange_interpolation(&points, x);

    println!("The interpolated value at x = {} is {}", x, interpolated_value);
}
