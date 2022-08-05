use std::fs::File;

use bytemuck::{cast_slice, cast_slice_mut};
use image::{DynamicImage, ImageBuffer, Luma, RgbImage};
use ndarray::prelude::*;
use tiff::decoder::{Decoder, DecodingBuffer};

fn get_bytes_mut<'a>(buf: &'a mut DecodingBuffer) -> &'a mut [u8] {
    match buf {
        DecodingBuffer::U8(b) => cast_slice_mut(b),
        DecodingBuffer::U16(b) => cast_slice_mut(b),
        DecodingBuffer::U32(b) => cast_slice_mut(b),
        DecodingBuffer::U64(b) => cast_slice_mut(b),
        _ => unimplemented!("Ya no, I'm not supporting float inputs or signed values. Use PIL or something in python to convert beforehand."),
    }
}

pub enum MagicTiffArray {
    GrayU16(Array<u16, Dim<[usize; 3]>>),
    RgbU8(Array<u8, Dim<[usize; 4]>>),
}

pub fn get_tiff_array(filename: &str) -> MagicTiffArray {
    let mut decoder = Decoder::new(File::open(filename).unwrap()).unwrap();
    let mut images = vec![decoder.read_image().unwrap()];

    let color_type = decoder.colortype().unwrap();
    let dimensions = decoder.dimensions().unwrap();
    println!("{:?}", dimensions);
    println!("{:?}", color_type);

    while decoder.more_images() {
        decoder.next_image().unwrap();
        images.push(decoder.read_image().unwrap());
    }

    let count = images.len();
    println!("{}", count);

    let v = images
        .into_iter()
        .flat_map(|mut b| {
            let b = &mut b.as_buffer(0);
            let v = get_bytes_mut(b).to_vec();
            println!("Bytes: {}", v.len());
            v
        })
        .collect::<Vec<_>>();

    // (v, dimensions, color_type)
    match color_type {
        tiff::ColorType::Gray(16) => {
            let d = cast_slice(&v[..]);
            let array = Array::from_shape_vec(
                (count, dimensions.0 as usize, dimensions.1 as usize),
                d.to_vec(),
            )
            .expect("Uknown error");

            MagicTiffArray::GrayU16(array)
        }
        tiff::ColorType::RGB(8) => {
            let array =
                Array::from_shape_vec((count, dimensions.0 as usize, dimensions.1 as usize, 3), v)
                    .expect("Uknown error");
            MagicTiffArray::RgbU8(array)
        }
        _ => todo!("No support for {:?} yet", color_type),
    }
}

impl MagicTiffArray {
    fn to_image(&self, frame: usize) -> DynamicImage {
        match self {
            MagicTiffArray::RgbU8(arr) => DynamicImage::ImageRgb8(
                RgbImage::from_vec(
                    arr.shape()[1] as u32,
                    arr.shape()[2] as u32,
                    arr.index_axis(Axis(0), frame)
                        .iter()
                        .copied()
                        .collect::<Vec<_>>(),
                )
                .expect("Hopefully this conversion works"),
            ),
            MagicTiffArray::GrayU16(arr) => DynamicImage::ImageLuma16(
                ImageBuffer::<Luma<u16>, Vec<u16>>::from_vec(
                    arr.shape()[1] as u32,
                    arr.shape()[2] as u32,
                    arr.index_axis(Axis(0), frame)
                        .iter()
                        .copied()
                        .collect::<Vec<_>>(),
                )
                .expect("Hopefully this conversion works"),
            ),
        }
    }
}
