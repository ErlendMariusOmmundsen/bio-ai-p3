use std::{cmp::min, collections::HashMap, f64::consts::E, vec};

use geo::{coord, Coord, Point};
use image::DynamicImage;

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
            total_aggregation_phero_density += aggregation_phero_density(p, (ant.x, ant.y));
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
                next_point.0 + ((ant.x.saturating_sub(p.0)) as f64) * density,
                next_point.1 + ((ant.y.saturating_sub(p.1)) as f64) * density,
            );
        }
    }
    next_point
}

pub fn get_clusters(image_matrix: &Vec<Vec<Ant>>) -> HashMap<&u32, Vec<Coord<i32>>> {
    let mut clusters = HashMap::new();
    for i in 0..image_matrix.len() {
        for j in 0..image_matrix[i].len() {
            clusters
                .entry(&image_matrix[i][j].cluster)
                .or_insert(Vec::new())
                .push(coord! {x: image_matrix[i][j].x as i32, y: image_matrix[i][j].y as i32});
        }
    }
    clusters
}

pub fn apc(image: DynamicImage) -> Vec<Vec<Ant>> {
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
                let mut x_prime;
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
                    let phero_x_prime = total_aggregation_phero_density(&ants, x_prime);
                    if phero_x_prime > phero_x {
                        ants[i as usize][j as usize].x = x_prime.0;
                        ants[i as usize][j as usize].y = x_prime.1;
                        continue;
                    }
                    break;
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
                    for j in 0..clusters.len() {
                        let boo = f64::min(
                            total_aggregation_phero_density(&ants, x_prime),
                            total_aggregation_phero_density(&ants, clusters[j]),
                        ) / f64::max(
                            total_aggregation_phero_density(&ants, x_prime),
                            total_aggregation_phero_density(&ants, clusters[j]),
                        ) > *PARAMS.get("threshold_density").unwrap();

                        if boo && euclidean_distance(x_prime, clusters[j]) < (4 * proximity) as f64
                        {
                            image_matrix[x_prime.0 as usize][x_prime.1 as usize].cluster =
                                (j + 1) as u32; // Zj already exists
                        } else {
                            clusters.push(x_prime);
                            let row_start = x_prime.0.saturating_sub(proximity);
                            let row_end = min(x_prime.0 + proximity, image.width());
                            let col_start = x_prime.1.saturating_sub(proximity);
                            let col_end = min(x_prime.1 + proximity, image.height());
                            for i in row_start..row_end {
                                for j in col_start..col_end {
                                    image_matrix[i as usize][j as usize].cluster = (j + 1) as u32;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    image_matrix
}
