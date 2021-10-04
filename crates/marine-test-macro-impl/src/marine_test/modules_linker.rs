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

use crate::{TResult, TestGeneratorError};

use marine_it_parser::it_interface::{IRecordTypes, IModuleInterface};
use marine_it_parser::it_interface::it::{IType, IRecordType};

use itertools::zip;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hasher;
use std::rc::Rc;
use static_assertions::const_assert;

pub(super) fn link_modules<'modules>(
    modules: impl ExactSizeIterator<Item = (&'modules str, &'modules IModuleInterface)>,
) -> TResult<LinkedModules<'modules>> {
    let mut all_record_types = HashMap::<IRecordTypeClosed<'_>, &str>::new();
    let mut linked_modules = HashMap::<&str, LinkedModule<'_>>::new();

    for (name, interface) in modules {
        let mut linking_module = LinkedModule::default();
        for record_type in interface.record_types.values() {
            let record_type_ex =
                IRecordTypeClosed::new(record_type.clone(), &interface.record_types);

            let entry = match all_record_types.get(&record_type_ex) {
                Some(owner_module) => RecordEntry::Use(UseDescription {
                    from: owner_module,
                    name: &record_type.name,
                }),
                None => {
                    all_record_types.insert(record_type_ex.clone(), name);
                    RecordEntry::Declare(record_type_ex)
                }
            };

            linking_module.records.push(entry);
        }

        if linked_modules.insert(name, linking_module).is_some() {
            return Err(TestGeneratorError::DuplicateModuleName(name.to_string()));
        }
    }

    Ok(linked_modules)
}

struct ITypeClosed<'r> {
    ty: &'r IType,
    records: &'r IRecordTypes,
}

impl<'r> ITypeClosed<'r> {
    fn new(ty: &'r IType, records: &'r IRecordTypes) -> Self {
        Self { ty, records }
    }
}

impl PartialEq for ITypeClosed<'_> {
    fn eq(&self, other: &Self) -> bool {
        use IType::*;
        // check if new variants require special handling in the match below
        #[allow(unused)]
        const LAST_VERIFIED_ITYPE_SIZE: usize = 17;
        const_assert!(IType::VARIANT_COUNT == LAST_VERIFIED_ITYPE_SIZE);

        match (&self.ty, &other.ty) {
            (Array(self_ty), Array(other_ty)) => {
                ITypeClosed::new(self_ty, self.records) == ITypeClosed::new(other_ty, other.records)
            }
            (Record(self_record), Record(other_record)) => {
                let self_record = self.records.get(self_record);
                let other_record = other.records.get(other_record);

                // ID from Record(ID) potentially may not be in .records, if it happens comparision is always FALSE
                match (self_record, other_record) {
                    (None, _) => false,
                    (_, None) => false,
                    (Some(self_record), Some(other_record)) => {
                        IRecordTypeClosed::new(self_record.clone(), self.records)
                            == IRecordTypeClosed::new(other_record.clone(), other.records)
                    }
                }
            }
            (lhs, rhs) if lhs == rhs => true,
            _ => false,
        }
    }
}

#[derive(Clone)]
pub struct IRecordTypeClosed<'r> {
    pub record_type: Rc<IRecordType>,
    pub records: &'r IRecordTypes,
}

impl<'r> IRecordTypeClosed<'r> {
    fn new(record_type: Rc<IRecordType>, records: &'r IRecordTypes) -> Self {
        Self {
            record_type,
            records,
        }
    }
}

impl PartialEq for IRecordTypeClosed<'_> {
    fn eq(&self, other: &Self) -> bool {
        let names_are_equal = self.record_type.name == other.record_type.name;
        names_are_equal && fields_are_equal(self, other)
    }
}

fn fields_are_equal(lhs: &IRecordTypeClosed<'_>, rhs: &IRecordTypeClosed<'_>) -> bool {
    let same_fields_count = lhs.record_type.fields.len() == rhs.record_type.fields.len();
    same_fields_count
        && zip(lhs.record_type.fields.iter(), rhs.record_type.fields.iter()).all(
            |(lhs_field, rhs_field)| -> bool {
                lhs_field.name == rhs_field.name
                    && ITypeClosed::new(&lhs_field.ty, lhs.records)
                        == ITypeClosed::new(&rhs_field.ty, rhs.records)
            },
        )
}

impl PartialOrd for IRecordTypeClosed<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IRecordTypeClosed<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.record_type.name.cmp(&other.record_type.name)
    }
}

impl Eq for IRecordTypeClosed<'_> {}

impl std::hash::Hash for IRecordTypeClosed<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.record_type.name.hash(state);
    }
}

pub type LinkedModules<'r> = HashMap<&'r str, LinkedModule<'r>>;

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct UseDescription<'r> {
    pub from: &'r str,
    pub name: &'r str,
}

#[derive(PartialEq, Eq)]
pub enum RecordEntry<'r> {
    Use(UseDescription<'r>),
    Declare(IRecordTypeClosed<'r>),
}

impl PartialOrd for RecordEntry<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RecordEntry<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        use RecordEntry::*;
        match (self, other) {
            (Use(_), Declare(_)) => Ordering::Less,
            (Declare(_), Use(_)) => Ordering::Greater,
            (Use(lhs), Use(rhs)) => lhs.cmp(rhs),
            (Declare(lhs), Declare(rhs)) => lhs.record_type.name.cmp(&rhs.record_type.name),
        }
    }
}

#[derive(Default)]
pub struct LinkedModule<'all> {
    pub records: Vec<RecordEntry<'all>>,
}
