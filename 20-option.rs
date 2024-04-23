//option 
//one of two things
//=> data of specified type
//=> nothing
// enum Option<T>{
//     Some(T),
//     None,
// }
struct Customer{
    id:Option<i32>,
    email:String
}
fn main(){
    let c1 = Customer{id:Some(1234),email:"uni@uni.com".to_owned()};
    let c2 = Customer{id:None,email:"uni@anouar.com".to_owned()};

    match c1.id {
        Some(idXXX) => println!("id is => {:?}",idXXX),
        None     => println!("not specify id for this customer"),
        
    }
}