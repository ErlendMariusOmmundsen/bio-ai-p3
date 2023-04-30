use core::f32;

use geo::convex_hull::graham_hull;
use phf::{phf_map, Map};

use image::{DynamicImage, Rgb};

use crate::aco::{apc, get_clusters};

mod aco;
mod ant;
mod individual;
mod pheromone_matrix;

pub static PARAMS: Map<&'static str, f64> = phf_map! {
    "gaussian_spread" => 0.09,
    "step_size" => 1.0,
    "threshold_density" => 0.9,
};

fn main() {
    let image = image::open("training_images/training_images/smile.png")
        .ok()
        .expect("Cannot open image");
    let img = image.as_rgb8().expect("Cannot get RGB from DynamicImage");
    println!("Image dimensions: {:?}", img.dimensions());
    let mut mut_img = img.to_owned();
    for pixel in mut_img.pixels_mut() {
        pixel.0[0] = 255;
    }

    let result_matrix = apc(image);
    let clusters = get_clusters(&result_matrix);

    let hull_points = graham_hull(&mut clusters.get(&1).unwrap().to_owned(), false);

    for point in hull_points {
        mut_img.put_pixel(point.x as u32, point.y as u32, Rgb([0, 0, 255]));
    }

    mut_img.save("test.jpg").expect("Cannot save image");
}

// https://stackoverflow.com/questions/5392061/algorithm-to-check-similarity-of-colors
fn color_distance(e1: Rgb<u8>, e2: Rgb<u8>) -> f32 {
    let rmean: u16 = ((e1.0[0] + e2.0[0]) / 2).into();
    let r: u16 = (e1.0[0] - e2.0[0]).into();
    let g: u16 = (e1.0[1] - e2.0[1]).into();
    let b: u16 = (e1.0[2] - e2.0[2]).into();
    return f32::from((((512 + rmean) * r * r) >> 8) + 4 * g * g + (((767 - rmean) * b * b) >> 8))
        .sqrt();
}

pub fn euclidean_distance(p1: (u32, u32), p2: (u32, u32)) -> f64 {
    let x1 = p1.0;
    let y1 = p1.1;
    let x2 = p2.0;
    let y2 = p2.1;
    let distance = ((x1.abs_diff(x2)).pow(2) + (y1.abs_diff(y2)).pow(2)) as f64;
    distance.sqrt()
}

fn generate_ant_matrix(image: &DynamicImage) -> Vec<Vec<ant::Ant>> {
    let mut ant_matrix: Vec<Vec<ant::Ant>> = Vec::new();
    for y in 0..image.height() {
        let mut row: Vec<ant::Ant> = Vec::new();
        for x in 0..image.width() {
            let ant = ant::Ant::new(x, y, 0);
            row.push(ant);
        }
        ant_matrix.push(row);
    }
    ant_matrix
}
