### Using Parameter List (parameter.rs)

1. Creating and Adding Parameters

```rust
let mut parameters = Parameter::new("W_xh".to_string(), Numrs::randn::<f64>(Dimensions::new(constants::d_x, constants::d_h)));    
parameters.add("W_hh".to_string(), None);
parameters.add("W_lh".to_string(), None);
```

2. Forward Iteration Over Parameter List

```rust
// Forward iteration
for node in parameters.iter() {

    println!("Forward Node: {}", node.borrow().get_name());

    if node.borrow().get_data().is_some() {

        let shape = node.borrow().get_data().unwrap().shape.unwrap();
            
        println!("Shape: {}", shape);
    }
}
```

3. Reverse Iteration Over Parameter List

```rust
// Reverse iteration    
let mut iter = parameters.iter();
while let Some(node) = iter.next_back() {

    println!("Reverse: {}", node.borrow().get_name());

    if node.borrow().get_data().is_some() {

        let shape = node.borrow().get_data().unwrap().shape.unwrap();
            
        println!("Shape: {}", shape);
    }
}
```

4. Zig-Zag Traversal of Parameter List

```rust
// Combined (Zig-Zag Traversal)
let mut iter = list.iter();
println!("Zig-Zag:");
println!("{}", iter.next().unwrap().borrow().name);      // "w_xh"
println!("{}", iter.next_back().unwrap().borrow().name); // "w_lh"
println!("{}", iter.next().unwrap().borrow().name);      // "w_hh"
```

5. Handling Edge Cases: Empty and Single Node Lists

```rust
// Edge case: empty list
// Edge case: single node list
```

6. Finding and Accessing a Parameter Node

```rust
// usage of find method
let node = parameters.find("W_xh");

let binding = link.unwrap();
let binding = binding.borrow();
let name = binding.get_name();

let data = binding.get_data();
if let Some(collective) = data {
    if let Some(data) = &collective.data {
        let shape = collective.shape.unwrap();
        //let columns = shape.columns();
        //let rows = shape.rows();
            
        println!("Shape: {}", shape);
                        
        for &value in data.iter() {

            println!("{}", value);
        }                        
    }
}
    
println!("Name: {}", name);    
```