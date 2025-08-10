/*
 * constants.rs
 * Q@khaa.pk
 */

/*
# Hyperparameters
d_x = 256  # Input dimension
d_h = 512  # HLM hidden dimension
d_l = 256  # LLM hidden dimension
d_y = 10   # Output dimension (number of classes)
k = 5      # HLM update frequency
seq_len = 20  # Sequence length
batch_size = 32
num_epochs = 10
learning_rate = 0.001
 */ 

pub const d_x: usize = 256; 
pub const d_h: usize = 512; 
pub const d_l: usize = 256; 
pub const d_y: i32 = 10; 
pub const k: usize = 5; 
pub const seq_len: usize = 20; 
pub const batch_size: usize = 32; 
pub const num_epochs: usize = 10; 
pub const learning_rate: f64 = 0.001; 
