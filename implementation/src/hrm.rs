/*
 * implementation/src/model.rs
 * Q@khaa.pk
 * https://arxiv.org/abs/2506.21734
 */

use numrs::{dimensions::Dimensions, collective::Collective, num::Numrs}; 
use crate::parameter::Parameter;

pub struct hrm_model {
    
}

impl hrm_model {
    
    pub fn new(d_x: usize, d_h: usize, d_l: usize, d_y: i32, parameters: &mut Parameter) -> hrm_model {

        println!("HRM Model created");
        
        //parameters.add("soni".to_string(), Some(Numrs::randn::<f64>(Dimensions::new(d_x, d_h)).unwrap() * 0.01));

        parameters.add("soni".to_string(), Some(Numrs::randn::<f64>(Dimensions::new(d_x, d_h)) * 0.01));

        hrm_model {
            
        }
    }
}