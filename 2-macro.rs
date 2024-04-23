//macros == like normal function but
//macro expand into additional code
//println! display informatin to the terminal
// ! indicat that we use a macro insted of using function

fn main() {
    let name = "univers";
    let code = 101010;
    println!("macro => println!");
    println!("{:?}",name);
    println!("{:?} {:?}",name,code);
    println!("my name is => {:?}",name);
    println!("{name:?}");
    println!("{name}");
    
    
}