#[derive(Debug)]
pub struct HighScores<'scores> {
    scores: &'scores [u32],
}

impl<'scores> HighScores<'scores> {
    pub fn new(scores: &'scores [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted = self.scores.to_vec();
        sorted.sort_unstable_by(|a, b| b.cmp(a));
        sorted.truncate(3);

        sorted
    }
}
