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

use crate::marine_test::utils;
use crate::TResult;

use marine_it_parser::it_interface::it::IRecordFieldType;
use marine_it_parser::it_interface::IRecordTypes;

use proc_macro2::TokenStream;
use quote::quote;

pub(super) fn generate_records(records: &IRecordTypes) -> TResult<Vec<TokenStream>> {
    use std::ops::Deref;

    records.iter().map(|(_, record)| -> TResult<_> {
        let record_name_ident = utils::generate_record_name(&record.name)?;
        let fields = prepare_field(record.fields.deref().iter(), records)?;

        let generated_record = quote! {
            #[derive(Clone, Debug, marine_rs_sdk_test::internal::serde::Serialize, marine_rs_sdk_test::internal::serde::Deserialize)]
            #[serde(crate = "marine_rs_sdk_test::internal::serde")]
            pub struct #record_name_ident {
                #(pub #fields),*
            }
        };

        Ok(generated_record)
    }
    ).collect::<TResult<Vec<_>>>()
}

fn prepare_field<'f>(
    fields: impl ExactSizeIterator<Item = &'f IRecordFieldType>,
    records: &IRecordTypes,
) -> TResult<Vec<TokenStream>> {
    fields
        .map(|field| -> TResult<_> {
            let field_name = utils::new_ident(&field.name)?;
            let field_type = utils::itype_to_tokens(&field.ty, records)?;

            let generated_field = quote! { #field_name: #field_type };

            Ok(generated_field)
        })
        .collect::<TResult<Vec<_>>>()
}
