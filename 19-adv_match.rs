enum PERSON{
    Age(i32),
    Salary(i32),
}
struct TICHET{
    price:i32,
    obj:String,
}
fn main(){
/////example 1
    let x = 3;
    match x {
        5 => println!("{}",x),
        other => println!("other {}",other),
    }
/////example 2

let x = PERSON::Salary(10340);
    match x {
          PERSON::Salary(50000) => println!("salary 50000"),    
          PERSON::Salary(amount) => println!("{}",amount),
          _ => (),
    }
////example 3
let obj1 = TICHET{price:100,obj:"obj1".to_string()};
match obj1 {
    TICHET {price:100,obj} => println!("price is 100 => {}",obj),
    TICHET {price,..} => println!("ticket => price => {}",price),
}

}