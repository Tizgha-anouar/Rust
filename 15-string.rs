//string in rust => exist two type:
//String => owned
//&str => borrowed String slice
//with struct we use owned type
//with function use &str when passing string
struct PERSON {
    //name:&str,
    //compiler error becouse of memory managemen
    //in the end od scope the name can not be deleted 
    //person not the owner of &str
    name:String
}
fn display(text:&str){
println!("{}",text);
}

fn main(){
display("my name is tizgha anouar !");
let  owned_string = "owned string";
//let  owned_string = "owned string 2".to_owned(); 
//let  owned_string = String::from("owned string 3");
//let  owned_string = "owned string 3.to_string();
display(&owned_string);
let univers = PERSON{name : "univers".to_string()};
println!("my name is => {}",univers.name);
    
}