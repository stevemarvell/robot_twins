// src/rdl/models.rs

#[derive(Debug)]
pub struct Robot {
    pub category: String,
    pub parts: Vec<Part>,
}

#[derive(Debug)]
pub struct Part {
    pub category: String,
    pub attributes: Vec<Attribute>,
    pub attachment_points: Vec<AttachmentPoint>,
}

#[derive(Debug)]
pub struct AttachmentPoint {
    pub name: String,
    pub position: Position,
}

#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
pub struct Attribute {
    pub key: String,
    pub value: String,
}
