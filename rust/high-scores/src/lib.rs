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
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().copied().max()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted_scores = self.scores.to_owned();
        sorted_scores.sort_unstable_by(|a, b| b.cmp(a));
        sorted_scores.truncate(3);
        sorted_scores
    }
}
