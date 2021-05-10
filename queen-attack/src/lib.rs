#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..=7, 0..=7) => Some(Self { rank, file }),
            _ => None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let s_position = &self.position;
        let o_position = &other.position;
        match (s_position.rank, s_position.file) {
            (rank, _) if rank == o_position.rank => true,
            (_, file) if file == o_position.file => true,
            (rank, file) => {
                let position_delta = (rank - o_position.rank).abs();
                let file_delta = (file - o_position.file).abs();
                position_delta == file_delta
            }
        }
    }
}
