use std::{env, io};
use tokio::task;


#[derive(Debug)]
pub enum AppErrors {
    EnvError(env::VarError),
    ReqwestError(reqwest::Error),
    FetchContentErr(String),
    ChronoError(chrono::ParseError),
    WriterError(io::Error),
    SerdeJsonError(serde_json::Error),
    TokioJoinError(task::JoinError)
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

impl From<io::Error> for AppErrors {
    fn from(err: io::Error) -> Self {
        AppErrors::WriterError(err)
    }
}

impl From<serde_json::Error> for AppErrors {
    fn from(err: serde_json::Error) -> Self {
        AppErrors::SerdeJsonError(err)
    }
}

impl From<task::JoinError> for AppErrors {
    fn from(err: task::JoinError) -> Self {
        AppErrors::TokioJoinError(err)
    }
}
