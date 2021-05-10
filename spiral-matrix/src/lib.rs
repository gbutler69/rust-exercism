mod alg;
mod draw;
mod orientation;

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut matrix = alg::Matrix::new(size as usize, size as usize, 0);
    let mut pen = 1u32..;
    let mut turtle = draw::Turtle::new();
    turtle.fill_clockwise((0, 0), orientation::Direction::East, &mut pen, &mut matrix);
    matrix.unflatten_to_owned()
}
