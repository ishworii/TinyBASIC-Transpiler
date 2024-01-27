use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
#[derive(Debug,Clone)]
pub struct Emitter{
    full_path : String,
    header : String,
    code : String
}

impl Emitter{
    pub fn new(full_path : String) -> Self{
        Emitter{
            full_path,
            header : String::from(""),
            code : String::from(""),
        }
    }

    pub fn emit(&mut self,code:String){
        self.code += &code
    }

    pub fn emit_line(&mut self,code:String){
        self.code += &*(code + "\n")
    }

    pub fn header_line(&mut self,code:String){
        self.header += &*(code + "\n")
    }

    pub fn write_to_file(&mut self) -> Result<()>{
        let mut file = File::create(&self.full_path)?;
        let contents = format!("{}{}",self.header,self.code);
        file.write_all(contents.as_bytes())?;

        Ok(())
    }


}