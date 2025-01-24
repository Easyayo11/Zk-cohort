pub(crate) fn lagrange_interpolation(points: &[(u64, u64)], x: u64) -> u64 {
    let mut result = 0;

    for i in 0..points.len() {
        let mut li = 1;
        for j in 0..points.len() {
            if i != j {
                dbg!(points[i]);
                li *= (x - points[j]) / (points[i]- points[j]);
            }
        }
        result += points[i] * li;
    }

    result
}

fn main() {

    let points = [(0, 1), (1, 3), (2, 2), (3, 5)];
    let x = 1; 

    let interpolated_value = lagrange_interpolation(&points, x);

    println!("The interpolated value at x = {} is {}", x, interpolated_value);
}
