mod bitfield;
use bitfield::Bitfield;
fn main() {
    let req = Bitfield::create_i32(String::from("mode"),0xff, 0, 8);
    match req {
        Ok(mut req) => {
            match req.modify(String::from("mode"), String::from("mode1"), 0, 1) {
                Ok(()) => println!("修改mode为mode1成功"),
                Err(str) => println!("{}",str),
            }
            match req.insert(String::from("test"), 2, 3){
                Ok(()) =>{},
                Err(str) => println!("{}",str),
            }
            match req.get_param("test") {
                Ok(str) => {
                    println!("test的值为:\n{:#?}",str)
                },
                Err(err) => println!("{}",err),
            }
            
            match req.get_value("mode1") {
                Ok(str) => println!("mode1:{}",str),
                Err(err) => {println!("{}",err)} ,
            }
            let str =req.get_value("test");
            match str {
                Ok(str) => println!("test:{}",str),
                Err(err) => {println!("{}",err)} ,
            }
            match req.set_value("mode1", 0) {
                Ok(()) =>{},
                Err(err) => {
                    println!("{}",err)
                }
            }
            match req.get_value("mode1") {
                Ok(str) => println!("mode1:{}",str),
                Err(err) => {println!("{}",err)} ,
            }
            let format = format!("0b{:0width$b}",req.get_data(), width = req.get_totalbit());
            println!("{}",format);
        }
        Err(err) => println!("{}",err),
    }
    
}