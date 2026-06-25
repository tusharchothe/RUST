// define a struct 
struct User{
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64,
}


fn main() {
    let mut user_1=User{
        email:String::from("tushu@gmail.com"),
        username:String::from("Tushar Chothe"),
        active:true,
        sign_in_count:0,

    };

    let mut user_2=User{
        email:String::from("john@gmail.com"),
        username:String::from("jhon cena"),
        active:true,
        sign_in_count:0,
    };


    //using build_user to create new user
    let mut user_3 =build_user(
        String::from("new user using build_user"),
        String::from("build_user@gmail.com"),
    );

    // acess the value of the struct using dot operator
    //println!("the name of user is {}",user_1.username); 
    // change the user name 
    //user_1.username=String::from("vedika chothe");
    //alternate method 
    //user_1.username.push_str("hey");
    //println!("the name of user is {}",user_1.username); 


    //create a instances from other instances using struct
    let mut user_4 =User{
        active :user_1.active,
        username:user_1.username,
        email:String::from("anotherexample@gmail.com"),
        sign_in_count:user_1.sign_in_count,
    };

    // another method 
    let  mut user_5 =User{
        email:String::from("simple@gmail.com"),
        ..user_3
    };
    // because of above two example 
    // i cannot use user3 and user1 ki values beacuse ownership chali gayi
    //dusare instance ke pas

    //use of tuple struct 
    let red =Color(100,0,0);
    set_bg_color(red);
    let moved =Point(100,20,0);
    move_point(moved);
}

// if we have to build multiple instances of user 
fn build_user(username:String,email:String) -> User{
    User{
        username,
        email,
        active:true,
        sign_in_count:0,
    }
}

//declare tuple Struct
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

//create a function to understand the tuple struct
fn set_bg_color(color:Color){
    println!(
        "Setting background color R:{},G:{},B:{}",
        color.0,color.1,color.2
    )
}

// coordinate point '
fn move_point(point:Point){
    println!(
        "The cursor was moved X:{},Y:{},Z:{}",
        point.0,point.1,point.2
    )
}