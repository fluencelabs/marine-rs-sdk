use super::WITGenerator;
use super::Interfaces;

use wit_support::AstRecordItem;

impl WITGenerator for AstRecordItem {
    fn generate_wit<'a>(&'a self, _interfaces: &mut Interfaces<'a>) {
        unimplemented!()
    }
}
