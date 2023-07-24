use marine_rs_sdk::marine;

fn main() {}

#[marine]
struct A {
    #[doc = "This is a doc attribute"]
    pub field: i64,
}
