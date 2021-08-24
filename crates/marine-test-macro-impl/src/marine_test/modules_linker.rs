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

use crate::marine_test::config_utils::Module;

use std::collections::HashMap;
use std::rc::Rc;
use marine_it_parser::it_interface::IRecordTypes;
use marine_it_parser::it_interface::it::{IType, IRecordType};
use itertools::zip;
use std::cmp::Ordering;
use std::hash::Hasher;

pub(super) fn link_modules<'modules>(modules: &'modules [Module<'_>]) -> LinkedModules<'modules> {
    let mut all_record_types = HashMap::<IRecordTypeClosed<'_>, Vec<&str>>::new();
    for module in modules {
        for (_, record_type) in &module.interface.record_types {
            let record_type_ex =
                IRecordTypeClosed::new(record_type.clone(), &module.interface.record_types);
            if let Some(module_names) = all_record_types.get_mut(&record_type_ex) {
                module_names.push(module.name);
            } else {
                all_record_types.insert(record_type_ex, vec![module.name]);
            }
        }
    }

    for (_, names) in &mut all_record_types {
        names.sort();
    }

    let records_in_modules = modules.iter().fold(
        HashMap::<&str, LinkedModule<'_>>::new(),
        |mut accumulator, module| {
            let records = module
                .interface
                .record_types
                .iter()
                .map(|(_, record_type)| -> RecordEntry<'_> {
                    let record_type_ex =
                        IRecordTypeClosed::new(record_type.clone(), &module.interface.record_types);
                    let names = all_record_types.get(&record_type_ex).unwrap();
                    let owner_module = *names.first().unwrap();
                    if owner_module == module.name {
                        RecordEntry::Declare(record_type_ex)
                    } else {
                        RecordEntry::Use(UseDescription {
                            from: owner_module,
                            name: &record_type.name,
                        })
                    }
                })
                .collect::<Vec<_>>();

            accumulator.insert(module.name, LinkedModule { records });
            accumulator
        },
    );

    records_in_modules
}

struct ITypeClosed<'r> {
    ty: IType,
    records: &'r IRecordTypes,
}

impl<'r> ITypeClosed<'r> {
    fn new(ty: IType, records: &'r IRecordTypes) -> Self {
        Self {
            ty,
            records,
        }
    }
}

impl PartialEq for ITypeClosed<'_> {
    fn eq(&self, other: &Self) -> bool {
        use IType::*;
        match (&self.ty, &other.ty) {
            (Boolean, Boolean)
            | (S8, S8)
            | (S16, S16)
            | (S32, S32)
            | (S64, S64)
            | (U8, U8)
            | (U16, U16)
            | (U32, U32)
            | (U64, U64)
            | (F32, F32)
            | (F64, F64)
            | (String, String)
            | (ByteArray, ByteArray)
            | (I32, I32)
            | (I64, I64) => true,
            (Array(self_ty), Array(other_ty)) => {
                ITypeClosed::new(*self_ty.clone(), self.records) == ITypeClosed::new(*other_ty.clone(), other.records)
            }
            (Record(self_record), Record(other_record)) => {
                let self_record = self.records.get(self_record).unwrap();
                let other_record = other.records.get(other_record).unwrap();
                IRecordTypeClosed::new(self_record.clone(), self.records)
                    == IRecordTypeClosed::new(other_record.clone(), other.records)
            }
            _ => false,
        }
    }
}

impl Eq for ITypeClosed<'_> {}

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
        self.record_type.name == other.record_type.name
            && self.record_type.fields.len() == other.record_type.fields.len()
            && zip(
                self.record_type.fields.iter(),
                other.record_type.fields.iter(),
            )
            .all(|(lhs, rhs)| -> bool {
                lhs.name == rhs.name
                    && ITypeClosed::new(lhs.ty.clone(), self.records)
                        == ITypeClosed::new(rhs.ty.clone(), other.records)
            })
    }
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

pub struct LinkedModule<'all> {
    pub records: Vec<RecordEntry<'all>>,
}
