use std::{f32::INFINITY, f64::consts::E, intrinsics::expf64, vec};

use image::{DynamicImage, GenericImageView};

use crate::{
    ant::Ant, euclidean_distance, generate_ant_matrix, pheromone_matrix::PheromoneMatrix, PARAMS,
};

pub fn aco_seg(image: DynamicImage) {}

fn aggregation_phero_density(p1: (u32, u32), p2: (u32, u32)) -> f64 {
    E.powf(
        euclidean_distance(p1, p2).powi(2) / (2.0 * PARAMS.get("gaussian_spread").unwrap().powi(2)),
    )
}

fn total_aggregation_phero_density(ant_matrix: &Vec<Vec<Ant>>, p: (u32, u32)) -> f64 {
    let mut total_aggregation_phero_density = 0.0;
    for i in 0..ant_matrix.len() {
        for ant in &ant_matrix[i] {
            if ant.cluster != 0 {
                total_aggregation_phero_density += aggregation_phero_density(p, (ant.x, ant.y));
            }
        }
    }
    total_aggregation_phero_density
}
