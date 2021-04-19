use crate::new_ident;
use crate::parsed_type::ParsedType;
use crate::ast_types::*;

use proc_macro2::TokenStream;
use quote::quote;

pub(super) struct FieldValuesBuilder {
    value_id: usize,
    fields_der: TokenStream,
    field_value_idents: Vec<syn::Ident>,
}

/// Contains all necessary info to construct record fields.
pub(super) struct FieldValuesOutcome {
    /// Generated deserializer for each record field.
    pub(super) fields_der: TokenStream,

    /// Idents of each record field.
    pub(super) field_value_idents: Vec<syn::Ident>,
}

impl FieldValuesBuilder {
    pub(super) fn build<'a>(
        fields: impl ExactSizeIterator<Item = &'a AstRecordField>,
    ) -> FieldValuesOutcome {
        let values_builder = Self::new(fields.len());
        values_builder.build_impl(fields)
    }

    fn new(fields_count: usize) -> Self {
        Self {
            value_id: 0,
            fields_der: TokenStream::new(),
            field_value_idents: Vec::with_capacity(fields_count),
        }
    }

    fn build_impl<'r>(
        mut self,
        fields: impl ExactSizeIterator<Item = &'r AstRecordField>,
    ) -> FieldValuesOutcome {
        for (id, ast_field) in fields.enumerate() {
            let field_value_ident = new_ident!(format!("field_{}", id));
            let field_der = self.field_der(ast_field, &field_value_ident);

            self.field_value_idents.push(field_value_ident);
            self.fields_der.extend(field_der);
        }

        let outcome = FieldValuesOutcome {
            fields_der: self.fields_der,
            field_value_idents: self.field_value_idents,
        };

        outcome
    }

    fn field_der(&mut self, ast_field: &AstRecordField, field: &syn::Ident) -> TokenStream {
        let der = match &ast_field.ty {
            ParsedType::Boolean(_) => self.bool_der(field),
            ParsedType::I8(_) => self.i8_der(field),
            ParsedType::I16(_) => self.i16_der(field),
            ParsedType::I32(_) => self.i32_der(field),
            ParsedType::I64(_) => self.i64_der(field),
            ParsedType::U8(_) => self.u8_der(field),
            ParsedType::U16(_) => self.u16_der(field),
            ParsedType::U32(_) => self.u32_der(field),
            ParsedType::U64(_) => self.u64_der(field),
            ParsedType::F32(_) => self.f32_der(field),
            ParsedType::F64(_) => self.f64_der(field),
            ParsedType::Utf8Str(_) | ParsedType::Utf8String(_) => self.string_der(field),
            ParsedType::Vector(ty, _) => self.vector_der(ty, field),
            ParsedType::Record(name, _) => self.record_der(name, field),
        };

        der
    }

    fn bool_der(&mut self, field: &syn::Ident) -> TokenStream {
        let value_id = self.value_id;
        let result = quote! { let #field = raw_record[#value_id] != 0; };
        self.value_id += 1;
        result
    }

    fn i8_der(&mut self, field: &syn::Ident) -> TokenStream {
        let value_id = self.value_id;
        let result = quote! { let #field = raw_record[#value_id] as i8; };
        self.value_id += 1;
        result
    }

    fn i16_der(&mut self, field: &syn::Ident) -> TokenStream {
        let value_id = self.value_id;
        let result = quote! { let #field = i16::from_le_bytes([
            raw_record[#value_id],
            raw_record[#value_id + 1],
        ]);
        };

        self.value_id += 2;
        result
    }

    fn i32_der(&mut self, field: &syn::Ident) -> TokenStream {
        let value_id = self.value_id;
        let result = quote! { let #field = i32::from_le_bytes([
            raw_record[#value_id],
            raw_record[#value_id + 1],
            raw_record[#value_id + 2],
            raw_record[#value_id + 3],
        ]);
        };

        self.value_id += 4;
        result
    }

    fn i64_der(&mut self, field: &syn::Ident) -> TokenStream {
        let value_id = self.value_id;
        let result = quote! { let #field = i64::from_le_bytes([
            raw_record[#value_id],
            raw_record[#value_id + 1],
            raw_record[#value_id + 2],
            raw_record[#value_id + 3],
            raw_record[#value_id + 4],
            raw_record[#value_id + 5],
            raw_record[#value_id + 6],
            raw_record[#value_id + 7],
        ]);
        };

        self.value_id += 8;
        result
    }

    fn u8_der(&mut self, field: &syn::Ident) -> TokenStream {
        let value_id = self.value_id;
        let result = quote! { let #field = raw_record[#value_id] as u8; };
        self.value_id += 1;
        result
    }

    fn u16_der(&mut self, field: &syn::Ident) -> TokenStream {
        let value_id = self.value_id;
        let result = quote! { let #field = u16::from_le_bytes([
            raw_record[#value_id],
            raw_record[#value_id + 1],
        ]);
        };

        self.value_id += 2;
        result
    }

    fn u32_der(&mut self, field: &syn::Ident) -> TokenStream {
        let value_id = self.value_id;
        let result = quote! { let #field = u32::from_le_bytes([
            raw_record[#value_id],
            raw_record[#value_id + 1],
            raw_record[#value_id + 2],
            raw_record[#value_id + 3],
        ]);
        };

        self.value_id += 4;
        result
    }

    fn u64_der(&mut self, field: &syn::Ident) -> TokenStream {
        let value_id = self.value_id;
        let result = quote! { let #field = u64::from_le_bytes([
            raw_record[#value_id],
            raw_record[#value_id + 1],
            raw_record[#value_id + 2],
            raw_record[#value_id + 3],
            raw_record[#value_id + 4],
            raw_record[#value_id + 5],
            raw_record[#value_id + 6],
            raw_record[#value_id + 7],
        ]);
        };

        self.value_id += 8;
        result
    }

    fn f32_der(&mut self, field: &syn::Ident) -> TokenStream {
        let value_id = self.value_id;
        let result = quote! { let #field = f32::from_le_bytes([
            raw_record[#value_id],
            raw_record[#value_id + 1],
            raw_record[#value_id + 2],
            raw_record[#value_id + 3],
        ]);
        };

        self.value_id += 4;
        result
    }

    fn f64_der(&mut self, field: &syn::Ident) -> TokenStream {
        let value_id = self.value_id;
        let result = quote! { let #field = f64::from_le_bytes([
            raw_record[#value_id],
            raw_record[#value_id + 1],
            raw_record[#value_id + 2],
            raw_record[#value_id + 3],
            raw_record[#value_id + 4],
            raw_record[#value_id + 5],
            raw_record[#value_id + 6],
            raw_record[#value_id + 7],
        ]);
        };

        self.value_id += 8;
        result
    }

    fn string_der(&mut self, field: &syn::Ident) -> TokenStream {
        let ptr_id = self.value_id;
        let size_id = self.value_id + 1;
        self.value_id += 1;

        quote! {
            let #field = unsafe { String::from_raw_parts(raw_record[#ptr_id] as _, raw_record[#size_id] as _, raw_record[#size_id] as _) };
        }
    }

    fn vector_der(&mut self, ty: &ParsedType, field: &syn::Ident) -> TokenStream {
        let generated_der_name = format!("__fce_generated_vec_deserializer_{}", self.value_id);
        let generated_der_name = crate::utils::prepare_ident(generated_der_name);
        let generated_der_ident = new_ident!(generated_der_name);

        let vector_deserializer =
            crate::parsed_type::generate_vector_deserializer(ty, &generated_der_name);

        let ptr_id = self.value_id;
        let size_id = self.value_id + 1;
        self.value_id += 1;

        quote! {
            #vector_deserializer
            let #field = unsafe { #generated_der_ident(raw_record[#ptr_id] as _, raw_record[#size_id] as _) };
        }
    }

    fn record_der(&mut self, name: &str, field: &syn::Ident) -> TokenStream {
        let ptr_id = self.value_id;
        let record_ident = new_ident!(name);

        quote! {
            let #field = #record_ident::__fce_generated_deserialize(raw_record[#ptr_id] as _);
        }
    }
}
