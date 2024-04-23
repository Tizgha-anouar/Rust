//enum contain data
//data can also contain enum

enum MOUSE{
    left,
    right,
    middle,
    Scrol(i32),
    Move(i32,i32)
}
enum ELEMENT{
    name:String,
    event(MOUSE),
}
enum DISCOUNT{
    Percent(f64),
    Custom(String),
    Flat(i32)
}