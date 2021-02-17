use std::env;


#[derive(Debug)]
pub enum AppErrors {
    EnvError(env::VarError),
    ReqwestError(reqwest::Error),
    FetchContentErr(String),
    ChronoError(chrono::ParseError)
}

impl From<env::VarError> for AppErrors {
    fn from(err: env::VarError) -> Self {
        AppErrors::EnvError(err)
    }
}

impl From<reqwest::Error> for AppErrors {
    fn from(err: reqwest::Error) -> Self {
        AppErrors::ReqwestError(err)
    }
}

impl From<String> for AppErrors {
    fn from(err: String) -> Self {
        AppErrors::FetchContentErr(err)
    }
}

impl From<chrono::ParseError> for AppErrors {
    fn from(err: chrono::ParseError) -> Self {
        AppErrors::ChronoError(err)
    }
}
