//tuples
// a type of recorde
//store data anonymously
// no need to name fields
// can be destructured easily into variables
//tuples => (var1,var2,var3,...);
//let (x,y) = (10,44);
//let data = (10,42,43,25,55)

fn getData()->(i32,i32,i32,i32){
    return (10,20,1,24);
}
enum Level{
    low,
    hight
    
}
fn main(){
    let data = getData();
    let (a,b,c,d) = getData();//destructure
    println!("{a}");
    println!("{b}");
    println!("{c}");
    println!("{d}");
println!("another way");
    println!("{:?}",data.0);
    println!("{:?}",data.1);
    println!("{:?}",data.2);
    println!("{:?}",data.3);    
    //destructure
    let (name,level)=("univers",Level::low);
}
