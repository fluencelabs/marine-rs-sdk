use marine_rs_sdk::marine;

fn main() {}

#[marine]
struct A {
    #[cfg(target_os = "wasm")]
    pub field: i64,
}
