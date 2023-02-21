use std::io;
fn main(){
    let mut a=String::new();
    io::stdin()
     .read_line(&mut a)
     .expect("Enter a vaild number.");
    let a=a.trim().parse().expect("Enter a valid number.");
    let mut y=0;
    let mut i=0;
    let mut r=1;
    let mut total;
    while y!=a{
       println!("{i}");
       total=i+r;
       i=r;
       r=total;
       y+=1;
    }
}