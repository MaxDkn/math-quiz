use crate::models::{Answers, Question};

pub fn generate() -> Question {
    Question {
        text: "La proposition \"$7\\mid63$\" est-elle vraie ?".to_string(),
        answers: Answers::Close([true, false]),
        index_answer: 0,
    }
}