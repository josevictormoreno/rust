//fn main() {
//// code 1
//let x = 5;
//let x = x + 1;
//{
//let x = x * 2;
//println!("The value of x is: {x}");
//}

//println!("The value of x is: {x}");

////code 2
//let spaces = "   ";
//let spaces = spaces.len();
//}

// code 3
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index must be a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
