use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Guess!!");
    let secrete_num = rand::thread_rng().gen_range(1..=100);
    loop{
    

    //println!("the secrete number is {}",secrete_num);

    println!("please enter your guess:");

    let mut guess =String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    // SHADOWING THE VAR AND PRASE
    
    //let guess:u32 = guess.trim().parse().expect("please give a valid input ");

    let guess:u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };


    println!("YOU GUESSED :{}",guess);

    //match 
    match guess.cmp(&secrete_num){
        Ordering::Less => println!("small"),
        Ordering::Greater => println!("big"),
        Ordering::Equal => {
            println!("YOU WIN !");
            break;
        }  
      }
    }
}
