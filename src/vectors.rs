// use std::mem;
#[derive(Debug)]
pub struct FruitProp {
	pub pname: String,
}


#[derive(Debug)]
pub struct Fruit {
	pub name: String,
	color: String,
	// pub prop: FruitProp,
}

// impl Fruit {
// 	pub fn new(name: String)
// }
pub fn run() {
    //forloop using string
		let f1 = Fruit {
			name: "apple".to_string(),
			color: "blue".to_string(),
		};
		let f2 = Fruit {
			name: "banana".to_string(),
			color: "red".to_string(),
		};
		let frts = vec![f1, f2];
		println!("{:?}", frts);
    let fruits = vec!["apple", "banana", "cherry", "grapes", "jack_fruit", "lemon", "mango", "orange", "pinapple"];
    // let mut liked_fruits = vec![String::from("apple"),String::from("mango"),String::from("banana")];//changing one value
    let  liked_fruits = vec!["apple","mango","banana"];
    let mut fin_fruit = vec![];
    for l in &liked_fruits {
        if fruits.contains(l) {
            fin_fruit.push(l);
        }
    }
    println!("out: {:?}", fin_fruit);
    //changing one value
    // for l in &mut liked_fruits {
    //     if l == "banana" {
    //         l.push_str("red");
    //         fin_fruit.push(l);
    //     }
    // }
    // println!("out: {:?}", fin_fruit);
    
    // using integers
    // let mut vector: Vec<i32> = vec![5; 10];
    // // get single value
    // let single = &vector[0];
    // vector.push(15);
    // vector[0] = vector[1] + vector[2]; 
    // vector[1] = vector[0] + 2;
    // vector[2] = vector[0] + 4;
    // println!("vector {:?}", vector);
    // for x in &mut vector {
    //     *x += 5;
    //     println!("loop_out: {}", x)
    // }

    // re-assign value
    // vector[2] = 0;
    // fruits[0] = "Grapes";
    // // Add on vector
    // vector.push(11);
    // vector.push(12);
    // fruits.push("pinapple");
    // // pop off the last value
    // vector.pop();
    // println!("{:?}", vector);
    // println!("{:?}", fruits);
    // // find length
    // println!("{}", vector.len());
    // println!("{}", fruits.len());
    // //stack allocated memory
    // println!("Allocated Memory{}", mem::size_of_val(&vector));
    // //Get slice
    // let slice: &[i32] = &vector[1..4];
    // println!("Slice {:?}", slice);
    //Loop through vector values
    // for x in vector.iter() {
    //     println!("Numbers: {}", x);
    // }
    // for x in vector.iter_mut() {
    //     *x += 2;
    // }
    // println!("vector Vec: {:?}", vector);
  
}