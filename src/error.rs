use thiserror::Error;

pub type Result<T> = core::result::Result<T, GlobalErr>;

#[derive(Error, Debug)]
pub enum GlobalErr {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error("Failed to {0}")]
    CustomErr(String),
}

impl From<&str> for GlobalErr {
    // if error is &str, returning Customerr
    fn from(value: &str) -> Self {
        Self::CustomErr(value.to_string())
    }
}
