use std::io;

fn main() {
    //sample_ifelse();
    //sample_loop();
    //r_loop();
    //loop_lable();
    //loop_while();
    //loop_arr();
    //for_loop();
    for_range();
    
}

fn sample_ifelse(){
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut num3 = String::new();

    println!("Enter the num 1:");
    io::stdin().read_line(&mut num1).expect("failed to read line");
    let num1:i32 =num1.trim().parse().expect("please type a number");

    println!("Enter the num 2:");
    io::stdin().read_line(&mut num2).expect("failed to read line");
    let num2:i32 =num2.trim().parse().expect("please type a number");

    println!("Enter the num 3:");
    io::stdin().read_line(&mut num3).expect("failed to read line");
    let num3:i32 =num3.trim().parse().expect("please type a number");


    if num1 > num2 && num1> num3{
        println!("NUM1 is greatest");
    }
    else if  num2 >num1 && num2>num3{
        println!("NUM2 is greatest");
    }
    else if num3 >num1 && num3>num2{
        println!("NUM3 is greatest");
    }
    else{
        println!("two or more number are equal");
    }
}

// loop

fn sample_loop(){
    let mut num =0;
    loop{
        num = num +1;

        if num == 5{
            // increment karna padega num
            //num = num+1;
            continue;
        }
        println!("the value of number is {num}");

        

        if num == 10{
            break;
        }

        
    }
    println!("end of program");
}

// return value from  loop

fn r_loop(){
    let mut counter =0;
    let result = loop{
        counter+=1;
        if counter==10{
            break counter*2;
        }
    };
    println!("{result}");
}

// loop lables 
fn loop_lable(){
    let mut count =0;
    'counting_up : loop{
        println!("count ={count}");

        let mut remaining =10;
        loop{
            println!("remaining : {remaining}");

            if remaining ==9{
                break;
            }
            if count == 2{
                break 'counting_up;
            }
            remaining-=1;
        }
        count+=1;
    }
    println!("count is {count}");
}


//conditional loop with while 
fn loop_while(){
    let mut num   = 3;
    while num != 0{
        println!("{num}");

        num -= 1;

    }
    println!("yeahhhh");
}

// looping through collection 
fn loop_arr(){
    let  a = [10,2,20,3,0];
    let mut i =0;
    while i < a.len() {
        println!("the value is {}",a[i]);
        i =i+1;
    }
}
// this approach is error prone and slow 
// whenever we have  work with collections use for loop

fn for_loop(){
    let a =[10,20,30,40,50];

    for values in a{
        println!("the value is {values}");
    }
}

// range of numbers 

fn for_range(){
    for num in 1..10{
        println!("the number is {num}");
    }

    println!("reverse");

    for num in (1..10).rev(){
        println!("the number is {num}");
    }
}
