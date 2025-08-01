fn main() {
  let n = 10;
  print_fibonaci(n);
}

fn print_fibonaci(x: i32){
  let mut a = 0;
  let mut b = 1;

  for _num in 0..x {
      let c = a + b;
      print!("{a} ");
      a = b;
      b = c;
  }
}