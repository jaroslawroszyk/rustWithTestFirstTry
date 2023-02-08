mod math;
mod math_tests;
use std::io;

fn main() {
    println!("Enter two numbers separated by a space: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let (x, y) = {
        let mut numbers = input.split_whitespace();
        (
            numbers.next().unwrap().parse().unwrap(),
            numbers.next().unwrap().parse().unwrap(),
        )
    };
    let result = math::sum(x, y);
    println!("The sum of {} and {} is: {}", x, y, result);
}

// #[cfg(test)]
// mod tests
// {
//     use crate::math::sum;

//     #[test]
//     fn add_works()
//     {
//         let result = sum(2,2);
//         assert_eq!(result,4);
//     }

//     #[test]
//     fn add_works2()
//     {
//         let result = sum(21,2);
//         assert_eq!(result,23);
//     }
// }