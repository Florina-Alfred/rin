use tokio;

pub fn logger(message: String) {
    tokio::spawn(async move {
        println!("{}", message);
    });
}
