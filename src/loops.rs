pub fn loops() {
    let mut count = 0;
		let mut array: Vec<i32> = vec![];
		let mut array1: Vec<i32> = vec![];
		let mut array2: Vec<i32> = vec![];
    // infinite loop
    loop {
			count += 1;
			println!("Numbers: {}", count);

			if count == 20 {
					break;
			}
    }
		// for loop
		for x in 1..100 {
			if x < 20 {
				array.push(x);
				println!("array: {}",x);
			} else if x >20 && x <50 {
				array1.push(x);
				println!("array1: {}", x);
			} else {
				array2.push(x);
				println!("array2: {}", x);
			}
		}
		//While loop
		while count <= 100 {
			if count % 15 == 0 {
				println!("fifteen {}", count);
			} else if count % 5 == 0 {
				println!("five {}", count);
			} else if count % 3 == 0 {
				println!("three {}", count);
			} else {
				println!("remaining {}", count);
			}
			count += 1;
		}
}