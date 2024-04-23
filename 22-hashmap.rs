use std::collections::HashMap;

// collection that stores data as key-value pairs
// => data is located using the keydata is the value
// =>    key => value
//     location => data
// hashmap = dictionary
fn main(){
let mut people = HashMap::new();
people.insert("univers",21);
people.insert("uni",29);
people.insert("light",31);
people.insert("L",12);

match people.get("light") {
    Some(age) => println!(" light => age  is {}",age),
    _ => println!("not found"),
 }
for (name , age) in people.iter(){
    println!(" {} => age  is {}",name,age);
}
for person in people.keys(){
    println!(" {}", person);
}
for value in people.values(){
      println!(" {} ", value);
}
}
