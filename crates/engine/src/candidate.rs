// candidate.rs is responsible for handling candidate word logic in the input method framework. 

pub struct Candidate {
    pub text: String,
    pub score: u32,
}

impl Candidate {
    pub fn new(text: String, score: u32) -> Self {
        Candidate { text, score }
    }
}

pub struct CandidateList {
    candidates: Vec<Candidate>,
}

impl CandidateList {
    pub fn new() -> Self {
        CandidateList {
            candidates: Vec::new(),
        }
    }

    pub fn add_candidate(&mut self, candidate: Candidate) {
        self.candidates.push(candidate);
    }

    pub fn get_candidates(&self) -> &Vec<Candidate> {
        &self.candidates
    }

    pub fn clear(&mut self) {
        self.candidates.clear();
    }
}