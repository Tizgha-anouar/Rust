//enum key word
//enum = enumeration
//data can be one of multiple possibilities
//each possibility call variant
enum Position {
    top,
    left,
    right,
    bottom
}
fn setPosition(p:Position){
    match p{
        Position::top=>{println!("top")},
        Position::left=>{println!("left")},
        Position::right=>{println!("right")},
        Position::bottom=>{println!("bottom")}
    }
}
fn main(){
    setPosition(Position::top)
}