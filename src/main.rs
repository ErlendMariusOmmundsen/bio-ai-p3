use image::GenericImageView;

fn main() {
    let img = image::open("training_images/training_images/147091/Test image.jpg").unwrap();
    println!("Image dimensions: {:?}", img.dimensions());
}
