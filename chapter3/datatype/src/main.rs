fn main(){
    let guess :u32="41".parse().expect("Not a number");

    let _a :u8=25;
    let _c :i8=-36;
    //range of i8 is -128 to 128
    // range of u8 is 0 to 255

    let d =36u8;
    let e:u64 =1_00_000; 


    let num:u8=random_number()+100;
    println!("{}",num);

    // above code is panicked at debug mode and run on release mode 

    //for this we need to use wrapping add

    let no:u8=random_number().wrapping_add(56);
    println!("{}",no);

    //checked methods 
    let a:u8=match random_number().checked_add(58){
        Some(num) => num,
        None =>{
            println!("cannot add");
            return
        }
     };
    println!("value of a is {}",a);

    // overflowing add 
    let(a,b):(u8,bool)=random_number().overflowing_add(58);
        
        
    println!("value of a is {} and b is {}",a,b);

    flaoting_num();

    //tup_example();
    array_example();
}

fn random_number()-> u8{
    200
}

fn flaoting_num(){
    let  x =2.0;
    println!("{}",x);
    let y :f32 =3.0;
    println!("{}",y);
}

fn math_op(){
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}

fn tup_example(){
    let tup: (i32, f32, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("the value of y is {}", y);
    println!("the value of x is {}", tup.0); // Changed from tup.0 to x
    println!("the value of z is {}", tup.2); // Changed from tup.2 to z
}


fn array_example(){
    let arr:[i32;5] =[1,2,3,4,5];
    println!("{}",arr);

    //another way 
    let a =[10;5];
    println!("{}",a);

}