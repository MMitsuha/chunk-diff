mod changed;

pub use changed::Changed;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Frame {
    Changed(Changed),
}
