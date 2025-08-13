/*
 * implementation/parameter.rs
 * Q@khaa.pk
 */

use std::{cell::RefCell, rc::Rc};
use numrs::{dimensions::Dimensions, collective::Collective, num::Numrs}; 
use crate::constants;

pub struct Parameter {
    
    name: String,
    data: Option<Collective<f64>>,
    next: Option<Rc<RefCell<Parameter>>>,
    prev: Option<Rc<RefCell<Parameter>>>    
}

// Type alias for cleaner code
type ParameterNode = Rc<RefCell<Parameter>>;

impl Parameter {
        
    pub fn new(name: String, data: Option<Collective<f64>>) -> Parameter {
        Parameter {
            name: name,
            data: data,
            next: None,
            prev: None
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    // Create a new node wrapped in Rc<RefCell<>>
    fn new_node(name: String) -> ParameterNode {
        Rc::new(RefCell::new(Parameter::new(name, None)))
    }

    pub fn add (&mut self, name: String) {
        
        if self.name == constants::PARAMETER_LIST_EMPTY_MARKER { // Empty list
            
            //println!("Empty list");

            self.name = name;
        }
        else { // Non-empty list and it can be a doubly linked list with just one node
            
            // If this is the first real node (no next), create the linked structure
            if self.next.is_none() {
                
                //println!("First real node");

                let new_node = Self::new_node(name);

                let self_rc = Rc::new(RefCell::new(Parameter {
                    name: self.name.clone(),
                    data: self.data.clone(),
                    next: Some(Rc::clone(&new_node)),
                    prev: None
                }));

                // Set up the bidirectional link
                new_node.borrow_mut().prev = Some(Rc::clone(&self_rc));
                
                // Update self to point to the new node
                self.next = Some(new_node);
            } 
            else { // Traverse to find the last node
                
                let mut current = Rc::clone(self.next.as_ref().unwrap());

                loop {
                    let next_exists = current.borrow().next.is_some();
                    if !next_exists {
                        break;
                    }
                    let next = Rc::clone(current.borrow().next.as_ref().unwrap());
                    current = next;
                }

                // Create new node and link it
                let new_node = Self::new_node(name);
                new_node.borrow_mut().prev = Some(Rc::clone(&current));
                current.borrow_mut().next = Some(new_node);

                //println!("New node added");
            }            
        }
    }
       
    pub fn find (self, name: String) -> Option<Rc<RefCell<Parameter>>> {

        if self.name == constants::PARAMETER_LIST_EMPTY_MARKER { // Empty list
                                    
            return None;            
        }

        if self.name == name {

            // Create a copy of self and return it wrapped in Rc<RefCell<>>
            let self_copy = Parameter {
                name: self.name.clone(),
                data: self.data.clone(),
                next: self.next.clone(), // This clones the Rc, not the data
                prev: self.prev.clone(), // This clones the Rc, not the data
            };
            return Some(Rc::new(RefCell::new(self_copy)));

            //return Some(Rc::new(RefCell::new(self.clone())));
        }

        let mut current = Rc::clone(self.next.as_ref().unwrap());

        if current.borrow().name == name {

            return  Some(current.clone());
        }

        loop {

            let next_exists = current.borrow().next.is_some();
            if !next_exists {

                break;
            }
            
            let next = Rc::clone(current.borrow().next.as_ref().unwrap());
            
            if next.borrow().name == name {

                return Some(next.clone());
            }

            current = next;
        } 
     
        None
    }

    pub fn traverse(&self) {
        
        if self.name == constants::PARAMETER_LIST_EMPTY_MARKER { // Empty list
                                    
            return;            
        }

        println!("First real node {}", self.name);
                           
        let mut current = Rc::clone(self.next.as_ref().unwrap());

        println!("Current node {}", current.borrow().name);

        loop {

            let next_exists = current.borrow().next.is_some();
            if !next_exists {

                break;
            }
            
            let next = Rc::clone(current.borrow().next.as_ref().unwrap());

            println!("Next node {}", next.borrow().name);

            current = next;
        }                                
    }
}
