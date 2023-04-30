use image::DynamicImage;

use crate::PARAMS;

pub struct Individual {
    pub edge_pheromones: Vec<Vec<f64>>,
    pub connectivity_pheromones: Vec<Vec<f64>>,
    pub deviation_pheromones: Vec<Vec<f64>>,
}

impl Individual {}

fn generate_solution(image: DynamicImage) {}
