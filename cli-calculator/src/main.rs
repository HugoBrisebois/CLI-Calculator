use std::io;

fn main() {

    // Welcome to prompt
    println!("Welcome to the CLI Calculator! \n what operation do you want to make?");
    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read line");

    // trim the user input to see which operation to do
    if operation.trim() == "add" {
        // prepare the first the number for calculation
        println!("enter the first number you want to enter");
        let mut a = String::new();
        io::stdin()
            .read_line(&mut a)
            .expect("Failed to read line");
        let num1 = a.trim().parse::<i32>();

        // prepare the second number for calculation
        println!("Enter the second number you want to add");
        let mut b = String::new();
        io::stdin()
            .read_line(&mut b)
            .expect("Failed to read line");
        let num2 = b.trim().parse::<i32>();

        // add both numbers
        let answer = add(&num1, &num2);
        println!("the sum of {:?} and {:?} is {:?}", &num1, &num2, &answer);
    }
}


// basic math functions

// addition
fn add(a:&i32, b:&i32) -> i32 {
    return a + b
}

// subtraction
fn subtract(a: &i32, b: &i32) -> i32 {
    return a - b
}

// multiplication
fn multiply(a: &i32, b: &i32) -> i32 {
    return a*b
}

// division
fn divide(a: &i32, b: &i32) -> i32 {
    return a/b
}

