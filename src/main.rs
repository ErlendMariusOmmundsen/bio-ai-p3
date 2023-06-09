use core::f32;

use phf::{phf_map, Map};

use image::Rgb;

mod aco;
mod ant;
mod pheromone_matrix;

pub static PARAMS: Map<&'static str, f32> = phf_map! {
    "gaussian_spread" => 1.0,
    "proportionality_constant" => 1.0,
    "threshold_density" => 0.5,
};

fn main() {
    let image = image::open("training_images/training_images/147091/Test image.jpg")
        .ok()
        .expect("Cannot open image");
    let img = image.as_rgb8().expect("Cannot get RGB from DynamicImage");
    println!("Image dimensions: {:?}", img.dimensions());
    let mut mut_img = img.to_owned();
    for pixel in mut_img.pixels_mut() {
        pixel.0[0] = 255;
    }
    mut_img.save("test.jpg").expect("Cannot save image");
}

pub fn euclidean_distance(p1: (u32, u32), p2: (u32, u32)) -> f64 {
    let x1 = p1.0;
    let y1 = p1.1;
    let x2 = p2.0;
    let y2 = p2.1;
    let distance = ((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f64;
    distance.sqrt()
}
    let mut ant_matrix: Vec<Vec<ant::Ant>> = Vec::new();
    for y in 0..image.height() {
        let mut row: Vec<ant::Ant> = Vec::new();
        for x in 0..image.width() {
            let ant = ant::Ant::new(x, y);
            row.push(ant);
        }
        ant_matrix.push(row);
    }
    ant_matrix
}
