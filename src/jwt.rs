use serde::{Serialize, Deserialize};


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Claims {
    pub aud: Option<String>,
    pub exp: usize,
}