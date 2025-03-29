mod args;
use args::ImgArgs;
use image::{ io::Reader, DynamicImage, ImageFormat, imageops::FilterType::Triangle, GenericImageView };
use std::{ io::BufReader, fs::File, convert::TryInto };

#[derive(Debug)]
enum ImgDataErrors {
    DifferentImageFormats,
    BufferTooSmall,
    //UnableToReadImageFromPath(std::io::Error),
}

struct FloatingImg {
    img_width: u32,
    img_height: u32,
    img_data: Vec<u8>,
    img_name: String,
}

impl FloatingImg {
    fn new(img_width: u32, img_height: u32, img_name: String) -> Self  {
        // let buffer_capacity = 3_655_744;
        let buffer_capacity = img_height * img_width * 4;
        let buffer = Vec::with_capacity(buffer_capacity.try_into().unwrap());

        FloatingImg {
            img_width,
            img_height,
            img_data: buffer,
            img_name,
        }
    }

    fn set_img_data(&mut self, img_data: Vec<u8>) -> Result<(), ImgDataErrors> {
        if img_data.len() > self.img_data.capacity() {
            return Err(ImgDataErrors::BufferTooSmall);
        }

        self.img_data = img_data;
        Ok(())
    }
}

fn main() -> Result<(), ImgDataErrors> {
    let args = ImgArgs::new();
    let (img_1, img1_format) = find_img_from_path(args.img_1);
    let (img_2, img2_format) = find_img_from_path(args.img_2);

    if img1_format != img2_format {
        return Err(ImgDataErrors::DifferentImageFormats);
    }

    let (img_1, img_2) = standardize_img_sizes(img_1, img_2);

    let mut result = FloatingImg::new(img_1.width(), img_1.height(), args.feedback);

    let combined_images_data= process_img_cobination(img_1, img_2);

    result.set_img_data(combined_images_data)?;

    image::save_buffer_with_format(result.img_name, &result.img_data, result.img_width, result.img_height, image::ColorType::Rgba8, img1_format).unwrap();

    Ok(())
}

fn find_img_from_path (path: String) -> (DynamicImage, ImageFormat) {
    let img_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let img_format: ImageFormat = img_reader.format().unwrap();
    let img: DynamicImage = img_reader.decode().unwrap();
    (img, img_format)
}

fn get_img_with_smallest_dimensions(img_1_dimensions: (u32, u32), img_2_dimensions: (u32, u32)) -> (u32, u32) {
    let img1_pixels = img_1_dimensions.0 * img_1_dimensions.1;
    let img2_pixels = img_2_dimensions.0 * img_2_dimensions.1;

    return if img1_pixels < img2_pixels  {
        img_1_dimensions
    } else {
        img_2_dimensions
    }
}

fn standardize_img_sizes (img_1: DynamicImage, img_2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (smallest_img_width, smallest_img_height) = get_img_with_smallest_dimensions(img_1.dimensions(), img_2.dimensions());

    println!("img_width: {}, img_height: {}\n", smallest_img_width, smallest_img_height);

    if img_2.dimensions() == (smallest_img_width, smallest_img_height) {
        return (img_1.resize_exact(smallest_img_width, smallest_img_height, Triangle), img_2);
    } else {
        return (img_1, img_2.resize_exact(smallest_img_width, smallest_img_height, Triangle))
    }
}

fn process_img_cobination(img_1: DynamicImage, img_2: DynamicImage) -> Vec<u8> {
    let img1_vec = img_1.to_rgba8().into_vec();
    let img2_vec = img_2.to_rgba8().into_vec();

    alternate_img_pixels(img1_vec, img2_vec)
}

fn alternate_img_pixels (img1_vec: Vec<u8>, img2_vec: Vec<u8>) -> Vec<u8> {
    let mut combined_images_data = vec![0u8; img1_vec.len()];

    let mut i = 0;

    while i < img1_vec.len() {
        if i % 8 == 0 {
            combined_images_data.splice(i..= i + 3, set_rgba(&img1_vec, i, i + 3));
        } else {
            combined_images_data.splice(i..= i + 3, set_rgba(&img2_vec, i, i + 3));
        }
        i += 4;
    }

    return combined_images_data;
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::new();

    for i in start..=end {
        let value: u8 = match vec.get(i) {
            Some(d) => *d,
            None => panic!("index out of range!!"),
        };
        rgba.push(value);
    }
    return rgba;
}


