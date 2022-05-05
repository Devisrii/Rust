// use crate::iter;
#[derive(Debug, Clone)]

struct Student {
    name: String,
    age: usize,
    mark: usize,
}

pub fn run() {
    //array
    let arr = [1,2,3,4,5,6,7,8];
    let mut iter = arr.iter();
    println!("out: {:?}", iter);
    println!("out: {:?}", iter.next());
    println!("lastOut: {:?}", iter.last());

    //vector
    let vector: Vec<usize> = vec![1,2,3,4,5,6,7,8,9];
    let mut iter = vector.iter();
    println!("vecOut: {:?}", iter);
    println!("vecNext: {:?}", iter.next());
    println!("vecLast: {:?}", iter.last());
    let iter1 = (0..10).filter(|x| x % 2 == 0).chain(15..20);
    println!(" iter1: {:?}", iter1);
    println!("count: {}",vector.iter().count());

    //filter:
    let a = vec![-5,-4,-3,0,1,2,3,4,5];
    let a1:Vec<i32>  = a.into_iter().filter(|&x| x < 1).collect();
    println!("filterOut: {:?}", &a1);

    let a2:Vec<i32> = a1.into_iter().filter(|x| x.is_negative()).collect();
    println!("a2Out: {:?}", &a2);

    //find
    let b = vec![2,4,5,6,7,8];
    let b1 = b.into_iter().find(|&x| x == 10);
    println!("findOut: {:?}", &b1);

    //map

    let c = vec!["hi".to_string(), "hello".to_string()];
    let c1: Vec<String> = c.iter().map(|x| x.to_uppercase()).collect();
    println!("mapOut: {:?}", &c1);

    let d = vec![1,2,3,4,5];
    let d1: Vec<i32> = d.iter().map(|y| y*2).collect();
    println!("int: {:?}", &d1);
    let d2  = d.iter().max();
    println!("minimum: {:?}", &d2);

    let strngs = Some(vec!["hi".to_string(), "hello".to_string()]);
    let strng1 = strngs.iter();
    println!("out: {:?}", &strng1);


    // using struct in vector:

    let student1 = Student {
        name: "Aaron".to_string(),
        age: 6,
        mark: 97,
    };
    let student2 = Student {
        name: "Kavya".to_string(),
        age: 5,
        mark: 89,
    };
    let student3 = Student {
        name: "Saran".to_string(),
        age: 6,
        mark: 98,
    };

    let vector: Vec<Student> = vec![student1, student2, student3];

    let filter: Vec<Student> = vector.clone().into_iter().filter(|x| x.age == 6).collect();
    let map: Vec<usize>  = vector.clone().iter().map(|x| x.mark + 2).collect();
    let map1: Vec<String> = vector.clone().iter().map(|x| x.name.to_uppercase()).collect();
    let find = vector.iter().find(|x| x.name == "Saran".to_string());
    // let name: Vec<String> = student1.name.clone().unwrap_or_default();
    println!("mapOut: {:?}", &map);
    println!("filterOut: {:#?}", &filter);
    println!("findOut: {:?}", &find);
    println!("map1Out: {:?}", &map1);
    println!("{:#?}", vector);


}