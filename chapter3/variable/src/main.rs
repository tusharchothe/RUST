fn main() {
   //let mut age =32;
   // println!("{}",age);

   // const PI :f32=3.1452;
   // age =25;
   // println!("{}",age);

   // println!("{}",PI);

   // const ONE_HOUR :u64= 60*60;
   // let time :u64 =ONE_HOUR*30;
   // println!("The time is {time}");

   //Shadowinng 
   //let apple :u64 =23;
   //let apple :bool =true ;
   //println!("{apple}");

   let x= 5;
   let x =x+5;
   {
      let x =x+2;
      println!("The value of x in inner scope {}",x);
   }
   println!("The value of x is {}",x);


   let space = "    ";
   let space=space.len();
   println!("{}",space);
}
