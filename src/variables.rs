//TODO
pub struct Variable {
    pub name: String,
    pub value: i32,
    pub domain: Vec<i32>, // TODO:: Change Vec<i32> to Domain type, i.e. Domain::Int(Vec<i32>)
}

impl Variable {
    pub fn new(name: String, value: i32, domain: Vec<i32>) -> Self {
        Variable {
            name: name,
            value: value,
            domain: domain,
        }
    }
}
