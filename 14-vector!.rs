//vector => can store multi pieces of data
//must be the same type
//use for list of informatiion 
// can add remove traverse the entries

fn main(){
    // let data = vec![1,2,3,4,5,6];
    // println!("{}",data[0]);
    
    let mut my_data = Vec::new();
    my_data.push(10);
    my_data.push(12);
    my_data.push(13);
    my_data.push(14);
    my_data.push(15);
    println!("{}",my_data[0]);
    
    my_data.pop();
    println!("{:?}",my_data);
    
    println!("{}",my_data.len());
    //use for in to iterate through items
    for num in my_data{
        println!("<{:?}>",num);
    }
//if we use my_data after for loop
//we need to use borrow to prevent dead variable
//becouse of memory managment of rust 
}