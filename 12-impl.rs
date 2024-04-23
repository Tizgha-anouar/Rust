//self=> indicate that we already have obj of struct
//Self=> indicate don't have obj of struct or we already have one
//we can use with enum or struct

struct INFO{
    note:f64,
}
impl INFO {
    // fn display(data:INFO){
    //     println!("note is => {}",data.note);
    // }
    
    //emprovement
    fn display(&self){
        println!("note is => {}",self.note);
    }
    fn setNote() -> Self{
        Self{note:100.00}
    }
    //butif we change the name of Struct we most change it here
    // fn setNote(n:f64)->INFO{
    //     INFO{note:100}
    // }
}

fn main(){
    let univers = INFO{note:12.44};
    // INFO::display(univers);
    
    //emprovement
    univers.display();
    
    let x = INFO::setNote(); // static
    x.display();
    //univers.setNote(); // not work
    


}