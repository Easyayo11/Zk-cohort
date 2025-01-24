pub fn main(){
    let n = 5;
    let sum = sum_of_nth_term(n);
    println!("The sum of the first {} whole numbers is: {}", n, sum);

}

fn sum_of_nth_term(n: u32) -> u32{
(n* ( n + 1 ))/2
}
