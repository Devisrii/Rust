pub fn strs() {
    let mut hello = String::from("Hello ");
    let mut a = String::from("EveryOne!");
    // concotenate
    let add = hello + &a;
    println!("add_out: {}", add);
    // // find length
    // println!("Length:{}", hello.len());
    // // push char
    // hello.push('W');
    // // push string
    // hello.push_str("orld!");
    // // replace
    // let data = hello.replace("Hello","Hii");
    // println!("{}" , data);
    // //empty string
    // let mut empty = String::new();
    // empty.push_str("Hii From Strings");
    // println!("{}",empty);
    // // capacity in bytes
    // println!("capacity: {}", hello.capacity());
    // // check if empty
    // println!("Is_empty: {}", hello.is_empty());
    // //contains
    // println!("Contains: {}", data.contains("Hii"));
    // // split with white space
    // for word in hello.split_whitespace() {
    //     println!("{}", word);
    // }
    // assert_eq!(12,hello.len());
}   