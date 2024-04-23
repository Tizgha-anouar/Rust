//derive can add some functionality of core implamentation
//struc or enum
//String not implement Copy
//struct PERSON{
//    name:String,
//    age:i32,
//    id:i32
//}

#[derive(Debug,Clone,Copy)]
struct PERSON{
    age:i32,
    id:i32
}
fn display(data:PERSON){
        println!("{:?}",data);
}

fn main(){
    let data = PERSON{
        age:30,
        id:123456
    };
 display(data);
 display(data);
}
