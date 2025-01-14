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

    let persons = vec![person1, person2];

    for person in persons {
        person.give_metrics();
    }
}
