#[derive(Debug)]
pub enum Error {
    ParsingParameters,
    MissingParameters,
    QuestionNotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::ParsingParameters => {
                write!(f, "Cannot parse parameter")
            }
            Error::MissingParameters => write!(f, "Missing required parameters"),
            Error::QuestionNotFound => write!(f, "Question not found"),
        }
    }
}
