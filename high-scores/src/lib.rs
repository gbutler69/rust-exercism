#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().map(|i| *i)
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().map(|i| *i)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = Vec::<u32>::from(self.scores);
        (&mut scores).sort();
        scores.reverse();
        while scores.len() > 3 {
            scores.pop();
        }
        scores
    }
}
