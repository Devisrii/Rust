pub fn run() {
    //print for console
    println!("Hello From Print.rs");
    //Basic Formatting
    println!("Number: {}",1);
    println!("{} is very {}", "Tom", "Intelligent");
    //positional formatting
    println!("{0} is very {1} and {0} likes {2}", "Tom", "Intelligent", "Fruits");
    // Named Arguments
    println!("{Name} likes {fruits}", 
    Name = "sai", fruits = "mango");
    // placeholder traits
    println!("Binary: {:b}, Hexa: {:x}, Octal: {:o}", 10,10,10);
    //placeholder debug traits
    println!("{:?}",(12, "hai", true));
    //basic math
    println!("{}", 10+10);

}