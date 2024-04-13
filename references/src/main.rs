// code 1
//fn main() {
    //let s1 = String::from("josevictorandreidaltomoreno");
    //let len = calculate_length(&s1);
    //println!("The length of '{}' is {}.", s1, len);
//}

//fn calculate_length(s: &String) -> usize {
    //s.len()
//}
//
// code 2
fn main() {
    let s = String::from("José Victor Moreno");
    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", José Victor Moreno");
}
