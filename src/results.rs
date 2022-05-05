pub fn run() {

    //Constant Result type:

    // let x: Result<i32, &str> = Ok(7);
    // println!("okout: {}", x.is_ok());
    
    // let x: Result<i32, &str> = Err("Some error message");
    // println!("okout: {}", x.is_ok());

    // let y: Result<i32, &str> = Ok(2);
    // println!("errorOut: {}", y.is_err());

    // let y: Result<i32, &str> = Err("Something Wrong");
    // println!("errorOut: {}", y.is_err());

//Option Result:

    let x: Result<i32, &str> = Ok(2);
    println!("out: {:?}",x.ok());

    let y: Result<i32, &str> = Err("Error");
    println!("out1: {:?}", y.ok());

    let x: Result<i32, &str> = Ok(4);
    println!("out: {:?}", x.err());

    let y: Result<i32, &str> = Err("Error");
    println!("out1: {:?}", y.err());

    let x: Result<u32, &str> = Ok(4);
    println!("out: {:?}", x.as_ref());

    let y: Result<u32, &str> = Err("err");
    println!("out: {:?}", y.as_ref());

    //asmut():

    // fn mutate(r: &mut Result<i32, i32>) {
    //     match r.as_mut() {
    //         Ok(v) => *v = *v,
    //         Err(e) => *e = 0,
    //     }
    // }
    // let mut x: Result<i32, i32> = Ok(9);
    // mutate(&mut x);
    // println!("{}", x.unwrap());
    // let line = "1\n2\n4\n";

    // for num in line.lines() {
    //     match num.parse::<i32>().map(|i| i * 2) {
    //         Ok(n) => println!("{}", n),
    //         Err(..) => {}
    //     }
    // }
    // Option:
    
    // match is_even(5)  {
    //     Some(data) => {
    //         if data == true {
    //             println!("EvenNumber")
    //         }
    //     }
    //         None => {
    //             println!("Not A EvenNumber")
    //         }
    //     }
    // }
// fn is_even(num: i32) -> Option<bool> {
//     if num % 2 == 0 {
//         Some(true)
//     }
//     else {
//         None
//     }
// }
let before: Option<i32> = Some(2);
let after: Option<String> = before.and_then(|i| {
    if i > 0 {
        Some(i.to_string())
    } else {
        None
    }
});
println!("{:?}",&after);

let before =Some(34);
let after = before.map(|x| x.to_string());
let fin = after.unwrap();


}