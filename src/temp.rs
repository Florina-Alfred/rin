use metrics_macros::Metrics;

#[allow(dead_code)]
#[derive(Metrics)]
struct Person {
    name: String,
    age: u8,
    location_metric: String,
    position_metric: u8,
    weight_metric: Option<f32>,
    height_metric: Option<f32>,
    is_active: bool,
}

impl Person {
    fn return_name(&self) -> String {
        self.name.clone()
    }
}

#[allow(dead_code)]
#[derive(Metrics)]
struct Child {
    name: String,
    age: u8,
    position_metric: u8,
    weight_metric: Option<f32>,
    height_metric: Option<f32>,
}

impl Child {
    fn return_name(&self) -> String {
        self.name.clone()
    }

    fn return_age(&self) -> u8 {
        self.age
    }
}

// Define a trait for returning name and age, with default methods
trait Returnable {
    fn return_name(&self) -> String;
    fn return_age(&self) -> Option<u8> {
        // Default implementation: return None if the type doesn't have age
        None
    }
}

// Implement Returnable for Person
impl Returnable for Person {
    fn return_name(&self) -> String {
        self.name.clone()
    }
}

// Implement Returnable for Child
impl Returnable for Child {
    fn return_name(&self) -> String {
        self.name.clone()
    }

    fn return_age(&self) -> Option<u8> {
        Some(self.age)
    }
}

// Now the print_details function can call return_name and return_age
fn print_details(human: &dyn Returnable) {
    println!("Name: {}", human.return_name());
    if let Some(age) = human.return_age() {
        println!("Age: {}", age);
    }
}

fn main() {
    let person1 = Person {
        name: String::from("John"),
        age: 25,
        location_metric: String::from("USA"),
        position_metric: 1,
        weight_metric: Some(70.5),
        height_metric: Some(1.75),
        is_active: true,
    };

    let person2 = Person {
        name: String::from("Jane"),
        age: 22,
        location_metric: String::from("UK"),
        position_metric: 2,
        weight_metric: None,
        height_metric: None,
        is_active: false,
    };

    let child1 = Child {
        name: String::from("Tommy"),
        age: 7,
        position_metric: 1,
        weight_metric: Some(30.0),
        height_metric: Some(1.2),
    };

    // Print details for both Person and Child
    let people: Vec<Box<dyn Returnable>> =
        vec![Box::new(person1), Box::new(person2), Box::new(child1)];

    for person in people {
        print_details(&*person);
    }
}

