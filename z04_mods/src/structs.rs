#[allow(dead_code)]
#[derive(Debug)]
pub struct Person {
    first_name: String,
    last_name: String,
    birth_year: u16,
    birth_month: u8,
}

impl Person {
    pub fn new(first_name: String, last_name: String, birth_year: u16, birth_month: u8) -> Person {
        Person {
            first_name,
            last_name,
            birth_year,
            birth_month,
        }
    }
}
