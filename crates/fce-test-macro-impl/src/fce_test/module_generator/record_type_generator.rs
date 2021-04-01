/*
 * Copyright 2021 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::fce_test::utils;
use crate::TResult;

use fce_wit_parser::interface::it::IRecordFieldType;
use fce_wit_parser::interface::FCERecordTypes;

use proc_macro2::TokenStream;
use quote::quote;

pub(super) fn generate_records(records: &FCERecordTypes) -> TResult<TokenStream> {
    use std::ops::Deref;

    let mut result = TokenStream::new();

    for (_, record) in records.iter() {
        let record_name_ident = utils::generate_record_name(&record.name)?;
        let fields = prepare_field(record.fields.deref().iter(), records)?;

        let record = quote! {
            #[derive(Clone, fluence_test::internal::Serialize, fluence_test::internal::Deserialize)]
            pub struct #record_name_ident {
                #(#fields),*
            }
        };
        result.extend(record);
    }

    Ok(result)
}

fn prepare_field<'f>(
    fields: impl ExactSizeIterator<Item = &'f IRecordFieldType>,
    records: &FCERecordTypes,
) -> TResult<Vec<TokenStream>> {
    let mut result = Vec::with_capacity(fields.len());

    for field in fields {
        let field_name = utils::new_ident(&field.name)?;
        let field_type = utils::itype_to_tokens(&field.ty, records)?;

        let field = quote! { #field_name: #field_type };
        result.push(field);
    }

    Ok(result)
}
