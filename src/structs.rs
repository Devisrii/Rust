#[derive(Debug)]
//Traditional Struct
// struct Rectangle {
//     height: usize,
//     width: usize,
// }
struct Student {
    name: String,
    age: i32,
    father_name: String,
    email: String,
    mark: i32
}
impl Student{
  fn get_student(name: String, age:i32, father_name: String, email: String,mark:i32) -> Student {
    Student {
      name: name,
      age: age,
      father_name: father_name,
      email: email,
      mark: mark,
    }
  }
    // fn area(&self) -> usize {
    //   self.height*self.width
    // }
    // fn measure(&self) -> bool {
    //   self.width  > self.height
    // }
    // fn compare(&self, other: &Rectangle) -> bool {
    //   self.width < other.width
    // }
  // fn mark(&mut self) {
  //   self.mark += 5;
  // }
}
//Tuple struct 
// struct Colors (u8,u8,u8);
pub fn run() {
  //  let rect = Rectangle {
  //      height: 25,
  //      width: 40,
  //  };
  //  let rect1 = Rectangle {
  //   height: 20,
  //   width: 30,
  //  };
  //  let rect2 = Rectangle {
  //   height: 40,
  //   width: 60,
  //  };
  //  println!("area: {:?}", rect.area());
  //  println!("measure: {:?}", rect.measure());
  //  println!("compare: {:?}", rect.compare(&rect1));

//    let mut c = Colors (255,0,0);
//    c.0 = 200;
//    println!("Color: {:?}", (c.0, c.1, c.2));

   let mut stdnt = Student {
       name: String::from("Dev"),
       age:14,
       father_name: String::from("Kumar"),
       email: String::from("stdnt@gmail.com"),
       mark: 90
   };
   stdnt.name = String::from("madhav");

   let mut stdnt1 = Student {
     name: String::from("Shiya"),
     age: 15,
     father_name: String::from("Raju"),
     email: String::from("stdnt1@gmailcom"),
     mark: 85
   };
  println!("Student: {:#?}", stdnt);
  // println!("StudentDetails: {:#?}", stdnt.student_details());
  stdnt1.mark;
  // student_details(&mut stdnt);
  println!("output: {:?}", stdnt1.mark);

}