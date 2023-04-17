pub struct PheromoneMatrix {
    pub width: usize,
    pub height: usize,
    pub matrix: Vec<Vec<f64>>,
}

impl PheromoneMatrix {
    pub fn new(width: usize, height: usize) -> PheromoneMatrix {
        PheromoneMatrix {
            width,
            height,
            matrix: vec![vec![0.0; width]; height],
        }
    }

    pub fn update_element(&mut self, x: usize, y: usize, value: f64) {
        self.matrix[y][x] = value;
    }
}
