struct Person {
    name: String,
    age: u8,
    location_metric: String,
    position_metric: u8,
    weight_metric: Option<f32>,
    height_metric: Option<f32>,
}

fn print_metrics(person: Person) {
    // We can manually check for each field with "_metric" in the name and print it.
    if person.location_metric.contains("_metric") {
        println!("Location Metric: {}", person.location_metric);
    }

    if person.position_metric.to_string().contains("_metric") {
        println!("Position Metric: {}", person.position_metric);
    }

    if let Some(weight) = person.weight_metric {
        println!("Weight Metric: {}", weight);
    }

    if let Some(height) = person.height_metric {
        println!("Height Metric: {}", height);
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
    };

    let person2 = Person {
        name: String::from("Jane"),
        age: 22,
        location_metric: String::from("UK"),
        position_metric: 2,
        weight_metric: None,
        height_metric: None,
    };

    let persons = vec![person1, person2];

    for person in persons {
        print_metrics(person);
    }
}

