use fluence::fce;

fn main() {}

#[fce]
struct StructWithBox {
    pub a: Box<i32>,
}

#[fce]
struct StructWithRc {
    pub a: std::rc::Rc<i32>,
}

#[fce]
struct StructWithHashMap {
    pub a: std::collections::HashMap<i32, String>,
}
