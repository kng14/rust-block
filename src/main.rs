fn main() {
   let message = "hello world!";
   let x: i32 = 42;
   let pi: f64= 3.14;
   let is_rust_fun: bool = true;
   let letter_a: char = 'a';

   fn add(x: i32, y: i32) -> i32{
        x+y
   }

   let x = 4;
   if x>=0 {
    println!("x is not negative");
   }else {
    println!("x is negative");
   }

   let mut i=1;
   while i<=5{
    println!("{}",i);
    i +=1;
   }
   let min_i32 = i32::MIN;
   let max_i32 = i32::MAX;

   println!("The minimum value of i32 is {} and the maximum value {}.",min_i32, max_i32);

}
