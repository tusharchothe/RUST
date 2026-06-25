// FUNCTION pratice code 

fn main() {
    println!("Hello, world!");
    //anotherfunction();//function call
    //add_function(25,65);
    //state_exp();
    let mut a = add(10,20);
    println!("{a}");

    let mut b = is_Even(10);
    println!("the value of b is {b}");
}
fn anotherfunction(){
    println!("this is another function");
}
fn add_function(x:i32,y:i32){
    let mut add  = x+y ;
    println!("The addition of {x} and {y} is {add}");
}

// statement and expression
fn state_exp(){
    let y ={
        let x =3;
        x+1
    };
    println!("{y}");
}
//function that return value 

fn add(x:i32,y:i32) ->i32 {
    x+y
}

fn is_Even(x:i32)-> bool{
    if x%2==0{
        return true;
    }
    else{
        return false;
    }
} 