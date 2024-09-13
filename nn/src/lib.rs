#[macro-use]
extern crate derive_builder;
pub mod netowork;
pub mod activations;

pub mod matrix{
    pub use matrix::matrix::Matrix;
}