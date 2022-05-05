pub fn run() {
    //default "i32"
    let x = 1;
    // default "f64"
    let y = 2.5;
    // large number
    let z:i64 = 67890213;
    //find max number
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    //boolean 
    let is_active: bool = true;
    let is_greater: bool = 10 > 15;
    let face = '\u{20B9}';
    println!("{:?}", (x, y, z, is_active, is_greater, face));
    let sales_man = ["101", "102", "101"];
    let saleman = sales_man[0];
        for sman in sales_man {
            if saleman == sman {
                println!("{:?}", saleman);
            }
        }
            
}    