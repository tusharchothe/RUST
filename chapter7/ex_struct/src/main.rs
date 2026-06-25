#[derive (Debug)]

struct rectangle{
    width :u32,
    height :u32
}

fn main() {
    // for normal
   let w1 = 30;
   let h1 = 20;
   println!("Area (normal) :{}", cal_area_normal(w1, h1));


   //for refractoring tuple
   let rect = (30, 20);
   let area = cal_area_tup(rect);
   println!("Area (refractoring tuple) :{}", area);

   // struct
   let rect_struct = rectangle{
    width:30,
    height:20
   };
   println!("Area(struct):{}",cal_area_struct(&rect_struct));

   //debug 
   println!("the area of {:?} is {}", rect_struct,cal_area_struct(&rect_struct));
}

// normal method 
fn cal_area_normal(width: u32, height: u32) -> u32 {
    width * height
}

// Refactoring with tuples 
fn cal_area_tup(dimension: (u32, u32)) -> u32 {
    // Fixed: Removed the type declarations inside the pattern
    let (width, height) = dimension; 
    width * height
}

// using the struct 
fn cal_area_struct(rect : &rectangle) -> u32{
    rect.width *rect.height
}