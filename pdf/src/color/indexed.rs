use std::io::{Read, Seek};

use crate::color::device_gray::DeviceGray;
use crate::color::{create_colorspace, ColorSpace};
use crate::document::Document;
use crate::errors::{PDFError, PDFResult};
use crate::object::PDFArray;

#[derive(Debug, Clone)]
pub struct Indexed {
    base: Box<ColorSpace>,
    hival: u8,
    lookup: Vec<u8>,
}

impl Default for Indexed {
    fn default() -> Self {
        Indexed {
            base: Box::new(ColorSpace::DeviceGray(DeviceGray::default())),
            hival: 0,
            lookup: Vec::new(),
        }
    }
}

impl Indexed {
    pub fn try_new<T: Seek + Read>(obj: &PDFArray, doc: &Document<T>) -> PDFResult<Self> {
        // NOTE: base is a Name or Array not implement Array
        if obj.len() != 4 {
            return Err(PDFError::ColorError(format!(
                "Indexed colorspace param need 4 element got:{:?}",
                obj
            )));
        }
        // TODO: fix unwrap
        let base = obj.get(1).unwrap();
        let base = create_colorspace(base, doc)?;
        let base = Box::new(base);
        let hival = obj.get(2).unwrap().as_u8()?;
        let lookup_stream = doc.get_object_without_indriect(obj.last().unwrap())?;
        let lengths = doc.get_object_without_indriect(lookup_stream.get_value("Length").unwrap());
        // TODO: lookup table
        let lookup_bytes = lookup_stream.bytes();

        unimplemented!()
    }

    pub fn number_of_components(&self) -> u8 {
        unimplemented!()
    }
}
