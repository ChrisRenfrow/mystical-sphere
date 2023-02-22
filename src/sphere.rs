use rand::Rng;

use crate::config::CfgAnswers;

#[derive(Debug)]
pub struct Sphere {
    pub answers: Vec<Answer>,
}

#[derive(Debug, Clone)]
pub struct Answer {
    pub flavor: Flavor,
    pub text: String,
}

#[derive(Debug, Clone)]
pub enum Flavor {
    Affirmative,
    Neutral,
    Negative,
}

impl Sphere {
    pub fn new(answers: CfgAnswers) -> Self {
        Self {
            answers: answers.into(),
        }
    }

    pub fn get_answer(&self, rng: &mut impl Rng) -> Answer {
        let idx: usize = rng.gen_range(0..self.answers.len());
        self.answers[idx].clone()
    }
}

impl From<CfgAnswers> for Vec<Answer> {
    fn from(answers: CfgAnswers) -> Self {
        let affirmative: Vec<Answer> = answers
            .affirmative
            .iter()
            .map(|a| Answer {
                flavor: Flavor::Affirmative,
                text: a.to_string(),
            })
            .collect();
        let neutral: Vec<Answer> = answers
            .neutral
            .iter()
            .map(|a| Answer {
                flavor: Flavor::Neutral,
                text: a.to_string(),
            })
            .collect();
        let negative: Vec<Answer> = answers
            .negative
            .iter()
            .map(|a| Answer {
                flavor: Flavor::Negative,
                text: a.to_string(),
            })
            .collect();
        vec![affirmative, neutral, negative]
            .iter()
            .flatten()
            .cloned()
            .collect()
    }
}
