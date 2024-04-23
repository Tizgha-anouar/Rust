//borrow or ownerShip
//rust use ownership model ti manage the memory
//the owner of memory is reponsible for cleanig up the memory at the end of scope { }
//also moved or borrowed
//to prevent leaks
//use ampersand &  to allow code o borrow  memory


struct INFO{
    age:i32,
    id:i32
}
fn displayInfo(data:&INFO){
    println!("age => {:?}, ID => {:?}",data.age,data.id);
}
fn main(){
    let univers = 100;
    let address = &univers;
    println!("{:p}", &univers); 
    println!("{:p}", address); 
    ////test
    println!("{:?}", address);
    println!("{:?}", univers);
    println!("{:?}", &address);
    ////test
    let data = INFO{age:30,id:12345};
     displayInfo(&data);
     displayInfo(&data);
}