pub fn create_node() {
    use routing::Node;

    let min_section_size = 8;
    let node = Node::builder().create(min_section_size).unwrap();
}

// Client Creation
pub fn new_client() {    
    use std::sync::mpsc;
    use routing::{Client, Event, FullId};

    let (sender, receiver) = mpsc::channel::<Event>();
    let full_id = FullId::new(); // Generate new keys.
    let client = Client::new(sender, Some(full_id)).unwrap();
    println!("Client created successfully.");
}