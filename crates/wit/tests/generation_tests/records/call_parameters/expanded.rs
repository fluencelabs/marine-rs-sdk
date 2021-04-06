pub struct CallParameters {
    /// Peer id of the AIR script initiator.
    pub init_peer_id: String,
    /// Id of the current service.
    pub service_id: String,
    /// Id of the service creator.
    pub service_creator_peer_id: String,
    /// Id of the host which run this service.
    pub host_id: String,
    /// Id of the particle which execution resulted a call this service.
    pub particle_id: String,
    /// Security tetraplets which described origin of the arguments.
    pub tetraplets: Vec<Vec<SecurityTetraplet>>,
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
impl CallParameters {
    pub fn __fce_generated_serialize(self) -> *const u8 {
        let mut raw_record: Vec<u64> = Vec::new();
        raw_record.push(self.init_peer_id.as_ptr() as _);
        raw_record.push(self.init_peer_id.len() as _);
        raw_record.push(self.service_id.as_ptr() as _);
        raw_record.push(self.service_id.len() as _);
        raw_record.push(self.service_creator_peer_id.as_ptr() as _);
        raw_record.push(self.service_creator_peer_id.len() as _);
        raw_record.push(self.host_id.as_ptr() as _);
        raw_record.push(self.host_id.len() as _);
        raw_record.push(self.particle_id.as_ptr() as _);
        raw_record.push(self.particle_id.len() as _);
        unsafe fn __fce_generated_vec_serializer_tetraplets_5(
            arg: &Vec<Vec<SecurityTetraplet>>
        ) -> (u32, u32) {
            unsafe fn __fce_generated_vec_serializer_tetraplets_5_SecurityTetraplet(
                arg: &Vec<SecurityTetraplet>
            ) -> (u32, u32) {
                let mut result: Vec<u32> = Vec::with_capacity(arg.len());
                for value in arg {
                    result.push(value.__fce_generated_serialize() as _);
                }
                (result.as_ptr() as _, (4 * result.len()) as _)
            }
            let mut result: Vec<u32> = Vec::with_capacity(2 * arg.len());
            for value in arg {
                let (ptr, size) =
                    __fce_generated_vec_serializer_tetraplets_5_SecurityTetraplet(&value);
                result.push(ptr as _);
                result.push(size as _);
            }
            (result.as_ptr() as _, (4 * result.len()) as _)
        }
        let serialized_arg_5 =
            unsafe { __fce_generated_vec_serializer_tetraplets_5(&self.tetraplets) };
        raw_record.push(serialized_arg_5.0 as _);
        raw_record.push(serialized_arg_5.1 as _);
        let raw_record_ptr = raw_record.as_ptr();
        fluence::internal::add_object_to_release(Box::new(self));
        raw_record_ptr as _
    }
    pub unsafe fn __fce_generated_deserialize(record_ptr: *const u8) -> Self {
        let raw_record: Vec<u64> = Vec::from_raw_parts(record_ptr as _, 96usize, 96usize);
        let field_0 = unsafe {
            String::from_raw_parts(
                raw_record[0usize] as _,
                raw_record[1usize] as _,
                raw_record[1usize] as _
            )
        };
        let field_1 = unsafe {
            String::from_raw_parts(
                raw_record[2usize] as _,
                raw_record[3usize] as _,
                raw_record[3usize] as _
            )
        };
        let field_2 = unsafe {
            String::from_raw_parts(
                raw_record[4usize] as _,
                raw_record[5usize] as _,
                raw_record[5usize] as _
            )
        };
        let field_3 = unsafe {
            String::from_raw_parts(
                raw_record[6usize] as _,
                raw_record[7usize] as _,
                raw_record[7usize] as _
            )
        };
        let field_4 = unsafe {
            String::from_raw_parts(
                raw_record[8usize] as _,
                raw_record[9usize] as _,
                raw_record[9usize] as _
            )
        };
        unsafe fn __fce_generated_vec_deserializer_10(
            offset: u32,
            size: u32
        ) -> Vec<Vec<SecurityTetraplet>> {
            let size = size / 8;
            unsafe fn __fce_generated_vec_deserializer_10_SecurityTetraplet(
                offset: u32,
                size: u32
            ) -> Vec<SecurityTetraplet> {
                let size = size / 8;
                let mut arg: Vec<u64> = Vec::from_raw_parts(offset as _, size as _, size as _);
                let mut result = Vec::with_capacity(arg.len());
                for offset in arg {
                    let value = SecurityTetraplet::__fce_generated_deserialize(offset as _);
                    result.push(value);
                }
                result
            }
            let mut arg: Vec<u64> = Vec::from_raw_parts(offset as _, size as _, size as _);
            let mut result = Vec::with_capacity(arg.len());
            let mut arg = arg.into_iter();
            while let Some(offset) = arg.next() {
                let size = arg.next().unwrap();
                let value = __fce_generated_vec_deserializer_10_SecurityTetraplet(
                    offset as _,
                    size as _
                );
                result.push(value);
            }
            result
        }
        let field_5 = unsafe {
            __fce_generated_vec_deserializer_10(
                raw_record[10usize] as _,
                raw_record[11usize] as _
            )
        };
        Self {
            init_peer_id: field_0,
            service_id: field_1,
            service_creator_peer_id: field_2,
            host_id: field_3,
            particle_id: field_4,
            tetraplets: field_5
        }
    }
}
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
#[allow(clippy::all)]
#[link_section = "__fce_generated_section__CallParameters"]
pub static __fce_generated_static_global_CallParameters: [u8; 445usize] = {
    * b"{\"ast_type\":\"Record\",\"name\":\"CallParameters\",\"fields\":[{\"name\":\"init_peer_id\",\"ty\":{\"Utf8String\":\"ByValue\"}},{\"name\":\"service_id\",\"ty\":{\"Utf8String\":\"ByValue\"}},{\"name\":\"service_creator_peer_id\",\"ty\":{\"Utf8String\":\"ByValue\"}},{\"name\":\"host_id\",\"ty\":{\"Utf8String\":\"ByValue\"}},{\"name\":\"particle_id\",\"ty\":{\"Utf8String\":\"ByValue\"}},{\"name\":\"tetraplets\",\"ty\":{\"Vector\":[{\"Vector\":[{\"Record\":[\"SecurityTetraplet\",\"ByValue\"]},\"ByValue\"]},\"ByValue\"]}}]}"
};