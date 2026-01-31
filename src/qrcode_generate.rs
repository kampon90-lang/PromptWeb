
use std::io::Cursor;
use base64::{Engine, engine::general_purpose};
use qrcode::QrCode;
use qrcode::render::svg;
use image::{Luma};
//use base64::{Engine as _, engine::general_purpose};
use image::ImageFormat;

pub fn generate_qr_file_png(payload: &str, filename: &str){
    let code = QrCode::new(payload).unwrap();
    let namefile = format!("{}.png", &filename);
    let img = code.render::<Luma<u8>>().build();
    img.save(namefile).unwrap(); 
}


pub fn generate_qr_file_svg(payload: &str, filename: &str){
    let code = QrCode::new(payload).unwrap();
    let namefile = format!("{}.svg", &filename);
    let svg_data = code.render::<svg::Color>()
        .dark_color(svg::Color("black"))
        .light_color(svg::Color("white"))
        .build();
    std::fs::write(namefile, svg_data).unwrap();
}


pub fn generate_qr_raw_base64_svg(payload: &str) -> String{
    let code = QrCode::new(payload).unwrap();

    let svg_data = code.render::<svg::Color>()
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build();

    let base64_string = base64::engine::general_purpose::STANDARD.encode(svg_data);
    base64_string
}


pub fn generate_qr_base64_svg(payload: &str) -> String{
    let base64_string = generate_qr_raw_base64_svg(payload);
    format!("data:image/svg+xml;base64,{}", base64_string)

}


pub fn generate_qr_raw_base64_png(payload: &str) -> String{
// Create QrCode payload
    let code = match QrCode::new(payload) {
        Ok(c) => c,
        Err(_) => return String::new(),
    };

    // Render image
    let img = code.render::<Luma<u8>>().build();

    // Convert to base64
    // Write to Memory Cursor
    let mut buff = Cursor::new(Vec::new());
    img.write_to(&mut buff, ImageFormat::Png).unwrap();
    
    // Base64
    let base64_string = general_purpose::STANDARD.encode(buff.get_ref());

    base64_string
}


pub fn generate_qr_base64_png(payload: &str) -> String{

    let base64_string = generate_qr_raw_base64_png(payload);

    format!("data:image/png;base64,{}", base64_string)
}


