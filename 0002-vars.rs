fn main(){
    let name: &str = "guys";
    let num: i8 = 3;
    // num = 5;
    // ^^^^^^^ cannot assign twice to immutable variable
    let mut percent: f32 = 100.0;
    println!("Hello to you {:?} {:?}", num, name);
    println!("I'm {:?}% you know this...", percent);
    percent = 90.5;
    let boolean: bool = true;
    println!("{:?}% of users don't use j key to go down... {:?}", percent, boolean)
}
