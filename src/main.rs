/*
 * src/main.rs
 * Q@khaa.pk
 */

#![allow(non_snake_case)]

//use std::{cell::RefCell, fs::{File, metadata}, io::Read, io::Write, path::Path, path::PathBuf, rc::Rc, str};

use argsv::{common_argc, find_arg, help, help_line, process_argument, start, stop, COMMANDLINES, PCLA};
use numrs::{dimensions::Dimensions, collective::Collective, num::Numrs};
use hrm::{constants, utility::generate_hrm_training_data, hrm::hrm_model, parameter::Parameter};

fn main() {

    let command_lines = "h -h help --help ? /? (Displays help screen)\n\
                         v -v version --version /v (Displays version number)\n\
                         t -t traverse --traverse /t (Traverses PNG file structure and displays it)\n\
                         d -d delete --delete /d (Deletes the named chunk from the PNG file)\n\
                         verbose --verbose (Displays detailed information or feedback about the execution of another command)\n\
                         suffix --suffix (Suffix for the output PNG file)\n";

    let arg_suffix: *mut COMMANDLINES;

    //let mut suffix_token: Option<&str> = Some(constants::PNG_OUTPUT_FILE_SUFFIX);                 

    // Get the command-line arguments as an iterator
    let args: Vec<String> = std::env::args().collect();
    // Create a Vec<CString> from the Vec<String>
    let c_args: Vec<std::ffi::CString> = args.iter().map(|s| std::ffi::CString::new(s.as_str()).expect("Failed to create CString")).collect();
    // Get the equivalent of argv in C. Create a Vec<*const c_char> from the Vec<CString>.
    let c_argv: Vec<*const std::os::raw::c_char> = c_args.iter().map(|s| s.as_ptr()).collect();
    // Get the C equivalent of argc.    
    let argc: i32 = c_args.len() as std::os::raw::c_int;

    let mut ncommon: i32 = 0;

    let head = start (argc, c_argv.clone(), command_lines);
        
        ncommon = common_argc (head);

        let arg_help = find_arg (head, command_lines, "h");
        if !arg_help.is_null() || argc < 1 {
            
            help (head, command_lines);
            stop (head); 

            return;
        }
        
        arg_suffix = find_arg (head, command_lines, "--suffix");
        if !arg_suffix.is_null() {

            let arg_suffix_clap: *mut PCLA = unsafe {(*arg_suffix).get_clap()};
            if unsafe{(*arg_suffix_clap).get_argc()} > 0 {

            } 
        } 
        
    stop (head); 
               
    // for loop with range
    for i in 1..ncommon {

        let arg = &args[i as usize];
        
        println!("arg is: {}", arg);
    }

    let mut x_train: Option<Collective<f64>> = None; // Input sequence
    let mut y_train: Option<Collective<i32>> = None; // Target sequence
    let mut x_val: Option<Collective<f64>> = None; // The validation input sequence
    let mut y_val: Option<Collective<i32>> = None; // The validation target sequence

    generate_hrm_training_data::<f64>(&mut x_train, &mut y_train, &mut x_val, &mut y_val);

    /*let mut dim = Dimensions::new(constants::batch_size, constants::seq_len); // 100 elements
    let x_train_l: Option<Collective<f64>> = Numrs::randn::<f64>(dim);
    dim = Dimensions::new(constants::batch_size, constants::seq_len);
    let y_train_l: Option<Collective<f64>> = Numrs::randn::<f64>(dim);*/

    if let Some(collective) = y_val {

        if let Some(data) = &collective.data {

            let shape = collective.shape.unwrap();

            println!("Shape: {}", shape);
            
            for &value in data.iter() {

                //println!("{}", value);
            }                        
        }
    }

    //let mut parameters: Vec<parameter> = Vec::new();
    
    let mut parameters = Parameter::new(constants::PARAMETER_LIST_EMPTY_MARKER.to_string(), None);    
    let model = hrm_model::new(constants::d_x, constants::d_h, constants::d_l, constants::d_y);
    parameters.add("W_xh".to_string(), Numrs::randn::<f64>(Dimensions::new(constants::d_x, constants::d_h)));
    parameters.add("W_hh".to_string(), None);
    parameters.add("W_lh".to_string(), None);

    parameters.traverse();

    let link = parameters.find("W_xh".to_string());

    let binding = link.unwrap();
    let binding = binding.borrow();
    let name = binding.get_name();

    let data = binding.get_data();
    if let Some(collective) = data {

        if let Some(data) = &collective.data {

            let shape = collective.shape.unwrap();

            println!("Shape: {}", shape);
            
            for &value in data.iter() {

                println!("{}", value);
            }                        
        }
    }
    
    println!("Name: {}", name);    
} 
