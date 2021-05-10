use fluence::marine;

fn main() {}

#[marine]
struct StructWithBox {
    pub a: Box<i32>,
}

#[marine]
struct StructWithRc {
    pub a: std::rc::Rc<i32>,
}

#[marine]
struct StructWithHashMap {
    pub a: std::collections::HashMap<i32, String>,
}
