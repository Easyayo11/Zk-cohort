fn sum_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    if arr1.len() != arr2.len() {
        panic!("Arrays must have the same length!");
    }

    arr1.iter().zip(arr2.iter()).map(|(a, b)| a + b).collect()
}

fn main() {
    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    let result = sum_arrays(&array1, &array2);

    println!("The sum of the arrays is: {:?}", result);
}