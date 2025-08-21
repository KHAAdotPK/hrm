/*
 * implementation/src/utility.rs
 * Q@khaa.pk
 */

use rand::Rng;
use rand::distributions::{Distribution, Standard}; 

use std::{cell::RefCell, rc::Rc};
use numrs::{dimensions::Dimensions, collective::Collective, num::Numrs};
use crate::constants;

/*
 * Generates synthetic training and validation data for the Hierarchical Reasoning Model (HRM, https://arxiv.org/abs/2506.21734).
 * 
 * This function creates random input and target sequences for both training and validation datasets,
 * suitable for training the HRM, a recurrent neural network with hierarchical modules (High-Level Module
 * and Low-Level Module) designed for sequential reasoning tasks. The input sequences (`x_train` and `x_val`)
 * are generated as 3D tensors with shape [batch_size, seq_len, d_x], where `batch_size` is the number of
 * sequences, `seq_len` is the length of each sequence, and `d_x` is the input dimension (e.g., feature or
 * embedding size). The target sequences (`y_train` and `y_val`) are generated as 2D tensors with shape
 * [batch_size, seq_len], containing random integer labels in the range [0, d_y) for classification tasks.
 * 
 * The data is generated using the `Numrs` library, with `randn` for normally distributed input values and
 * `randint` for integer labels. The dimensions are defined using the `Dimensions` struct, with constants
 * (`batch_size`, `seq_len`, `d_x`, `d_y`) assumed to be defined in a `constants` module.
 * 
 * Arguments:
 * - `x_train`: A mutable reference to an `Option<Collective<E>>` that will store the training input sequences.
 *              On return, it is set to a `Collective` containing a 3D tensor of shape [batch_size, seq_len, d_x]
 *              with normally distributed values.
 * - `y_train`: A mutable reference to an `Option<Collective<i32>>` that will store the training target sequences.
 *              On return, it is set to a `Collective` containing a 2D tensor of shape [batch_size, seq_len]
 *              with random integer labels in [0, d_y).
 * - `x_val`: A mutable reference to an `Option<Collective<E>>` that will store the validation input sequences.
 *            On return, it is set to a `Collective` with the same structure as `x_train`.
 * - `y_val`: A mutable reference to an `Option<Collective<i32>>` that will store the validation target sequences.
 *            On return, it is set to a `Collective` with the same structure as `y_train`.
 * 
 * Type Constraints:
 * - `E`: The element type for input sequences (e.g., `f32` or `f64`), which must implement `Copy`.
 * - `Standard: Distribution<E>`: Ensures `E` supports generation of normally distributed values via `Numrs::randn`.
 * 
 * Notes:
 * - The function assumes constants (`batch_size`, `seq_len`, `d_x`, `d_y`) are defined in a `constants` module.
 * - The generated data is synthetic and intended for testing or demonstration. For real-world tasks, replace
 *   with a dataset-specific loader.
 * - The `Dimensions` struct is used to define tensor shapes, with a linked structure (`set_next`) to represent
 *   3D tensors for inputs.
 */
pub fn generate_hrm_training_data<E>(x_train: &mut Option<Collective<E>>, y_train: &mut Option<Collective<i32>>, x_val: &mut Option<Collective<E>>, y_val: &mut Option<Collective<i32>>)
    where E: Copy,
    Standard: Distribution<E>     
    {
        
    //println!("------>>>>>> {}", x_train.clone().unwrap().shape.unwrap().get_n());

    // 3D input tensors
    let mut dim1 = Dimensions::new(0, constants::batch_size);
    let mut dim2 = Dimensions::new(constants::d_x, constants::seq_len);
    dim1.set_next(Some(Rc::new(RefCell::new(dim2))));

    //println!("---->>>>>>>>> {}", dim1.get_n());
    
    /*if let Some(collective) = x_train.as_mut() {
        if let Some(shape) = collective.shape.as_mut() {
            shape.set_rows(constants::batch_size);
            shape.set_columns(0);
            shape.set_next(None);
            shape.set_prev(None);

            let mut dim = Dimensions::new(constants::d_x, constants::seq_len);

            shape.set_next(Some(Rc::new(RefCell::new(dim))));
            
            println!("------------>>>>>>>    {}", shape.get_n());
            
            // *x_train = Numrs::randn::<E>(*shape.clone());           
        }

        *x_train = Numrs::randn::<E>(*collective.shape.clone().unwrap());
    }*/

    *x_train = Some(Numrs::randn::<E>(dim1.clone()));
    *x_val = Some(Numrs::randn::<E>(dim1.clone()));

    // y = torch.randint(0, d_y, (num_samples, seq_len))  # Random class labels
    // 2D target tensors
    dim1 = Dimensions::new(constants::seq_len, constants::batch_size);

    *y_train = Some(Numrs::randint (0, constants::d_y, dim1.clone()));
    *y_val = Some(Numrs::randint (0, constants::d_y, dim1.clone()));
}