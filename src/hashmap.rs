#![allow(unused)]
pub fn main() {
use std::collections::HashMap;

// let mut marks = HashMap::new();
// let maths = String::from("Maths");
// let english = String::from("English");
// let social = String::from("Social");
// marks.insert(&maths, 98);
// marks.insert(&english,90);
// marks.insert(&social, String::from("Good"));

// let mark = marks.get(&social);// we used key here, if that key has some value return that.. otherwise return None
// println!("{:?}", mark);
// for (key, value) in marks {
//   println!("{}", key);  
// }
// let mut students: HashMap<&String, Vec<u32>> = HashMap::new();
// let student1 = String::from("Balu");
// let student2 = String::from("Arul");
// let student3 = String::from("Student3");
// students.insert(&student1, [89, 78, 98, 54, 90].to_vec());
// students.insert(&student2, [69, 80, 78, 52, 40].to_vec());
// println!("{:?}", students);

// for (key, value) in students {
//     println!("{} {:?}", key, value);
//     for val in value {
//         if val>70 {
//         //  println!("{:?}", val);   
//         }
        
//     }
// }
// let mark = students.get(&student3);
// println!("{:?}", mark);


//updating:

// students.entry(&student1).or_insert([76, 78, 89, 64, 56].to_vec());
// students.entry(&student3).or_insert([45, 55, 78, 87, 54].to_vec());
// println!("{:?}", students);

let mut student_list : HashMap<&String, Vec<&str>> = HashMap::new();

    let department = String::from("English");
    let student_name = "Jothi";

    student_list.insert(&department, ["Arul", "Rahul", "Malar"].to_vec());

    match student_list.get_mut(&department){
        Some(val) => {
            val.push(student_name);
        },
        None => {
            student_list.insert(&department, ["Gayu"].to_vec());
        }
    }
    
    student_list.entry(&department).or_insert(["Gayu"].to_vec()).push(student_name);
    println!("{:?}", student_list);

    #[derive(Hash, Eq, PartialEq, Debug)]
    struct Viking {
        name: String,
        country: String,
    } 
    impl Viking {
        fn new(name: &str, country: &str) -> Viking {
            Viking {
                name : name.to_string(),
                country : country.to_string(),
            }
        }
    }
    let vikings = HashMap::from([
    (Viking::new("Sea", "India" ),25),
    (Viking::new("River", "US"),20),
    (Viking::new("Mountain", "China"),23),
    ]);
    for (key, val) in &vikings {
        // println!("{:?} {}",  key, val);
    }
    
    
// let mut book_reviews = HashMap::new();

// // Review some books.
// book_reviews.insert(
//     "Adventures of Huckleberry Finn".to_string(),
//     "My favorite book.".to_string(),
// );
// book_reviews.insert(
//     "Grimms' Fairy Tales".to_string(),
//     "Masterpiece.".to_string(),
// );
// book_reviews.insert(
//     "Pride and Prejudice".to_string(),
//     "Very enjoyable.".to_string(),
// );
// book_reviews.insert(
//     "The Adventures of Sherlock Holmes".to_string(),
//     "Eye lyked it alot.".to_string(),
// );

// Check for a specific one.
// When collections store owned values (String), they can still be
// queried using references (&str).
// if !book_reviews.contains_key("Pride and Prejudice") {
//     // Some(val)
//     // println!("We've got {} reviews, but Les Misérables ain't one.",
//     //          book_reviews.len());
// }
// let to_find = ["Pride and Prejudice", "The Adventures of Sherlock Holmes"];
// for &book in &to_find {
//     match book_reviews.get(book) {
//         Some(review) => println!("{}: {}", book, review),
//         None => println!("{} is unreviewed.", book)
//     }

// }


}