use std::io::{Read, Seek};

use crate::color::iccbased::IccBased;
use crate::color::separation::Separation;
use crate::document::Document;
use crate::errors::{PDFError, PDFResult};
use crate::object::PDFObject;

pub mod cal_gray;
pub mod cal_rgb;
pub mod device_cmyk;
pub mod device_gray;
pub mod device_rgb;
pub mod devicen;
pub mod iccbased;
pub mod indexed;
pub mod lab;
pub mod pattern;
pub mod separation;

mod common;

#[derive(Debug, Clone)]
pub enum ColorSpace {
    DeviceGray(device_gray::DeviceGray),
    DeviceRGB(device_rgb::DeviceRGB),
    DeviceCMYK(device_cmyk::DeviceCMYK),
    CalGray(cal_gray::CalGray),
    CalRGB(cal_rgb::CalRGB),
    Lab(lab::Lab),
    ICCBased(iccbased::IccBased),
    Separation(separation::Separation),
    DeviceN(devicen::DeviceN),
    Indexed(indexed::Indexed),
    Pattern(pattern::Pattern),
}

#[derive(Debug)]
pub struct ColorRGBValue(u32, u32, u32);

impl ColorRGBValue {
    pub fn r(&self) -> u8 {
        self.0 as u8
    }
    pub fn g(&self) -> u8 {
        self.1 as u8
    }

    pub fn b(&self) -> u8 {
        self.2 as u8
    }
}

pub fn create_colorspace<T: Seek + Read>(
    obj: &PDFObject,
    doc: &Document<T>,
) -> PDFResult<ColorSpace> {
    match obj {
        PDFObject::Name(name) => match name.name() {
            "DeviceGray" => Ok(ColorSpace::DeviceGray(device_gray::DeviceGray::default())),
            "DeviceRGB" => Ok(ColorSpace::DeviceRGB(device_rgb::DeviceRGB::default())),
            _ => Err(PDFError::ColorError(format!(
                "colorspace {:?} not implement ",
                name
            ))),
        },
        PDFObject::Arrray(arr) => {
            let first = arr.first().unwrap().as_string().unwrap();
            match first.as_str() {
                "ICCBased" => {
                    let stream = arr.get(1).unwrap();
                    let stream = doc.get_object_without_indriect(stream).unwrap();
                    let iccbased = IccBased::try_new(&stream, doc)?;
                    Ok(ColorSpace::ICCBased(iccbased))
                }
                "Separation" => {
                    let separation = Separation::try_new(arr, doc)?;
                    Ok(ColorSpace::Separation(separation))
                }
                _ => Err(PDFError::ColorError("colorspace not implement".to_string())),
            }
        }
        PDFObject::Dictionary(d) => {
            Err(PDFError::ColorError("colorspace not implement".to_string()))
        }
        _ => Err(PDFError::ColorError("colorspace not implement".to_string())),
    }
}
impl ColorSpace {
    pub fn to_rgb(&self, values: &[f32]) -> PDFResult<ColorRGBValue> {
        match self {
            ColorSpace::Separation(ref s) => s.to_rgb(values),
            ColorSpace::ICCBased(ref c) => c.to_rgb(values),
            _ => {
                panic!("not implement")
            }
        }
    }
}
