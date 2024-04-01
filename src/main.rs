// hello world
/*
  multi line comment
*/
fn main() {
  println!("Hello world");
  // variable();
  // array_fn();
  // tuple_fn();
  // test(10);
  // println!("{:?}",square(10));
  // conditional()
  // loop_fn();
  // while_fn();
  // for_loop_fn();
  // shadowing()
  // string_var()
  ownership()
}

fn ownership(){
  let outer : String;
  {
    let inner= String::from("mars");
    outer = inner;
    println!("{}", outer);
    // println!("{}", inner); // fails since the inner looses the ownership, there can only be one owner of a memory
    // this is only valid for var allocated in heap
  }
}

fn string_var(){
  let mut s = String::from("Earth");
  println!("{}",s);
  println!("{}",s.len());
  s.push_str(" Moon");
  println!("{}",s);
}

fn shadowing(){
  let a = 1;
  println!("{}", a);
  let a = "2";
  println!("{}", a);
}

fn for_loop_fn(){
  let mut a = [1,2,3,4];
  for item in a{
    println!("{}", item)
  }
  for item in a.iter(){
    println!("{}", item)
  }
  for item in a.iter_mut(){
    println!("{}", item)
  }
  for (idx, item) in a.iter().enumerate(){
    println!("{}, {}", idx, item)
  }
  for number in 0..5 {
    println!("{}", number)
  }
}

fn while_fn(){
  let mut cnt = 0;
  while cnt < 10 {
    println!("{}", cnt);
    cnt += 1;
  }
}

fn loop_fn(){
  let mut count = 0;
  let x = loop {
    count += 1;
    println!("{}", count);
    if count > 5 {
      break count;
    }
  };
  println!("{}", x);
}

fn variable(){
  let x: i32= 10; // signed int
  println!("X is {}",x);
  // x = 20 // compiler error since you cannot mutate x

  let mut y: u32 = 10; // unsigned int
  println!("y is {}",y);

  y = 100;
  println!("y is {}",y);

  let fx: f32 = 10.19;
  println!("fx is {}",fx);

  let fy = 19.3;
  println!("fy is {}",fy);

  let a = 10;
  let b = 3.0;
  println!("{}",a as f32 / b); // typecasting

  println!("{0}, {1}, {0}", a, b); // print formatting

  let val = 0b1111_0000u8; // u8 suffix to ensure it is stored as u8, 0b prefix to denote it is binary format, _ is ignored
  println!("{}", val);
  println!("{:08b}", val);
  println!("{:08b}", !val);
  println!("{:08b}", val | val);
  println!("{:08b}", val & val);
  println!("{:08b}", val ^ val);
  println!("{:08b}", val << 1);
  println!("{:08b}", val >> 1);

  let letter = 'a';
  let number = '1'; // it is 4 bytes long, so can store unicode chars not just ascii
  println!("{} {}", letter, number)
}

fn array_fn(){
  let mut letters = ['a','b','c'];
  println!("{}", letters[0]);
  letters[0] = 'd';
  println!("{}", letters[0]);

  let mut numbers: [i32; 5] = [1,2,3,4,5];
  println!("{:?}", numbers);
  numbers = [0; 5];
  println!("{:?}", numbers);
  println!("{}", numbers.len());
  let size: usize = numbers.len();
  println!("{}", size);

  let pl = [[1,2,3],[5,6,7]];
  println!("{:?}", pl);
  println!("{:?}", pl[1]);
  println!("{:?}", pl[1][1]);
}

fn tuple_fn(){
  let stuff = (10, 3.4, 'a');
  println!("{:?}", stuff);
  println!("{:?}", stuff.0);

  let (a, b, c) = stuff;
  println!("{}, {}, {}", a,b,c);
}

fn test(num: i32){
  println!("{}", num);

}

fn square(num: i32) -> (i32, i32) {
  return (num * num, num); // num * num will also do
}

fn conditional(){
  let x = 3;
  if x == 3 {
    println!("x is 3")
  } else if x == 0 {
    println!("x is not 0")
  }else {
    println!("x is not 3")
  }
  let y = if x == 3 {0} else {4};
  println!("{}", y);
}