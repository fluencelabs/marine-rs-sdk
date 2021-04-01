use fluence::fce;

pub fn main() {}

#[fce]
pub fn byte_type(mut arg: Vec<u8>) -> Vec<u8> {
    arg.push(0);

    let mut arg = unsafe { effector::byte_type(arg) };

    arg.push(2);
    arg
}

#[fce]
pub fn inner_arrays_1(mut arg: Vec<Vec<Vec<Vec<u8>>>>) -> Vec<Vec<Vec<Vec<u8>>>> {
    arg.push(vec![vec![vec![0]]]);

    let mut arg = unsafe { effector::inner_arrays_1(arg) };

    arg.push(vec![vec![vec![2]]]);
    arg
}

#[fce]
#[derive(Default)]
pub struct TestRecord {
    pub field_0: i32,
    pub field_1: Vec<Vec<u8>>,
}

#[fce]
pub fn inner_arrays_2(mut arg: Vec<Vec<Vec<Vec<TestRecord>>>>) -> Vec<Vec<Vec<Vec<TestRecord>>>> {
    arg.push(vec![vec![vec![
        TestRecord {
            field_0: 0,
            field_1: vec![vec![1]],
        },
        TestRecord::default(),
    ]]]);

    let mut arg = unsafe { effector::inner_arrays_2(arg) };

    arg.push(vec![vec![vec![
        TestRecord {
            field_0: 1,
            field_1: vec![vec![2]],
        },
        TestRecord::default(),
    ]]]);

    arg
}

#[fce]
pub fn string_type(mut arg: Vec<String>) -> Vec<String> {
    arg.push(String::from("fce"));

    let mut arg = unsafe { effector::string_type(arg) };

    arg.push(String::from("test"));
    arg
}

/*
#[fce]
pub fn bool_type(arg: Vec<bool>) -> Vec<bool> {
    let mut arg = unsafe { effector::bool_type(arg) };

    arg.push(false);
    arg
}
 */

#[fce]
pub fn f32_type(mut arg: Vec<f32>) -> Vec<f32> {
    arg.push(0.0);

    let mut arg = unsafe { effector::f32_type(arg) };

    arg.push(1.0);
    arg
}

#[fce]
pub fn f64_type(mut arg: Vec<f64>) -> Vec<f64> {
    arg.push(0.0);

    let mut arg = unsafe { effector::f64_type(arg) };

    arg.push(1.0);
    arg
}

#[fce]
pub fn u32_type(mut arg: Vec<u32>) -> Vec<u32> {
    arg.push(0);

    let mut arg = unsafe { effector::u32_type(arg) };

    arg.push(2);
    arg
}

#[fce]
pub fn u64_type(mut arg: Vec<u64>) -> Vec<u64> {
    arg.push(0);

    let mut arg = unsafe { effector::u64_type(arg) };

    arg.push(2);
    arg
}

#[fce]
pub fn i32_type(mut arg: Vec<i32>) -> Vec<i32> {
    arg.push(0);

    let mut arg = unsafe { effector::i32_type(arg) };

    arg.push(2);
    arg
}

#[fce]
pub fn i64_type(mut arg: Vec<i64>) -> Vec<i64> {
    arg.push(0);

    let mut arg = unsafe { effector::i64_type(arg) };

    arg.push(1);
    arg
}

#[fce]
pub fn empty_type() -> Vec<String> {
    unsafe { effector::empty_type() }
}

mod effector {
    use fluence::fce;
    use super::TestRecord;

    #[fce]
    #[link(wasm_import_module = "arrays_passing_effector")]
    extern "C" {
        pub fn inner_arrays_1(arg: Vec<Vec<Vec<Vec<u8>>>>) -> Vec<Vec<Vec<Vec<u8>>>>;

        pub fn inner_arrays_2(
            arg: Vec<Vec<Vec<Vec<TestRecord>>>>,
        ) -> Vec<Vec<Vec<Vec<TestRecord>>>>;

        pub fn string_type(arg: Vec<String>) -> Vec<String>;

        pub fn byte_type(arg: Vec<u8>) -> Vec<u8>;

        /*
        pub fn bool_type(arg: Vec<bool>) -> Vec<bool>;
         */

        pub fn f32_type(arg: Vec<f32>) -> Vec<f32>;

        pub fn f64_type(arg: Vec<f64>) -> Vec<f64>;

        pub fn u32_type(arg: Vec<u32>) -> Vec<u32>;

        pub fn u64_type(arg: Vec<u64>) -> Vec<u64>;

        pub fn i32_type(arg: Vec<i32>) -> Vec<i32>;

        pub fn i64_type(arg: Vec<i64>) -> Vec<i64>;

        pub fn empty_type() -> Vec<String>;
    }
}
