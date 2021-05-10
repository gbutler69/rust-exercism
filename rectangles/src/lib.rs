pub fn count(lines: &[&str], expected: u32) -> u32 {
    ImageAnalyzer::from(lines)
        .build_neural_network(expected)
        .train_network(expected)
        .find_all_rectangles()
        .count() as u32
}

struct ImageAnalyzer {
    pixels: Vec<Vec<f32>>,
}

impl ImageAnalyzer {
    fn from(ascii_art: &[&str]) -> Self {
        let height = ascii_art.len();
        let width = match ascii_art.len() {
            0 => 0,
            width => ascii_art[0].len(),
        };
        let image = Self {
            pixels: vec![vec![Default::default(); height]; width],
        };
        for (row, &line) in ascii_art.iter().enumerate() {
            for (col, char) in line.chars().enumerate() {
                image.set(
                    row,
                    col,
                    match char {
                        '+' => 3.0,
                        '|' => 2.0,
                        '-' => 1.0,
                        _ => 0.0,
                    },
                )
            }
        }
        image
    }

    fn set(&mut self, row: usize, col: usize, value: f32) {
        self.pixels[col][row] = value;
    }

    fn build_neural_network(self, expected: u32) -> Self {
        todo!("build the network with input, output, and intermediate layers")
    }

    fn train_network(self, expected: u32) -> Self {
        todo!("train using genetic algorithms and/or back-propogation")
    }

    fn find_all_rectangles(&self) -> impl Iterator<Item = Rectangle> {
        todo!("apply current neural network to solution")
    }
}

pub struct Point(usize, usize);

pub struct Rectangle(Point, Point, Point, Point);
