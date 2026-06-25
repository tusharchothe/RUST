fn main() {
//     let  s = String::from("this is length calculation");
//     let len = cal_str_withref(&s);
//     println!("the length of string {s} is {len}");

//     println!();
//     println!();

//     let mut s1 = String::from("Rust is crazy language");
//     let l =cal_mut(&mut  s1);
//     println!("the length of string {s1} is {l}");

       mut_ref();

       //dangling
       let rf_to_nothing = dangle();

}

// implement the calculation 

fn cal_str_withref(s:&String) -> usize{
    let str_len = s.len();
    str_len
}


// same thing with mutable refernce 

fn cal_mut(s1: &mut String) -> usize{
    s1.push_str("Yeahh it is");
    s1.len()
}

fn mut_ref(){
    let mut s :String = String::from("hello");

    let r1 =&s;
    let r2 =&s;
    println!("{r1}, {r2}");
    // r1 and r2 are not used after this point 
    let r3 = &mut s;
    println!("{r3}");
}

// dangling refernces 

fn dangle() -> &String {
    let s =String::from("hello");
    let ref_to_s = &s;
    ref_to_s
}