use serde::Serialize;

#[derive(Serialize)]
pub enum Answers {
    Close([bool; 2]),
    Open([String; 4]),
}

#[derive(Serialize)]
pub struct Question {
    pub text: String,
    pub answers: Answers,
    pub index_answer: u8
}