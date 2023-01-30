pub fn simple_1(){
    let test=|x|x+1;
    println!("closure_annotated: {}", test(1));
}
pub fn closure_mut(){
    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    inc();
}
pub fn closure_borrow(){
    use std::mem;
    let movable = Box::new(3);
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };
    consume();
}