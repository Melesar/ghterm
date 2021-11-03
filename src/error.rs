use std::fmt::Display;
use std::convert::From;

#[derive(Debug)]
pub enum Error {
    Other(String),
    GhNotInstalled,
    RefusedToAuthenticate,
    NotARepo(String),
    PrDoesntExist(String, u32),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Other(msg) => write!(f, "Error occured in ghterm:\n{}", msg),
            Error::GhNotInstalled => write!(f, "ghterm requires GitHub cli (aka gh) to be in $PATH"),
            Error::RefusedToAuthenticate => write!(f, "You need to be authenticated to GitHub to use ghterm"),
            Error::NotARepo(repo) => write!(f, "{} is not a GitHub repository", repo),
            Error::PrDoesntExist(repo, pr) => write!(f, "Pull request #{} in {} doesn't exist", pr, repo),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Other(err.to_string())
    }
}
