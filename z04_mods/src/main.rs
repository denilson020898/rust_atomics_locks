mod options;
mod structs;

fn main() {
    options::options_runner();
    let person = structs::Person::new("denilson".to_string(), "son".to_string(), 1998, 12);
    println!("the person is: {person:?}");
}
