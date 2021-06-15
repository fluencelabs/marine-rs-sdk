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
    pub fn __m_generated_serialize(&self) -> *const u8 {
        let mut raw_record: Vec<u8> = Vec::with_capacity(4 * 6usize);
        let field_ident_ptr = self.init_peer_id.as_ptr() as u32;
        raw_record.extend(&field_ident_ptr.to_le_bytes());
        raw_record.extend(&(self.init_peer_id.len() as u32).to_le_bytes());
        let field_ident_ptr = self.service_id.as_ptr() as u32;
        raw_record.extend(&field_ident_ptr.to_le_bytes());
        raw_record.extend(&(self.service_id.len() as u32).to_le_bytes());
        let field_ident_ptr = self.service_creator_peer_id.as_ptr() as u32;
        raw_record.extend(&field_ident_ptr.to_le_bytes());
        raw_record.extend(&(self.service_creator_peer_id.len() as u32).to_le_bytes());
        let field_ident_ptr = self.host_id.as_ptr() as u32;
        raw_record.extend(&field_ident_ptr.to_le_bytes());
        raw_record.extend(&(self.host_id.len() as u32).to_le_bytes());
        let field_ident_ptr = self.particle_id.as_ptr() as u32;
        raw_record.extend(&field_ident_ptr.to_le_bytes());
        raw_record.extend(&(self.particle_id.len() as u32).to_le_bytes());
        unsafe fn __m_generated_vec_serializer_tetraplets_5(
            arg: &Vec<Vec<SecurityTetraplet>>
        ) -> (u32, u32) {
            unsafe fn __m_generated_vec_serializer_tetraplets_5_SecurityTetraplet(
                arg: &Vec<SecurityTetraplet>
            ) -> (u32, u32) {
                let mut result: Vec<u32> = Vec::with_capacity(arg.len());
                for value in arg {
                    result.push(value.__m_generated_serialize() as _);
                }
                let result_ptr = result.as_ptr();
                let result_len = result.len();
                marine_rs_sdk::internal::add_object_to_release(Box::new(result));
                (result_ptr as _, result_len as _)
            }
            let mut result: Vec<u32> = Vec::with_capacity(2 * arg.len());
            for value in arg {
                let (ptr, size) =
                    __m_generated_vec_serializer_tetraplets_5_SecurityTetraplet(&value);
                result.push(ptr as _);
                result.push(size as _);
            }
            let result_ptr = result.as_ptr();
            let result_len = result.len() / 2;
            marine_rs_sdk::internal::add_object_to_release(Box::new(result));
            (result_ptr as _, result_len as _)
        }
        let serialized_arg_5 =
            unsafe { __m_generated_vec_serializer_tetraplets_5(&self.tetraplets) };
        raw_record.extend(&serialized_arg_5.0.to_le_bytes());
        raw_record.extend(&serialized_arg_5.1.to_le_bytes());
        let raw_record_ptr = raw_record.as_ptr();
        marine_rs_sdk::internal::add_object_to_release(Box::new(raw_record));
        raw_record_ptr as _
    }
    pub unsafe fn __m_generated_deserialize(record_ptr: *const u8) -> Self {
        let raw_record: Vec<u8> = Vec::from_raw_parts(record_ptr as _, 48usize, 48usize);
        let field_0 = unsafe {
            let offset = u32::from_le_bytes([
                raw_record[0usize],
                raw_record[0usize + 1],
                raw_record[0usize + 2],
                raw_record[0usize + 3],
            ]);
            let size = u32::from_le_bytes([
                raw_record[0usize + 4],
                raw_record[0usize + 5],
                raw_record[0usize + 6],
                raw_record[0usize + 7],
            ]);
            String::from_raw_parts(offset as _, size as _, size as _)
        };
        let field_1 = unsafe {
            let offset = u32::from_le_bytes([
                raw_record[8usize],
                raw_record[8usize + 1],
                raw_record[8usize + 2],
                raw_record[8usize + 3],
            ]);
            let size = u32::from_le_bytes([
                raw_record[8usize + 4],
                raw_record[8usize + 5],
                raw_record[8usize + 6],
                raw_record[8usize + 7],
            ]);
            String::from_raw_parts(offset as _, size as _, size as _)
        };
        let field_2 = unsafe {
            let offset = u32::from_le_bytes([
                raw_record[16usize],
                raw_record[16usize + 1],
                raw_record[16usize + 2],
                raw_record[16usize + 3],
            ]);
            let size = u32::from_le_bytes([
                raw_record[16usize + 4],
                raw_record[16usize + 5],
                raw_record[16usize + 6],
                raw_record[16usize + 7],
            ]);
            String::from_raw_parts(offset as _, size as _, size as _)
        };
        let field_3 = unsafe {
            let offset = u32::from_le_bytes([
                raw_record[24usize],
                raw_record[24usize + 1],
                raw_record[24usize + 2],
                raw_record[24usize + 3],
            ]);
            let size = u32::from_le_bytes([
                raw_record[24usize + 4],
                raw_record[24usize + 5],
                raw_record[24usize + 6],
                raw_record[24usize + 7],
            ]);
            String::from_raw_parts(offset as _, size as _, size as _)
        };
        let field_4 = unsafe {
            let offset = u32::from_le_bytes([
                raw_record[32usize],
                raw_record[32usize + 1],
                raw_record[32usize + 2],
                raw_record[32usize + 3],
            ]);
            let size = u32::from_le_bytes([
                raw_record[32usize + 4],
                raw_record[32usize + 5],
                raw_record[32usize + 6],
                raw_record[32usize + 7],
            ]);
            String::from_raw_parts(offset as _, size as _, size as _)
        };
        unsafe fn __m_generated_vec_deserializer_40(
            offset: u32,
            size: u32
        ) -> Vec<Vec<SecurityTetraplet>> {
            unsafe fn __m_generated_vec_deserializer_40_SecurityTetraplet(
                offset: u32,
                size: u32
            ) -> Vec<SecurityTetraplet> {
                let mut arg: Vec<u32> = Vec::from_raw_parts(offset as _, size as _, size as _);
                let mut result = Vec::with_capacity(arg.len());
                for offset in arg {
                    let value = SecurityTetraplet::__m_generated_deserialize(offset as _);
                    result.push(value);
                }
                result
            }
            let vec_passing_size = 2;
            let mut arg: Vec<u32> =
                Vec::from_raw_parts(offset as _, (vec_passing_size * size) as _, (vec_passing_size * size) as _);
            let mut result = Vec::with_capacity(arg.len());
            let mut arg = arg.into_iter();
            while let Some(offset) = arg.next() {
                let size = arg.next().unwrap();
                let value = __m_generated_vec_deserializer_40_SecurityTetraplet(
                    offset as _,
                    size as _
                );
                result.push(value);
            }
            result
        }
        let offset = u32::from_le_bytes([
            raw_record[40usize],
            raw_record[40usize + 1],
            raw_record[40usize + 2],
            raw_record[40usize + 3],
        ]);
        let size = u32::from_le_bytes([
            raw_record[40usize + 4],
            raw_record[40usize + 5],
            raw_record[40usize + 6],
            raw_record[40usize + 7],
        ]);
        let field_5 = unsafe { __m_generated_vec_deserializer_40(offset as _, size as _) };
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
#[link_section = "__m_generated_section__CallParameters"]
pub static __m_generated_static_global_CallParameters: [u8; 455usize] = {
    * b"{\"ast_type\":\"Record\",\"name\":\"CallParameters\",\"fields\":{\"Named\":[{\"name\":\"init_peer_id\",\"ty\":{\"Utf8String\":\"ByValue\"}},{\"name\":\"service_id\",\"ty\":{\"Utf8String\":\"ByValue\"}},{\"name\":\"service_creator_peer_id\",\"ty\":{\"Utf8String\":\"ByValue\"}},{\"name\":\"host_id\",\"ty\":{\"Utf8String\":\"ByValue\"}},{\"name\":\"particle_id\",\"ty\":{\"Utf8String\":\"ByValue\"}},{\"name\":\"tetraplets\",\"ty\":{\"Vector\":[{\"Vector\":[{\"Record\":[\"SecurityTetraplet\",\"ByValue\"]},\"ByValue\"]},\"ByValue\"]}}]}}"
};
