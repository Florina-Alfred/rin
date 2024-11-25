mod args;
mod msg;
mod node;

use args::Args;
use clap::Parser;
use tokio;

fn string_callback(input: String) {
    println!("String callback: {:?}", input);
}

fn int_callback(input: i32) {
    println!("Integer callback: {:?}", input);
}

fn bytes_callback(input: u8) {
    println!("Bytes callback: {:?}", input);
}

fn any_callback(input: &dyn std::any::Any) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(input_str) = input.downcast_ref::<String>() {
        println!("String: {}", input_str);
        let number = input_str.split("-").collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap();
        println!("Number: {}", number);
    } else if let Some(input_int) = input.downcast_ref::<i32>() {
        println!("Integer: {}", input_int);
    } else if let Some(input_float) = input.downcast_ref::<f64>() {
        println!("Float: {}", input_float);
    } else if let Some(input_bool) = input.downcast_ref::<bool>() {
        println!("Boolean: {}", input_bool);
    } else {
        println!("Unknown type {:?}", input);
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    node::subscribe(
        args.key_expr.as_str(),
        args.mode.as_str(),
        args.endpoints.iter().map(|x| x.as_str()).collect(),
        // std::any::TypeId::of::<i32>(),
        // node::CallbackInput::TypeInt,
        // int_callback,
        // node::CallbackInput::TypeString,
        std::any::TypeId::of::<String>(),
        // std::string::String::from("TypeString"),
        string_callback,
    )
    .await;
}

// use std::any::{Any, TypeId};
//
// // Function 1: string_callback
// fn string_callback(input: String) -> Result<String, String> {
//     if input.is_empty() {
//         Err("String cannot be empty".to_string())
//     } else {
//         Ok(format!("Received string: {}", input))
//     }
// }
//
// // Function 2: int_callback
// fn int_callback(input: i32) -> Result<i32, String> {
//     if input < 0 {
//         Err("Number cannot be negative".to_string())
//     } else {
//         Ok(input * 2)
//     }
// }
//
// // Function 3: bytes_callback
// fn bytes_callback(input: u8) -> Result<u8, String> {
//     if input == 0 {
//         Err("Byte cannot be zero".to_string())
//     } else {
//         Ok(input + 1)
//     }
// }
//
// // The generic callback function
// fn callback<F, R>(type_id: TypeId, func: F) -> Result<R, String>
// where
//     F: Fn(Box<dyn Any>) -> Result<R, String>,
// {
//     // Generate input based on the type_id
//     let input: Box<dyn Any> = match type_id {
//         t if t == TypeId::of::<String>() => Box::new("Generated String".to_string()),
//         t if t == TypeId::of::<i32>() => Box::new(42),
//         t if t == TypeId::of::<u8>() => Box::new(8u8),
//         _ => return Err("Unsupported type".to_string()),
//     };
//
//     // Call the function with the generated input
//     func(input)
// }
//
// fn main() {
//     // Calling callback with string_callback and String type
//     let result_string: Result<String, String> = callback(TypeId::of::<String>(), |input| {
//         if let Some(input_string) = input.downcast_ref::<String>() {
//             string_callback(input_string.clone()) // Call string_callback with the correct input
//         } else {
//             Err("Type mismatch".to_string())
//         }
//     });
//     match result_string {
//         Ok(value) => println!("Success: {}", value),
//         Err(err) => println!("Error: {}", err),
//     }
//
//     // Calling callback with int_callback and i32 type
//     let result_int: Result<i32, String> = callback(TypeId::of::<i32>(), |input| {
//         if let Some(input_int) = input.downcast_ref::<i32>() {
//             int_callback(*input_int) // Call int_callback with the correct input
//         } else {
//             Err("Type mismatch".to_string())
//         }
//     });
//     match result_int {
//         Ok(value) => println!("Success: {}", value),
//         Err(err) => println!("Error: {}", err),
//     }
//
//     // Calling callback with bytes_callback and u8 type
//     let result_bytes: Result<u8, String> = callback(TypeId::of::<u8>(), |input| {
//         if let Some(input_byte) = input.downcast_ref::<u8>() {
//             bytes_callback(*input_byte) // Call bytes_callback with the correct input
//         } else {
//             Err("Type mismatch".to_string())
//         }
//     });
//     match result_bytes {
//         Ok(value) => println!("Success: {}", value),
//         Err(err) => println!("Error: {}", err),
//     }
//
//     // Calling callback with unsupported type
//     let result_unsupported: Result<String, String> = callback(TypeId::of::<f64>(), |input| {
//         Err("Unsupported type".to_string())
//     });
//     match result_unsupported {
//         Ok(value) => println!("Success: {}", value),
//         Err(err) => println!("Error: {}", err),
//     }
// }
