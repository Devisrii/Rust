pub fn vars() {
    let name = "Devi";
    let mut age = 25;
    println!("Iam {} My age is {}", name, age);
    age = 26;
    println!("Iam {} Now iam {}", name, age);
    // Define Constant
    const DOB_YEAR:i32 = 1996;
    println!("{}", DOB_YEAR);
    const DOB:&str = "April 1996";
    println!("{}", DOB);
    // Assign multiple variables
    let (my_name, my_place) = ("Devi", "Tuticorin");
    println!("{} lives in {}",my_name,my_place);
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error!
    // _immutable_binding += 1;
    // FIXME ^ Comment out this line
    let binding = 2;
    {
        println!("binding:{:?}", binding);
    }
}