#[derive(Debug)]
pub struct Robot {
    pub category: String,
    pub parts: Vec<Part>,
}

#[derive(Debug)]
pub struct Part {
    pub category: String,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug)]
pub struct Attribute {
    pub key: String,
    pub value: String,
}
