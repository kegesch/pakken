use ast::{Model, Namespace};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub struct Generator<T>
where
    T: Generative,
{
    model: Model<T>,
    target_file: Path,
}

pub trait Generative {}
