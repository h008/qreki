use qreki::Qreki;

fn main(){
   let k= Qreki::from_ymd(2033, 9, 24);
   println!("{:?}",k);
   let l = Qreki::new();
   println!("{:?}",l);
}