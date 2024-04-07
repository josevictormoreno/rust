// code 1
//fn main() {
    //another_function(5);
//}

//fn another_function(x: i32) {
    //println!("The value of x is: {}", x);
//}

// code 2
fn five() -> i32 {
    return 5;
}

fn main() {
    let x = five();
    println!("The value of x is: {}", x);
}
