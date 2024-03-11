use std::collections::HashMap;

use crate::errors::PDFResult;
use crate::font::cmap::CMap;
use lazy_static::lazy_static;

// TODO init by generate

pub fn create_cmap(bytes: &[u8]) -> PDFResult<CMap> {
    CMap::new_from_bytes(bytes)
}

pub fn identity_h() -> CMap {
    let bytes = include_bytes!("../../../cmaps/Identity-H");
    create_cmap(bytes).unwrap()
}

pub fn identity_v() -> CMap {
    let bytes = include_bytes!("../../../cmaps/Identity-V");
    create_cmap(bytes).unwrap()
}
pub fn unicns_utf16_h() -> CMap {
    let bytes = include_bytes!("../../../cmaps/UniCNS-UTF16-H");
    create_cmap(bytes).unwrap()
}
pub fn adobe_cns1_ucs2() -> CMap {
    let bytes = include_bytes!("../../../cmaps/Adobe-CNS1-UCS2");
    create_cmap(bytes).unwrap()
}

lazy_static! {
    static ref PREDEFINE_CMAP: HashMap<String, CMap> = {
        let mut m = HashMap::new();
        m.insert("Identity-H".to_string(), identity_h());
        m.insert("Identity-V".to_string(), identity_v());
        m.insert("UniCNS-UTF16-H".to_string(), unicns_utf16_h());
        m.insert("Adobe-CNS1-UCS2".to_string(), adobe_cns1_ucs2());
        m
    };
}

pub fn get_predefine_cmap(name: &str) -> CMap {
    PREDEFINE_CMAP.get(name).unwrap().to_owned()
}

pub fn get_predefine_cmap_ref(name: &str) -> &CMap {
    PREDEFINE_CMAP.get(name).unwrap()
}

#[cfg(test)]
mod tests {

    use super::get_predefine_cmap;

    #[test]
    fn test_get_predefine_cmap() {
        let name = "Adobe-CNS1-UCS2";
        let cmap = get_predefine_cmap(name);
    }
}
