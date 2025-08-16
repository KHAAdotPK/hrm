/*
 * implementation/src/constants.rs
 * Q@khaa.pk
 */

pub const PARAMETER_LIST_EMPTY_MARKER: &str = "PARAMETER_LIST_EMPTY_MARKER";

pub const d_x: usize = 256; // Input dimension
pub const d_h: usize = 512; // HLM hidden dimension
pub const d_l: usize = 256; // LLM hidden dimension
pub const d_y: i32 = 10; // Output dimension (number of classes)
pub const k: usize = 5; // HLM update frequency
pub const seq_len: usize = 20; // Sequence length
pub const batch_size: usize = 32; // Batch size
pub const num_epochs: usize = 10; // Number of epochs
pub const learning_rate: f64 = 0.001; // Learning rate
