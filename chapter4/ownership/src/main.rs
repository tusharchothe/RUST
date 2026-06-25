fn main() {
    //let s = "Hello world from s !!";

    // {
    //     let x = "hello from x !!";
    // }// x ka scop ends

    //println!("{x}"); this gives error becauz variable is outoff scope

    // println!("{s}");

    // let mut b = String::from("hello world");
    // b.push_str("from the b");
    // println!("{b}");


    // memory and allocation
    //sample();
    // let mut x = add(5);
    // println!("{x}");

    // let name =String::from("Tushar Chothe");
    // takes_ownership(name);

    //println!("the name is {name}");// gives error of borrowed value 
    // let s = gives_ownership();
    // println!("s is {s}");

    // let s = give_and_take_back(s);
    // println!{"s2 is {s}"};

    let str = String::from("tushar Chothe");
    let (str,length) = cal_str_length(s);
    println!("The len of {s} is {length}");

}

fn sample(){
    let mut x =32;
    let y =x;
    x=10;
    println!("the X is {} and Y is {}",x,y);

}


fn sample_str(){
    let s1:String=String::from("im tushar");

    let s2=s1.clone();

    println!("the s1 is {s1}");
}

// ownership and function
fn add(x:i32)->i32{
    return x+10;
}

fn takes_ownership(s:String){
    println!("inside ownership {s}");
}

fn gives_ownership()->String{
    let s =String::from("gives the  ownership");
    s
}
// give and take  back ownership

fn give_and_take_back(s:String)->String{
    println!("s give and takeback th ownership {s}");
    s
}

//calculate the length od string 

fn cal_str_length(s:String)->(String,usize){
    let result = s.len();
    (s,result)
}