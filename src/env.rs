use std::path::PathBuf;
use std::io::prelude::*;
use std::env;
use std::fs::File;
pub struct EnvGet{
location:PathBuf,
}
impl EnvGet{
pub fn setup()->Self{
EnvGet{location:env::current_dir().unwrap()}
}
pub fn env(&mut self)->Result<String,std::io::Error>{
let a=&mut self.location;
a.push("src");
a.push(".env");
let mut m = File::open(a)?;
    let mut s = String::new();
    let _r = m.read_to_string(&mut s)?;
    let h:Vec<&str>=s.split_inclusive("=").collect();
    let g=h.get(1).unwrap();
    let l=String::from(*g);
    Ok(l)
}
}
