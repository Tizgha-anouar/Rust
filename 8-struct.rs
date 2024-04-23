//struct
//data type that contain multiple pieces of data type
//each piece of data called a field
//field with in the struct can be accessing by using a dot (.)

fn main(){
    
struct Car{
    width:i32,
    height:i32,
    weigth:i32
}
let BMW = Car{
    width:100,
    height:50,
    weigth:500
};
println!("the weigth of BMW is => {:?}", BMW.weigth)
}