#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Person {
        // Person { name, age }
        Self { name, age } // same as above
    }
}

// fn main() {
//     let peter = Person::new(String::from("Peter"), 27);
//     println!("{peter:?}");
// }

impl Default for Person {
    fn default() -> Person {
        Person {
            name: "Bot".to_string(),
            age: 0,
        }
    }
}
fn create_default() {
    let tmp = Person {
        ..Default::default()
    };
    let tmp = Person {
        name: "Sam".to_string(),
        ..Default::default()
    };
}

fn main() {
    let peter = create_default();
    println!("{peter:?}", );
    
}