// This file defines common schemas for the double pinyin input method framework. 

#[derive(Debug, Clone)]
pub struct DoublePinyinSchema {
    pub initials: Vec<String>,
    pub finals: Vec<String>,
    pub tone_marks: Vec<String>,
}

impl DoublePinyinSchema {
    pub fn new(initials: Vec<String>, finals: Vec<String>, tone_marks: Vec<String>) -> Self {
        DoublePinyinSchema {
            initials,
            finals,
            tone_marks,
        }
    }

    pub fn get_initials(&self) -> &Vec<String> {
        &self.initials
    }

    pub fn get_finals(&self) -> &Vec<String> {
        &self.finals
    }

    pub fn get_tone_marks(&self) -> &Vec<String> {
        &self.tone_marks
    }
}