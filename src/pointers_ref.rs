pub fn pointer() {
    let arr = [1,2,3,4,5];
    let arr1 = arr;
    println!("values: {:?}", (arr,arr1));

    let vector = vec![6, 7, 8, 9, 10];
    let vector1 = &vector;
    println!("values: {:?}", (&vector,vector1));
}