fn main(){
    let mut s = String::from("hello world");
    let res = first_word(&s);
    s.clear();
    let s =String::from("ye nayi string hai");
    println!("{res} and {s}");

    println!(" ");
    slice_str();
    println!(" ");

    let mut s1 = String::from("hello world");
    let rest = first_word_slice(&s1);
    //s1.clear();
    let s1 =String::from("ye nayi string hai");
    println!("{rest} and {s1}");

}

fn first_word(input:&String) -> usize{
    let s =input.as_bytes();

    for (i, &item) in s.iter().enumerate(){
        if item ==b' ' {
            return i;
        }
        
    }
    s.len()
}

// slicing
fn slice_str(){
    let s =String::from("hello world");
    let hello =&s[0..5];
    let world =&s[6..11];

    println!("HELLO :{hello}");
    println!("WORLD :{world}");

}

// corrected code after slicing 

fn first_word_slice(input:&str) -> &str {
    let s =input.as_bytes();

    for (i, &item) in s.iter().enumerate(){
        if item ==b' ' {
            return &input[..i];
        }
        
    }
    &input[..]
}