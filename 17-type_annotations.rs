//most the time is optional
//required for fonction signatures 
//user can specified the data type  ==> explicit type annotation
fn display(age:i32,name:&str){
println!("{:?} {:?}",name,age);
}
enum POSITION{
    top,
    bottom
}

fn main(){
let x: i32  = 12;
let y: f64  = 99.99;
let c: char = 'C'; 
let p: POSITION = POSITION::top;
//generac 
let items_int : Vec<i32> = vec![1,2,3];
let items_char : Vec<char> = vec!['a','b','c'];
    display(12,"univers");
}