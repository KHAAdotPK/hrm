/*
 * implementation/src/parameter.rs
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

/*pub struct ParameterIterator {

    current: Option<ParameterNode>,
}*/

pub struct ParameterIterator {

    front: Option<ParameterNode>, // Tracks forward iteration
    back: Option<ParameterNode>,  // Tracks backward iteration
}

/*impl Iterator for ParameterIterator {

    type Item = ParameterNode;

    fn next(&mut self) -> Option<Self::Item> {

        let current = self.current.take()?; // Take ownership of current node
        let next = current.borrow().next.as_ref().map(Rc::clone);
        self.current = next; // Move to next node

        Some(current) // Return current node
    }
}*/

impl Iterator for ParameterIterator {

    type Item = ParameterNode;

    fn next(&mut self) -> Option<Self::Item> {
        self.front.take().map(|node| {
            // Move forward and update `front`
            self.front = node.borrow().next.as_ref().map(Rc::clone);
            node
        })
    }
}

impl DoubleEndedIterator for ParameterIterator {

    fn next_back(&mut self) -> Option<Self::Item> {
        self.back.take().map(|node| {
            // Move backward and update `back`
            self.back = node.borrow().prev.as_ref().map(Rc::clone);
            node
        })
    }
}

impl Parameter {
    // Creates a new `Parameter` with the given name and optional data.
    // Initializes `next` and `prev` as `None`.    
    pub fn new(name: String, data: Option<Collective<f64>>) -> Parameter {
        Parameter {
            name: name,
            data: data,
            next: None,
            prev: None
        }
    }

    // Returns a reference to the parameter's name.
    pub fn get_name(&self) -> &str {

        &self.name
    }

    // Returns a clone of the parameter's data, if it exists.
    pub fn get_data(&self) -> Option<Collective<f64>> {

        self.data.clone()
    }

    /*pub fn iter(&self) -> ParameterIterator {

        ParameterIterator {

            current: Some(Rc::new(RefCell::new(Parameter {

                name: self.name.clone(),
                data: self.data.clone(),
                next: self.next.clone(),
                prev: None, // Iterator doesn't need prev links

            }))),
        }
    }*/

    // Creates an iterator for traversing the parameter list in both directions.
    // Initializes `front` with a clone of the current node and `back` with the last node.
    pub fn iter(&self) -> ParameterIterator {

        ParameterIterator {
            front: Some(Rc::new(RefCell::new(Parameter {
                name: self.name.clone(),
                data: self.data.clone(),
                next: self.next.clone(),
                prev: None,
            }))),

            back: self.last_node(), // Helper method to find tail
        }
    }
    
    // Finds the last node in the parameter list by traversing `next` links.
    // Returns `None` if no next node exists.
    fn last_node(&self) -> Option<ParameterNode> {

        let mut current = self.next.as_ref().map(Rc::clone)?;

        /*while let Some(next) = current.borrow().next.as_ref().map(Rc::clone) {*/

            /*                
error[E0506]: cannot assign to `current` because it is borrowed
    --> implementation\src\parameter.rs:125:13
     |
123  |         while let Some(next) = current.borrow().next.as_ref().map(Rc::clone) {
     |                                ----------------
     |                                |
     |                                `current` is borrowed here
     |                                a temporary with access to the borrow is created here ...
124  |
125  |             current = next;
     |             ^^^^^^^ `current` is assigned to here but it was already borrowed
126  |         }
     |         - ... and the borrow might be used here, when that temporary is dropped and runs the destructor for type `Ref<'_, Parameter>`
     |
     = note: borrow occurs due to deref coercion to `RefCell<Parameter>`
note: deref defined here

                The error occurs because we're trying to modify current while a borrow from it (current.borrow()) is still active. Let's fix this by restructuring the last_node() method to avoid holding a borrow while reassigning current.
             */
          
            /*current = next;
        }*/

        loop {
            // Temporarily store the next node before moving
            let next = {
                let borrowed = current.borrow();
                borrowed.next.as_ref().map(Rc::clone)
            }; // Borrow ends here
    
            match next {
                Some(next_node) => current = next_node,
                None => break,
            }
        }

        /*
            Ah, the error occurs because the last_node() helper method has a trailing semicolon (;) after Some(current), which discards the value. Let's fix this and ensure the iterator works correctly.
            Some (current);

            If put semicolon after Some(current), it will not compile.
            error[E0308]: mismatched types
            --> implementation\src\parameter.rs:119:28
    |
119 |       fn last_node(&self) -> Option<ParameterNode> {
    |        ---------           ^^^^^^^^^^^^^^^^^^^^^ expected `Option<Rc<RefCell<Parameter>>>`, found `()`
    |        |
    |        implicitly returns `()` as its body has no tail or `return` expression
...
128 |         Some(current);
    |                      - help: remove this semicolon to return this value
    |
    = note:   expected enum `Option<Rc<RefCell<Parameter>>>`
            found unit type `()`
         */
        Some(current)
    }

    
    // Creates a new `ParameterNode` wrapped in `Rc<RefCell<Parameter>>`.
    fn new_node(name: String, data: Option<Collective<f64>>) -> ParameterNode {
        Rc::new(RefCell::new(Parameter::new(name, data)))
    }
    
    // Adds a new parameter node to the end of the list.
    // Updates `next` and `prev` links accordingly.
    pub fn add (&mut self, name: String, data: Option<Collective<f64>>) {
        let new_node = Self::new_node(name, data);
        
        // If this is the first node in the list (next is None)
        if self.next.is_none() {
            new_node.borrow_mut().prev = Some(Rc::new(RefCell::new(Parameter {
                name: self.name.clone(),
                data: self.data.clone(),
                next: Some(Rc::clone(&new_node)),
                prev: None,
            })));
            self.next = Some(new_node);
        } else { // If this is not the first node in the list
            // Find the last node
            let mut current = Rc::clone(self.next.as_ref().unwrap());
            while current.borrow().next.is_some() { // Till last next which is NULL is found
                let next = Rc::clone(current.borrow().next.as_ref().unwrap());
                current = next;
            }
            
            // Link the new node
            new_node.borrow_mut().prev = Some(Rc::clone(&current));
            current.borrow_mut().next = Some(new_node);
        }
    }

    // Finds a parameter node by name in the list.
    // Returns the node wrapped in `Rc<RefCell<Parameter>>` if found, else `None`.
    pub fn find (&self, name: &str) -> Option<ParameterNode> {

        // Check if current node matches
        if self.name == name {

            let self_node = Self::new_node(self.name.clone(), self.data.clone());
            self_node.borrow_mut().next = self.next.clone();
            self_node.borrow_mut().prev = self.prev.clone();

            return Some(self_node);
        }
    
        // Traverse the list
        let mut current = self.next.as_ref().map(Rc::clone)?;
    
        loop {

            // Check if current node matches (without holding a borrow during reassignment)
            
            let borrowed = current.borrow();

            if borrowed.name == name {

                return Some(current.clone());
            }

            // Store next node (if it exists) before releasing the borrow
            let next = borrowed.next.as_ref().map(Rc::clone);

            drop(borrowed); // Explicitly drop borrow,  otherwise compile time error "cannot move out of "borrowed" because it is borrowed"

            match next {

                Some(next_node) => current = next_node, // We we would have not dropped the borrow, we would have not been able to move out of next_node and would have gotton compile time error "cannot move out of "borrowed" because it is borrowed; which is "current" here"
                None => break,
            }            
        }
    
        None
    }
    
    /*
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
    */      
}
