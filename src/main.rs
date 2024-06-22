use std::{env, path::Path};

use image::{imageops::FilterType, io::Reader as ImageReader};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: {} <image file>", args[0]);
        return;
    }
    // ex: samples\File_Ruine Aggstein 20180527.jpg
    let img_path = Path::new(&args[1]);

    let img = ImageReader::open(img_path).unwrap().decode().unwrap();

    let x = img.resize(1920, 1080, FilterType::Triangle);
    let out_img_path = img_path
        .parent()
        .unwrap()
        .join("processed")
        .join(img_path.file_name().unwrap());

    println!("out_img_path: {:?}", out_img_path);

    // TODO: handle error if png to jpg conversion fails
    x.save_with_format(out_img_path, image::ImageFormat::Png)
        .unwrap();
}
