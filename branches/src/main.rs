//code 1
//fn main() {
//let number = 3;

//if number < 5 {
//println!("condition was true");
//} else {
//println!("condition was false");
//}
//}

//code 2
//fn main() {
//let number = 6;
//if number % 4 == 0 {
//println!("number is divisible by 4");
//} else if number % 3 == 0 {
//println!("number is divisible by 3");
//} else if number % 2 == 0 {
//println!("number is divisible by 2");
//} else {
//println!("number is not divisible by 4, 3, or 2");
//}
//}

//code 3
//fn main() {
//let condition = true;
//let number = if condition {5} else {6};
//println!("The value of number is: {}", number);
//}

//code 4
//fn main() {
    //let mut counter = 0;

    //let result = loop {
        //counter += 1;

        //if counter == 10 {
            //break counter * 2;
        //}
    //};

    //println!("The result is {result}");
//}

//code 5
fn main() {
    let mut conte = 0;
    'counting_up: loop {
        let mut conte = 0;
        'counting_down: loop {
            conte += 1;
            if conte == 5 {
                break 'counting_up;
            }
        }
    }
}

// the for and while blocks are just like in python 
// for number in (1..4) {}
// while 1 < 4 {}
