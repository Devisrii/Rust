use std::mem;
pub fn run() {
    let mut array: [i32; 10] = [1,2,3,4,5,6,7,8,9,10];
    //get single value
    println!("{}", array[4]);
    // re-assign value
    array[2] = 0;
    println!("{:?}", array);
    // find length
    println!("{}", array.len());
    //stack allocated memory
    println!("Allocated Memory{}", mem::size_of_val(&array));
    //Get slice
    let slice: &[i32] = &array[1..4];
    println!("Slice {:?}", slice);


}