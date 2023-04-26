use std::{cmp::min, f32::INFINITY, f64::consts::E, intrinsics::expf64, vec};

use image::{DynamicImage, GenericImageView};

use crate::{
    ant::Ant, euclidean_distance, generate_ant_matrix, pheromone_matrix::PheromoneMatrix, PARAMS,
};

pub fn aco_seg(image: DynamicImage) {}

fn aggregation_phero_density(p1: (u32, u32), p2: (u32, u32)) -> f64 {
    E.powf(
        -(euclidean_distance(p1, p2).powi(2)
            / (2.0 * PARAMS.get("gaussian_spread").unwrap().powi(2))),
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

fn next(image_matrix: &Vec<Vec<Ant>>, p: (u32, u32)) -> (f64, f64) {
    let mut next_point = (0.0, 0.0);
    for i in 0..image_matrix.len() {
        for ant in &image_matrix[i] {
            let density = aggregation_phero_density(p, (ant.x, ant.y));
            next_point = (
                next_point.0 + ((ant.x - p.0) as f64) * density,
                next_point.1 + ((ant.y - p.1) as f64) * density,
            );
        }
    }
    next_point
}

fn apc(image: DynamicImage) {
    let edge_pheromones: PheromoneMatrix =
        PheromoneMatrix::new(image.width() as usize, image.height() as usize);
    let connectivity_pheromones: PheromoneMatrix =
        PheromoneMatrix::new(image.width() as usize, image.height() as usize);
    let overall_deviation_pheromones: PheromoneMatrix =
        PheromoneMatrix::new(image.width() as usize, image.height() as usize);

    let mut clusters: Vec<(u32, u32)> = vec![];
    let num_ants = image.width() * image.height();
    let mut image_matrix = generate_ant_matrix(&image);
    let mut ants: Vec<Vec<Ant>> = generate_ant_matrix(&image);
    let proximity = 4;

    for i in 0..image.width() {
        for j in 0..image.height() {
            if image_matrix[i as usize][j as usize].cluster == 0 {
                let phero_x = total_aggregation_phero_density(&ants, (i, j));
                let x_prime;
                loop {
                    let next_point = next(&ants, (i, j));
                    x_prime = (
                        i + ((PARAMS.get("step_size").unwrap() * next_point.0).round() as f32)
                            as u32
                            / num_ants,
                        j + ((PARAMS.get("step_size").unwrap() * next_point.1).round() as f32)
                            as u32
                            / num_ants,
                    );
                    let phero_x_prime = total_aggregation_phero_density(&image_matrix, x_prime);
                    ants[i as usize][j as usize].x = x_prime.0;
                    ants[i as usize][j as usize].y = x_prime.1;

                    if phero_x_prime < phero_x {
                        break;
                    }
                }

                if clusters.len() == 0 {
                    clusters.push(x_prime);
                    let row_start = x_prime.0.saturating_sub(proximity);
                    let row_end = min(x_prime.0 + proximity, image.width());
                    let col_start = x_prime.1.saturating_sub(proximity);
                    let col_end = min(x_prime.1 + proximity, image.height());
                    for i in row_start..row_end {
                        for j in col_start..col_end {
                            image_matrix[i as usize][j as usize].cluster = 1;
                        }
                    }
                } else {
                }
            }
        }
    }
}
