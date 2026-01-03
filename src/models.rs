use serde::Serialize;

#[derive(Serialize)]
pub enum Answer {
    Close([bool; 2]),
    Open([String; 4]),
}

#[derive(Serialize)]
pub struct Question {
    pub text: String,
    pub answers: Answer,
    pub index_answer: usize
}