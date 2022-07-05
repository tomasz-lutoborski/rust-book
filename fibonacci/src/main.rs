use std::io;

fn main() {
    println!("Enter number");

    let mut num = String::new();

    io::stdin().read_line(&mut num).expect("failed to read a line");

    let num: u32 = num.trim().parse().expect("not a number");

    let mut result = 1;
    let mut prev = 0;
    let mut prev_res = 0;

    for _number in 1..num {
        prev_res = result;
        result += prev;
        prev = prev_res;
    }

    println!("{}", result);

}
