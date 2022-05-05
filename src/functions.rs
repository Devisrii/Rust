pub fn function() {
   intro("Hii", "sowmi");
   let sum = add(8, 9);
   println!("Sum {}", sum);
}
fn intro (greet: &str, name: &str) {
    println!("{} {} Nice to meet you", greet, name);
}
fn add (n1: i32, n2: i32) ->i32 {
    n1+n2
}