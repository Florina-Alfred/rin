fn takes_slice(slice: &[i32]) {
    println!("Got a slice: {:?}", slice);
}

fn empty_fn_1() {
    println!("Empty function 1");
}

fn empty_fn_2() {
    println!("Empty function 2");
}

fn empty_fn_3() {
    println!("Empty function 3");
}

fn takes_slice_of_fns(slice: &[fn()]) {
    for f in slice {
        f();
    }
}

fn main() {
    println!("Hello, world!");
    let numbers = vec![1, 2, 3, 4, 5];
    let vec_of_fns: Vec<fn()> = vec![empty_fn_1, empty_fn_2, empty_fn_3];
    // let vec_of_fns: Vec<fn()> = vec![empty_fn_1, empty_fn_1, empty_fn_1];
    takes_slice(&numbers);
    takes_slice_of_fns(&vec_of_fns);
}
