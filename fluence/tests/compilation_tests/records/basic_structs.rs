use fluence::marine;

fn main() {}

#[marine]
pub struct TestRecord {
    pub field_0: bool,
    pub field_1: i8,
    pub field_2: i16,
    pub field_3: i32,
    pub field_4: i64,
    pub field_5: u8,
    pub field_6: u16,
    pub field_7: u32,
    pub field_8: u64,
    pub field_9: f32,
    pub field_10: f64,
    pub field_11: String,
    pub field_12: Vec<u8>,
}

#[marine]
pub struct Tx {
    pub block_hash: String,
    pub block_number: String,
    pub from: String,
    pub gas: String,
    pub gas_price: String,
    pub hash: String,
    pub input: String,
    pub nonce: String,
    pub to: String,
    pub transaction_index: String,
    pub value: String,
}

#[marine]
#[derive(Debug)]
pub struct JsonRpcResult {
    pub json_rpc: String,
    pub result: String,
    pub error: String,
    pub id: u64,
}

#[marine]
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct User {
    pub peer_id: String,
    pub relay_id: String,
    pub signature: String,
    pub name: String,
}

#[marine]
pub struct GetUsersServiceResult {
    pub ret_code: i32,
    pub err_msg: String,
    pub users: Vec<User>,
}

#[marine]
pub struct EmptyServiceResult {
    pub ret_code: i32,
    pub err_msg: String,
}

#[marine]
pub struct ExistsServiceResult {
    pub ret_code: i32,
    pub err_msg: String,
    pub is_exists: bool,
}

#[marine]
pub struct AuthResult {
    pub ret_code: i32,
    pub err_msg: String,
    pub is_authenticated: bool,
}
