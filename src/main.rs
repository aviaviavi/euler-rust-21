fn divisors(x: i32) -> Vec<i32> {
   let mut divisors: Vec<i32> = Vec::new();
   let max_candidate = (x / 2) + 1;
   for candidate in 1..max_candidate {
       if x % candidate == 0 {
           divisors.push(candidate);
       }
   }
   divisors
}

fn sum_vec(v: Vec<i32>) -> i32 {
    v.iter().fold(0, |sum, x| sum + x)
}

fn are_amicable(x: i32, y: i32) -> bool {
    let sum_x_divs = sum_vec(divisors(x));
    let sum_y_divs = sum_vec(divisors(y));
    println!("{}", sum_x_divs);
    sum_x_divs == y && sum_y_divs == x
}

fn main () {
    println!("hello");
    println!("result: {:?}", divisors(20));
    println!("result: {:?}", sum_vec(divisors(20)));
    println!("amicable 220, 284: {}", are_amicable(220, 284));
} 
