trait Update {
    fn update(&mut self, msg: String);
}

#[derive(Debug, Clone, Default)]
struct MachineStruct {
    message: String,
    count: u32,
}

impl Update for MachineStruct {
    fn update(&mut self, msg: String) {
        self.message = msg;
        self.count += 1;
    }
}

#[derive(Debug, Default, Clone)]
struct UserStruct {
    old: String,
    new: String,
}

impl Update for UserStruct {
    fn update(&mut self, msg: String) {
        self.old = self.new.clone();
        self.new = msg.clone();
    }
}

fn machine_struct_callback(msg: MachineStruct) {
    println!("Calling from machine CB: {:?}", msg);
}

fn user_struct_callback(msg: UserStruct) {
    println!("Calling from user CB: {:?}", msg);
}

fn subscribe<T>(cb: impl Fn(T))
where
    T: Default + Update + Clone, // Use Clone instead of Copy
{
    let mut msg = T::default(); // Create msg outside the loop, so it's reused
    for i in 0..5 {
        println!("Looping: {}", i);
        msg.update(format!("Message {}", i));
        cb(msg.clone()); // Use clone to pass a copy of the current state of msg
    }
}

fn main() {
    subscribe(user_struct_callback);
    subscribe(machine_struct_callback);
}
