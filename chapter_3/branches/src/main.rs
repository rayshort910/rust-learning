// fn main() {
//     let number = 3;

//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }

// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// fn main() {
//     let condition = true;
//     let number = if condition {
//         5
//     } else {
//         6
//     };

//     println!("The value of number is: {}", number);
// }

// fn main() {
//     loop {
//         println!("again!");
//     }
// }

// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {}", result);
// }

// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{}!", number);

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5 {
//         println!("the value is: {}", a[index]);

//         index += 1;
//     }
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a.iter() {
//         println!("the value is: {}", element);
//     }
// }

fn main() {
    let n = 83;
    println!("Fibonacci number {} is {}", n, fib(n));
}

// fib provides the nth fibonacci number
fn fib(n: u32) -> u64 {
    if n == 0 {
        0
    } else if n == 1 || n == 2 {
        1
    } else {
        fibo(1, 1, n-2)
    }
}

// fibo is the recursive function to progress through fibonacci numbers
fn fibo(prev1: u64, prev2: u64, iter: u32) -> u64 {
    if iter == 1 {
        prev1 + prev2
    } else {
        fibo(prev2, prev1+prev2, iter-1)
    }
}