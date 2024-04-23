//unction can have:
//name
//parameters with type name:type
//return type use arrow->
//body{}

fn double(x:i32)->i32{
   return x*2;
}
fn print(x:i32){
    println!("{x}");
}

fn main(){
    print(double(4))
}