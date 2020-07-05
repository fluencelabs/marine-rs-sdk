use fluence::fce;

fn main() {
    println!("Hello, world!");
}

#[fce]
pub fn greeting(arg: String, arg2: String, arg3: i32) -> i64 {
    let res = format!("Hi {} {}", arg, arg2);
    ipfs(res, arg2);
    arg3 as _
}

#[fce]
#[link(wasm_import_module = "ipfs_node.wasm")]
extern "C" {
    pub fn ipfs(cmd: String, aa: String) -> String;
}
