use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum Answers {
    Close([bool; 2]),
    Open([String; 4]),
}

#[derive(Serialize, Debug)]
pub struct Question {
    pub text: String,
    pub answers: Answers,
    pub index_answer: u8
}