pub fn tups() {
    let data: (&str, &str, i32) = ("Raju", "Apple", 50);
    println!("{} buy {} {}", data.0, data.2, data.1);
}