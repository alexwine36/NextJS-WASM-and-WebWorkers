// #[cfg(target_arch = "wasm32")]
pub mod web;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
