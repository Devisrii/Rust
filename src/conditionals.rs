pub fn conditional() {
    let age = 10;
    let check_id: bool = true;

    if age >= 18 && check_id {
        println!("College Students");
    }
    else if age <18 && check_id {
        println!("School Students");
    } else {
        println!("Please show your Id");
    }
//short hand if
    let short = if age >= 18 {true} else {false};
    println!("Short: {}", short);

}