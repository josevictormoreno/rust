// code 1
//fn main() {
    //let s1 = String::from("josevictorandreidaltomoreno");
    //let len = calculate_length(&s1);
    //println!("The length of '{}' is {}.", s1, len);
//}

//fn calculate_length(s: &String) -> usize {
    //s.len()
//}

// code 2
//fn main() {
    //let mut s = String::from("José Victor Moreno");
    //change(&mut s);
//}

//fn change(some_string: &mut String) {
    //some_string.push_str(", José Victor Moreno");
//}

// code 3

fn main () {
    let s = String::from("hello");

    // we can do this
    let r1 = &s;
    let r2 = &s;

    //we cannot do this
    //because the last two references are not expecting the value to change
    let r3 = &mut s;
}
