use std::io;
const STANDARD_INPUT_ERROR:&str = "Standard input should work";
const INPUT_ERROR:&str = "Input should be number";
fn main() {
    println!("Welcome to my calculator!");

    println!("Enter a number: ");
    let num1 = parse_input();

    println!("Enter another number: ");
    let num2 = parse_input();

    println!("The result is {}",add(num1,num2));
}

fn add(x:i32,y:i32) -> i32{
    x+y
}

fn parse_input() -> i32 {
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect(STANDARD_INPUT_ERROR);
   input1.trim().parse().expect(INPUT_ERROR)
}