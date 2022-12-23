use std::ops::Add;
use std::time::Duration;
use serenity::model::timestamp::Timestamp;
//write a statement when 24 is reached to change a day one more time
fn hours(a:String)->u64{
let s:Vec<&str>=a.rsplit("h").collect();
let sort=*s.get(1).unwrap();
let j=sort.parse::<u64>().unwrap();
if j>14{
let h=j-14;
return j*3600+h*3600;    
}
else{
return j*3600;
}
}
fn minutes(a:String)->u64{
    let s:Vec<&str>=a.rsplit("m").collect();
    let sort=*s.get(1).unwrap();
let j=sort.parse::<u64>().unwrap();
j*60
}
fn mixed(a:String){
let s:Vec<&str>=a.rsplit("m").collect();
for i in s.iter(){
println!("{}",i);
}
}
pub struct Parser{
parsed:Timestamp,
}
impl Parser{
pub fn from(a:Timestamp)->Self{
Parser{parsed:a}
}
pub fn convert(&self)->String{
let dur=Duration::new(3600,1);
let date=&self.parsed.date().to_string();
let hour=&self.parsed.time().add(dur).to_string();
format!("{} {}",date,hour)
}
pub fn calculate(&self,s:String)->u64{
let a=s.contains("h");
let b=s.contains("m");
let c=s.contains("hm");
let d="30hm".to_string();
mixed(d);
match s{
s if a==true=>hours(s),
s if b==true=>minutes(s),
_=>panic!("Error!"),
}
}
}