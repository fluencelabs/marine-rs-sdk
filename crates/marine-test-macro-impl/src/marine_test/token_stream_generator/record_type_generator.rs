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

use crate::marine_test::modules_linker::{LinkedModule, RecordEntry, UseDescription};
use itertools::Itertools;

pub(super) fn generate_records(
    linked_module: &LinkedModule<'_>,
    import_generator: fn(info: &UseDescription<'_>) -> TResult<TokenStream>,
) -> TResult<Vec<TokenStream>> {
    linked_module.records
        .iter()
        .sorted()
        .map(|record| -> TResult<_> {
            use RecordEntry::*;
            match record {
                Use(use_info) => import_generator(use_info),
                Declare(record) => {
                    let record_name_ident = utils::new_ident(&record.record_type.name)?;
                    let fields = prepare_field(record.record_type.fields.iter(), record.records)?;

                    Ok(quote! {
                        #[derive(Clone, Debug, marine_rs_sdk_test::internal::serde::Serialize, marine_rs_sdk_test::internal::serde::Deserialize)]
                        #[serde(crate = "marine_rs_sdk_test::internal::serde")]
                        pub struct #record_name_ident {
                            #(pub #fields),*
                        }
                    })
                }
            }
        })
        .collect::<TResult<Vec<_>>>()
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
