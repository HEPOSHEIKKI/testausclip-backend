use std::collections::HashMap;

use itertools::Itertools;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub fn generate_token() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(24)
        .map(char::from)
        .collect()
}