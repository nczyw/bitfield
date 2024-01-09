
use std::collections::HashMap;
use std::mem;

/**
 * 基数结构体,原始数据存放位置
 */
#[derive(Debug)]
struct BitfieldBase<T> {
    bitfielddata: T,
}

/**
 * 位域数据存放结构体
 */
#[derive(Debug)]
pub struct BitfieldValue {
    start : u8 ,
    length : u8 ,
}

/**
 * 位域数据处理结构体
 */
#[derive(Debug)]
pub struct Bitfield<T> {
    bitfield: BitfieldBase<T>,
    bitmap: HashMap<String,BitfieldValue>,
}
/**
 * 为 u8 数据 实现按位读写
 */
#[allow(dead_code)]
impl Bitfield<u8> {
    /**
     * @description:  静态函数，用于创建一个u8位域处理结构体
     * @param {String} key  位域名字
     * @param {u8} bitfielddata 需要做位域处理的数据
     * @param {u8} start    位域开始地址
     * @param {u8} length   位域长度
     * @return {Result} 创建成功，返回一个Result枚举，失败返回失败信息
     */    
    pub fn create_u8(key:String, bitfielddata : u8 , start : u8 , length : u8) -> Result<Bitfield<u8>,String> {
        let size = mem::size_of::<u8>() as u8;
        if start > size * 8 - 1 {
            let format = format!("Illegal starting position;start{}",start);
            return Err(format);
        }
        else if (start + length) > size * 8  {
            let format = format!("Illegal total length;start:{},length:{}",start ,length);
            return Err(format);
        }
        let valuetype = BitfieldValue{start:start,length:length};
        let mut bitmap:HashMap<String,BitfieldValue> = HashMap::new();
        bitmap.insert(key, valuetype);
        Ok(Bitfield{
            bitfield:BitfieldBase{bitfielddata:bitfielddata},
            bitmap:bitmap,
        })
    }
    /**
     * @description:    打印位域结构体
     * @param {*} self  不可变的自己
     * @return {无}      无
     */    
    pub fn traversal(& self){
        println!("{:#?}",self);
    }
    /**
     * @description:    设置需要做位域处理的数据
     * @param {*} mut self  可变的自己
     * @param {u8} bitfielddata 需要做位域处理的数据
     * @return {无} 无
     */    
    pub fn set_data(& mut self , bitfielddata :u8){
        self.bitfield.bitfielddata = bitfielddata ;
    }
    /**
     * @description: 获取做位域处理的数据
     * @param {*} self  不可变的自己
     * @return {无} 无
     */    
    pub fn get_data(& self) -> u8 {
        self.bitfield.bitfielddata
    }
    /**
     * @description: 插入一个位域信息
     * @param {*} mut self  可变的自己
     * @param {String} key  位域名字
     * @param {u8} start    位域开始地址
     * @param {u8} length   位域长度
     * @return {Result}     成功，无信息，失败时，返回失败原因
     */    
    pub fn insert(& mut self , key:String , start : u8 , length : u8) -> Result<(), String>{
        let size = mem::size_of::<u8>() as u8;
        if start > size * 8 - 1 {
            let format = format!("Insert failed,Illegal starting position;start{}",start);
            return Err(format);
        }
        else if (start + length) > size * 8  {
            let format = format!("Insert failed,Illegal total length;start:{},length:{}",start ,length);
            return Err(format);
        }
        match self.bitmap.insert(key, BitfieldValue{start:start,length:length}){
            Some(_) =>{
                let format = format!("Insert failed,Key already exists");
                Err(format)
            },
            None => {Ok(())}
        }
        
    }
    /**
     * @description:    修改一个位域信息
     * @param {*} mut self  可变的自己
     * @param {String} oldkey   旧位域名字
     * @param {String} newkey   新位域名字
     * @param {u8} start        位域开始地址
     * @param {u8} length       位域长度
     * @return {Result} 成功，无信息，失败时，返回失败原因
     */    
    pub fn modify(& mut self ,oldkey:String , newkey:String , start :u8 , length : u8) -> Result<(),String> {
        match self.del_param(oldkey.as_str()) {
            Ok(()) =>{
                match self.insert(newkey, start, length){
                    Ok(()) => Ok(()) ,
                    Err(err) => Err(err) ,
                }
            },
            Err(_) => {
                let format = format!("Not found key:{}",oldkey);
                Err(format)
            },
        }
    }
    /**
     * @description: 获取位域设置信息
     * @param {*} self  不可变的自己
     * @param {&str} key   位域名字
     * @return {Result} 成功时返回BitfieldValue结构体,失败时，返回失败原因
     */    
    pub fn get_param(&self , key : &str) -> Result<&BitfieldValue,String> {
        match self.bitmap.get(key) {
            Some(value) => Ok(value),
            None => {
                let format = format!("Not found key:{}",key);
                Err(format)
            },
        }
    }
    /**
     * @description: 删除位域信息
     * @param {*}   可变的自己
     * @param {&str} key   位域名字
     * @return {Result} 成功，无信息，失败时，返回失败原因
     */    
    pub fn del_param(& mut self,key:&str) ->Result<(),String>{
        match self.bitmap.remove(key) {
            Some(_) => Ok(()),
            None => Err(String::from("Delete failed , key not found")),
        }
    }
    /**
     * @description: 根据位域信息，获取对应的值
     * @param {*} self  不可变的自己
     * @param {&str} key   位域名字
     * @return {Result} 成功，返回位域对应的数据,失败时，返回失败原因
     */    
    pub fn get_value(& self,key:&str) ->Result<u8 , String> {
        let valuetype = self.bitmap.get(key);
        match valuetype {
            Some(value) => {
                let size = mem::size_of::<u8>() as u8;
                if value.length == size * 8 {
                    let resul = (self.bitfield.bitfielddata >> value.start) & 0xff;
                    Ok(resul)
                }
                else {
                    let resul = (self.bitfield.bitfielddata >> value.start) & ((2u8.pow(value.length as u32)) - 1); //取出指定的位数
                    Ok(resul)
                }
            },
            None => {
                let format = format!("Not found key:{}",key);
                Err(format)
            },
        }
    }
    /**
     * @description: 根据位域信息，设置对应的值
     * @param {*}    可变的自己
     * @param {&str} key    位域名字
     * @param {u8} value    要改变位域的对应的值
     * @return {Result}     成功，无信息，失败，返回失败原因
     */    
    pub fn set_value(& mut self,key:&str , value : u8) -> Result<(),String> {
        match self.bitmap.get(key) {
            Some(mvalue) => {
                if value >= 2u8.pow(mvalue.length as u32)  { //防止超范围
                    let format = format!("The value is out of range,the maximum value is:{},value:{}",(2u8.pow(mvalue.length as u32)) - 1,value);
                    Err(format)
                }
                else {
                    let tmp = (2u8.pow(mvalue.length as u32)) - 1 << mvalue.start;      //计算预数据
                    let tmp1 = !tmp & self.bitfield.bitfielddata;    //清空指定位数据
                    let tmp2 = value << mvalue.start;
                    self.bitfield.bitfielddata = tmp1 | tmp2 ;
                    Ok(())
                }
            },
            None => {
                let format = format!("Not found key:{}",key);
                Err(format)
            },
        }
    }
    /**
     * @description: 获取位数
     * @param {*} self  不可变的自己
     * @return {usize}  返回对应的位数
     */    
    pub fn get_totalbit(& self) -> usize {
        8
    }
}

#[allow(dead_code)]
impl Bitfield<i8> {
    pub fn create_i8(key:String, bitfielddata : i8 , start : u8 , length : u8) -> Result<Bitfield<i8>,String> {
        let size = mem::size_of::<i8>() as u8;
        if start > size * 8 - 1 {
            let format = format!("Illegal starting position;start{}",start);
            return Err(format);
        }
        else if (start + length) > size * 8 {
            let format = format!("Illegal total length;start:{},length:{}",start ,length);
            return Err(format);
        }
        let valuetype = BitfieldValue{start:start,length:length};
        let mut bitmap:HashMap<String,BitfieldValue> = HashMap::new();
        bitmap.insert(key, valuetype);
        Ok(Bitfield{
            bitfield:BitfieldBase{bitfielddata:bitfielddata},
            bitmap:bitmap,
        })
    }
    pub fn traversal(& self){
        println!("{:#?}",self);
    }
    pub fn set_data(& mut self , bitfielddata :i8){
        self.bitfield.bitfielddata = bitfielddata ;
    }
    pub fn get_data(& self) -> i8 {
        self.bitfield.bitfielddata
    }
    pub fn insert(& mut self , key:String , start : u8 , length : u8) -> Result<(), String>{
        let size = mem::size_of::<i8>() as u8;
        if start > size * 8 - 1 {
            let format = format!("Insert failed,Illegal starting position;start{}",start);
            return Err(format);
        }
        else if (start + length) > size * 8  {
            let format = format!("Insert failed,Illegal total length;start:{},length:{}",start ,length);
            return Err(format);
        }
        match self.bitmap.insert(key, BitfieldValue{start:start,length:length}){
            Some(_) =>{
                let format = format!("Insert failed,Key already exists");
                Err(format)
            },
            None => {Ok(())}
        }
        
    }
    pub fn modify(& mut self ,oldkey:String , newkey:String , start :u8 , length : u8) -> Result<(),String> {
        match self.del_param(oldkey.as_str()) {
            Ok(()) =>{
                match self.insert(newkey, start, length){
                    Ok(()) => Ok(()) ,
                    Err(err) => Err(err) ,
                }
            },
            Err(_) => {
                let format = format!("Not found key:{}",oldkey);
                Err(format)
            },
        }
    }
    pub fn get_param(&self , key : &str) -> Result<&BitfieldValue,String> {
        match self.bitmap.get(key) {
            Some(value) => Ok(value),
            None => {
                let format = format!("Not found key:{}",key);
                Err(format)
            },
        }
    }
    pub fn del_param(& mut self,key:&str) ->Result<(),String>{
        match self.bitmap.remove(key) {
            Some(_) => Ok(()),
            None => Err(String::from("Delete failed , key not found")),
        }
    }
    pub fn get_value(& self,key:&str) ->Result<i8 , String> {
        let valuetype = self.bitmap.get(key);
        match valuetype {
            Some(value) => {
                let size = mem::size_of::<i8>() as u8;
                if value.length == size * 8 {
                    let resul = (self.bitfield.bitfielddata >> value.start) & -1;
                    Ok(resul)
                }
                else {
                    let resul = (self.bitfield.bitfielddata >> value.start) & ((2i8.pow(value.length as u32)) - 1); //取出指定的位数
                    Ok(resul)
                }
            },
            None => {
                let format = format!("Not found key:{}",key);
                Err(format)
            },
        }
    }
    pub fn set_value(& mut self,key:&str , value : i8) -> Result<(),String> {
        match self.bitmap.get(key) {
            Some(mvalue) => {
                if value >= 2i8.pow(mvalue.length as u32)  { //防止超范围
                    let format = format!("The value is out of range,the maximum value is:{},value:{}",(2i8.pow(mvalue.length as u32)) - 1,value);
                    Err(format)
                }
                else {
                    let tmp = (2i8.pow(mvalue.length as u32)) - 1 << mvalue.start;      //计算预数据
                    let tmp1 = !tmp & self.bitfield.bitfielddata;    //清空指定位数据
                    let tmp2 = value << mvalue.start;
                    self.bitfield.bitfielddata = tmp1 | tmp2 ;
                    Ok(())
                }
            },
            None => {
                let format = format!("Not found key:{}",key);
                Err(format)
            },
        }
    }
    pub fn get_totalbit(& self) -> usize {
        8
    }
}

#[allow(dead_code)]
impl Bitfield<u16> {
    pub fn create_u16(key:String, bitfielddata : u16 , start : u8 , length : u8) -> Result<Bitfield<u16>,String> {
        let size = mem::size_of::<u16>() as u8;
        if start > size * 8 - 1 {
            let format = format!("Illegal starting position;start{}",start);
            return Err(format);
        }
        else if (start + length) > size * 8  {
            let format = format!("Illegal total length;start:{},length:{}",start ,length);
            return Err(format);
        }
        let valuetype = BitfieldValue{start:start,length:length};
        let mut bitmap:HashMap<String,BitfieldValue> = HashMap::new();
        bitmap.insert(key, valuetype);
        Ok(Bitfield{
            bitfield:BitfieldBase{bitfielddata:bitfielddata},
            bitmap:bitmap,
        })
    }
    pub fn traversal(& self){
        println!("{:#?}",self);
    }
    pub fn set_data(& mut self , bitfielddata :u16){
        self.bitfield.bitfielddata = bitfielddata ;
    }
    pub fn get_data(& self) -> u16 {
        self.bitfield.bitfielddata
    }
    pub fn insert(& mut self , key:String , start : u8 , length : u8) -> Result<(), String>{
        let size = mem::size_of::<u16>() as u8;
        if start > size * 8 - 1 {
            let format = format!("Insert failed,Illegal starting position;start{}",start);
            return Err(format);
        }
        else if (start + length) > size * 8  {
            let format = format!("Insert failed,Illegal total length;start:{},length:{}",start ,length);
            return Err(format);
        }
        match self.bitmap.insert(key, BitfieldValue{start:start,length:length}){
            Some(_) =>{
                let format = format!("Insert failed,Key already exists",);
                Err(format)
            },
            None => {Ok(())}
        }
        
    }
    pub fn modify(& mut self ,oldkey:String , newkey:String , start :u8 , length : u8) -> Result<(),String> {
        match self.del_param(oldkey.as_str()) {
            Ok(()) =>{
                match self.insert(newkey, start, length){
                    Ok(()) => Ok(()) ,
                    Err(err) => Err(err) ,
                }
            },
            Err(_) => {
                let format = format!("Not found key:{}",oldkey);
                Err(format)
            },
        }
    }
    pub fn get_param(&self , key : &str) -> Result<&BitfieldValue,String> {
        match self.bitmap.get(key) {
            Some(value) => Ok(value),
            None => {
                let format = format!("Not found key:{}",key);
                Err(format)
            },
        }
    }
    pub fn del_param(& mut self,key:&str) ->Result<(),String>{
        match self.bitmap.remove(key) {
            Some(_) => Ok(()),
            None => Err(String::from("Delete failed , key not found")),
        }
    }
    pub fn get_value(& self,key:&str) ->Result<u16 , String> {
        let valuetype = self.bitmap.get(key);
        match valuetype {
            Some(value) => {
                let size = mem::size_of::<u16>() as u8;
                if value.length == size * 8 {
                    let resul = (self.bitfield.bitfielddata >> value.start) & 0xffff;
                    Ok(resul)
                }
                else {
                    let resul = (self.bitfield.bitfielddata >> value.start) & ((2u16.pow(value.length as u32)) - 1); //取出指定的位数
                    Ok(resul)
                }
            },
            None => {
                let format = format!("Not found key:{}",key);
                Err(format)
            },
        }
    }
    pub fn set_value(& mut self,key:&str , value : u16) -> Result<(),String> {
        match self.bitmap.get(key) {
            Some(mvalue) => {
                if value >= 2u16.pow(mvalue.length as u32)  { //防止超范围
                    let format = format!("The value is out of range,the maximum value is:{},value:{}",(2u16.pow(mvalue.length as u32)) - 1,value);
                    Err(format)
                }
                else {
                    let tmp = (2u16.pow(mvalue.length as u32)) - 1 << mvalue.start;      //计算预数据
                    let tmp1 = !tmp & self.bitfield.bitfielddata;    //清空指定位数据
                    let tmp2 = value << mvalue.start;
                    self.bitfield.bitfielddata = tmp1 | tmp2 ;
                    Ok(())
                }
            },
            None => {
                let format = format!("Not found key:{}",key);
                Err(format)
            },
        }
    }
    pub fn get_totalbit(& self) -> usize {
        16
    }
}

#[allow(dead_code)]
impl Bitfield<i16> {
    pub fn create_i16(key:String, bitfielddata : i16 , start : u8 , length : u8) -> Result<Bitfield<i16>,String> {
        let size = mem::size_of::<i16>() as u8;
        if start > size * 8 - 1 {
            let format = format!("Illegal starting position;start{}",start);
            return Err(format);
        }
        else if (start + length) > size * 8 {
            let format = format!("Illegal total length;start:{},length:{}",start ,length);
            return Err(format);
        }
        let valuetype = BitfieldValue{start:start,length:length};
        let mut bitmap:HashMap<String,BitfieldValue> = HashMap::new();
        bitmap.insert(key, valuetype);
        Ok(Bitfield{
            bitfield:BitfieldBase{bitfielddata:bitfielddata},
            bitmap:bitmap,
        })
    }
    pub fn traversal(& self){
        println!("{:#?}",self);
    }
    pub fn set_data(& mut self , bitfielddata :i16){
        self.bitfield.bitfielddata = bitfielddata ;
    }
    pub fn get_data(& self) -> i16 {
        self.bitfield.bitfielddata
    }
    pub fn insert(& mut self , key:String , start : u8 , length : u8) -> Result<(), String>{
        let size = mem::size_of::<i16>() as u8;
        if start > size * 8 - 1 {
            let format = format!("Insert failed,Illegal starting position;start{}",start);
            return Err(format);
        }
        else if (start + length) > size * 8  {
            let format = format!("Insert failed,Illegal total length;start:{},length:{}",start ,length);
            return Err(format);
        }
        match self.bitmap.insert(key, BitfieldValue{start:start,length:length}){
            Some(_) =>{
                let format = format!("Insert failed,Key already exists");
                Err(format)
            },
            None => {Ok(())}
        }
        
    }
    pub fn modify(& mut self ,oldkey:String , newkey:String , start :u8 , length : u8) -> Result<(),String> {
        match self.del_param(oldkey.as_str()) {
            Ok(()) =>{
                match self.insert(newkey, start, length){
                    Ok(()) => Ok(()) ,
                    Err(err) => Err(err) ,
                }
            },
            Err(_) => {
                let format = format!("Not found key:{}",oldkey);
                Err(format)
            },
        }
    }
    pub fn get_param(&self , key : &str) -> Result<&BitfieldValue,String> {
        match self.bitmap.get(key) {
            Some(value) => Ok(value),
            None => {
                let format = format!("Not found key:{}",key);
                Err(format)
            },
        }
    }
    pub fn del_param(& mut self,key:&str) ->Result<(),String>{
        match self.bitmap.remove(key) {
            Some(_) => Ok(()),
            None => Err(String::from("Delete failed , key not found")),
        }
    }
    pub fn get_value(& self,key:&str) ->Result<i16 , String> {
        let valuetype = self.bitmap.get(key);
        match valuetype {
            Some(value) => {
                let size = mem::size_of::<i16>() as u8;
                if value.length == size * 8 {
                    let resul = (self.bitfield.bitfielddata >> value.start) & -1;
                    Ok(resul)
                }
                else {
                    let resul = (self.bitfield.bitfielddata >> value.start) & ((2i16.pow(value.length as u32)) - 1); //取出指定的位数
                    Ok(resul)
                }
            },
            None => {
                let format = format!("Not found key:{}",key);
                Err(format)
            },
        }
    }
    pub fn set_value(& mut self,key:&str , value : i16) -> Result<(),String> {
        match self.bitmap.get(key) {
            Some(mvalue) => {
                if value >= 2i16.pow(mvalue.length as u32)  { //防止超范围
                    let format = format!("The value is out of range,the maximum value is:{},value:{}",(2i16.pow(mvalue.length as u32)) - 1,value);
                    Err(format)
                }
                else {
                    let tmp = (2i16.pow(mvalue.length as u32)) - 1 << mvalue.start;      //计算预数据
                    let tmp1 = !tmp & self.bitfield.bitfielddata;    //清空指定位数据
                    let tmp2 = value << mvalue.start;
                    self.bitfield.bitfielddata = tmp1 | tmp2 ;
                    Ok(())
                }
            },
            None => {
                let format = format!("Not found key:{}",key);
                Err(format)
            },
        }
    }
    pub fn get_totalbit(& self) -> usize {
        16
    }
}

#[allow(dead_code)]
impl Bitfield<u32> {
    pub fn create_u32(key:String, bitfielddata : u32 , start : u8 , length : u8) -> Result<Bitfield<u32>,String> {
        let size = mem::size_of::<u32>() as u8;
        if start > size * 8 - 1 {
            let format = format!("Illegal starting position;start{}",start);
            return Err(format);
        }
        else if (start + length) > size * 8  {
            let format = format!("Illegal total length;start:{},length:{}",start ,length);
            return Err(format);
        }
        let valuetype = BitfieldValue{start:start,length:length};
        let mut bitmap:HashMap<String,BitfieldValue> = HashMap::new();
        bitmap.insert(key, valuetype);
        Ok(Bitfield{
            bitfield:BitfieldBase{bitfielddata:bitfielddata},
            bitmap:bitmap,
        })
    }
    pub fn traversal(& self){
        println!("{:#?}",self);
    }
    pub fn set_data(& mut self , bitfielddata :u32){
        self.bitfield.bitfielddata = bitfielddata ;
    }
    pub fn get_data(& self) -> u32 {
        self.bitfield.bitfielddata
    }
    pub fn insert(& mut self , key:String , start : u8 , length : u8) -> Result<(), String>{
        let size = mem::size_of::<u32>() as u8;
        if start > size * 8 - 1 {
            let format = format!("Insert failed,Illegal starting position;start{}",start);
            return Err(format);
        }
        else if (start + length) > size * 8  {
            let format = format!("Insert failed,Illegal total length;start:{},length:{}",start ,length);
            return Err(format);
        }
        match self.bitmap.insert(key, BitfieldValue{start:start,length:length}){
            Some(_) =>{
                let format = format!("Insert failed,Key already exists",);
                Err(format)
            },
            None => {Ok(())}
        }
        
    }
    pub fn modify(& mut self ,oldkey:String , newkey:String , start :u8 , length : u8) -> Result<(),String> {
        match self.del_param(oldkey.as_str()) {
            Ok(()) =>{
                match self.insert(newkey, start, length){
                    Ok(()) => Ok(()) ,
                    Err(err) => Err(err) ,
                }
            },
            Err(_) => {
                let format = format!("Not found key:{}",oldkey);
                Err(format)
            },
        }
    }
    pub fn get_param(&self , key : &str) -> Result<&BitfieldValue,String> {
        match self.bitmap.get(key) {
            Some(value) => Ok(value),
            None => {
                let format = format!("Not found key:{}",key);
                Err(format)
            },
        }
    }
    pub fn del_param(& mut self,key:&str) ->Result<(),String>{
        match self.bitmap.remove(key) {
            Some(_) => Ok(()),
            None => Err(String::from("Delete failed , key not found")),
        }
    }
    pub fn get_value(& self,key:&str) ->Result<u32 , String> {
        let valuetype = self.bitmap.get(key);
        match valuetype {
            Some(value) => {
                let size = mem::size_of::<u32>() as u8;
                if value.length == size * 8 {
                    let resul = (self.bitfield.bitfielddata >> value.start) & 0xffff_ffff;
                    Ok(resul)
                }
                else {
                    let resul = (self.bitfield.bitfielddata >> value.start) & ((2u32.pow(value.length as u32)) - 1); //取出指定的位数
                    Ok(resul)
                }
            },
            None => {
                let format = format!("Not found key:{}",key);
                Err(format)
            },
        }
    }
    pub fn set_value(& mut self,key:&str , value : u32) -> Result<(),String> {
        match self.bitmap.get(key) {
            Some(mvalue) => {
                if value >= 2u32.pow(mvalue.length as u32)  { //防止超范围
                    let format = format!("The value is out of range,the maximum value is:{},value:{}",(2u32.pow(mvalue.length as u32)) - 1,value);
                    Err(format)
                }
                else {
                    let tmp = (2u32.pow(mvalue.length as u32)) - 1 << mvalue.start;      //计算预数据
                    let tmp1 = !tmp & self.bitfield.bitfielddata;    //清空指定位数据
                    let tmp2 = value << mvalue.start;
                    self.bitfield.bitfielddata = tmp1 | tmp2 ;
                    Ok(())
                }
            },
            None => {
                let format = format!("Not found key:{}",key);
                Err(format)
            },
        }
    }
    pub fn get_totalbit(& self) -> usize {
        32
    }
}

#[allow(dead_code)]
impl Bitfield<i32> {
    pub fn create_i32(key:String, bitfielddata : i32 , start : u8 , length : u8) -> Result<Bitfield<i32>,String> {
        let size = mem::size_of::<i32>() as u8;
        if start > size * 8 - 1 {
            let format = format!("Illegal starting position;start{}",start);
            return Err(format);
        }
        else if (start + length) > size * 8 {
            let format = format!("Illegal total length;start:{},length:{}",start ,length);
            return Err(format);
        }
        let valuetype = BitfieldValue{start:start,length:length};
        let mut bitmap:HashMap<String,BitfieldValue> = HashMap::new();
        bitmap.insert(key, valuetype);
        Ok(Bitfield{
            bitfield:BitfieldBase{bitfielddata:bitfielddata},
            bitmap:bitmap,
        })
    }
    pub fn traversal(& self){
        println!("{:#?}",self);
    }
    pub fn set_data(& mut self , bitfielddata :i32){
        self.bitfield.bitfielddata = bitfielddata ;
    }
    pub fn get_data(& self) -> i32 {
        self.bitfield.bitfielddata
    }
    pub fn insert(& mut self , key:String , start : u8 , length : u8) -> Result<(), String>{
        let size = mem::size_of::<i32>() as u8;
        if start > size * 8 - 1 {
            let format = format!("Insert failed,Illegal starting position;start{}",start);
            return Err(format);
        }
        else if (start + length) > size * 8  {
            let format = format!("Insert failed,Illegal total length;start:{},length:{}",start ,length);
            return Err(format);
        }
        match self.bitmap.insert(key, BitfieldValue{start:start,length:length}){
            Some(_) =>{
                let format = format!("Insert failed,Key already exists");
                Err(format)
            },
            None => {Ok(())}
        }
        
    }
    pub fn modify(& mut self ,oldkey:String , newkey:String , start :u8 , length : u8) -> Result<(),String> {
        match self.del_param(oldkey.as_str()) {
            Ok(()) =>{
                match self.insert(newkey, start, length){
                    Ok(()) => Ok(()) ,
                    Err(err) => Err(err) ,
                }
            },
            Err(_) => {
                let format = format!("Not found key:{}",oldkey);
                Err(format)
            },
        }
    }
    pub fn get_param(&self , key : &str) -> Result<&BitfieldValue,String> {
        match self.bitmap.get(key) {
            Some(value) => Ok(value),
            None => {
                let format = format!("Not found key:{}",key);
                Err(format)
            },
        }
    }
    pub fn del_param(& mut self,key:&str) ->Result<(),String>{
        match self.bitmap.remove(key) {
            Some(_) => Ok(()),
            None => Err(String::from("Delete failed , key not found")),
        }
    }
    pub fn get_value(& self,key:&str) ->Result<i32 , String> {
        let valuetype = self.bitmap.get(key);
        match valuetype {
            Some(value) => {
                let size = mem::size_of::<i32>() as u8;
                if value.length == size * 8 {
                    let resul = (self.bitfield.bitfielddata >> value.start) & -1;
                    Ok(resul)
                }
                else {
                    let resul = (self.bitfield.bitfielddata >> value.start) & ((2i32.pow(value.length as u32)) - 1); //取出指定的位数
                    Ok(resul)
                }
            },
            None => {
                let format = format!("Not found key:{}",key);
                Err(format)
            },
        }
    }
    pub fn set_value(& mut self,key:&str , value : i32) -> Result<(),String> {
        match self.bitmap.get(key) {
            Some(mvalue) => {
                if value >= 2i32.pow(mvalue.length as u32)  { //防止超范围
                    let format = format!("The value is out of range,the maximum value is:{},value:{}",(2i32.pow(mvalue.length as u32)) - 1,value);
                    Err(format)
                }
                else {
                    let tmp = (2i32.pow(mvalue.length as u32)) - 1 << mvalue.start;      //计算预数据
                    let tmp1 = !tmp & self.bitfield.bitfielddata;    //清空指定位数据
                    let tmp2 = value << mvalue.start;
                    self.bitfield.bitfielddata = tmp1 | tmp2 ;
                    Ok(())
                }
            },
            None => {
                let format = format!("Not found key:{}",key);
                Err(format)
            },
        }
    }
    pub fn get_totalbit(& self) -> usize {
        32
    }
}

#[allow(dead_code)]
impl Bitfield<u64> {
    pub fn create_u64(key:String, bitfielddata : u64 , start : u8 , length : u8) -> Result<Bitfield<u64>,String> {
        let size = mem::size_of::<u64>() as u8;
        if start > size * 8 - 1 {
            let format = format!("Illegal starting position;start{}",start);
            return Err(format);
        }
        else if (start + length) > size * 8  {
            let format = format!("Illegal total length;start:{},length:{}",start ,length);
            return Err(format);
        }
        let valuetype = BitfieldValue{start:start,length:length};
        let mut bitmap:HashMap<String,BitfieldValue> = HashMap::new();
        bitmap.insert(key, valuetype);
        Ok(Bitfield{
            bitfield:BitfieldBase{bitfielddata:bitfielddata},
            bitmap:bitmap,
        })
    }
    pub fn traversal(& self){
        println!("{:#?}",self);
    }
    pub fn set_data(& mut self , bitfielddata :u64){
        self.bitfield.bitfielddata = bitfielddata ;
    }
    pub fn get_data(& self) -> u64 {
        self.bitfield.bitfielddata
    }
    pub fn insert(& mut self , key:String , start : u8 , length : u8) -> Result<(), String>{
        let size = mem::size_of::<u64>() as u8;
        if start > size * 8 - 1 {
            let format = format!("Insert failed,Illegal starting position;start{}",start);
            return Err(format);
        }
        else if (start + length) > size * 8  {
            let format = format!("Insert failed,Illegal total length;start:{},length:{}",start ,length);
            return Err(format);
        }
        match self.bitmap.insert(key, BitfieldValue{start:start,length:length}){
            Some(_) =>{
                let format = format!("Insert failed,Key already exists",);
                Err(format)
            },
            None => {Ok(())}
        }
        
    }
    pub fn modify(& mut self ,oldkey:String , newkey:String , start :u8 , length : u8) -> Result<(),String> {
        match self.del_param(oldkey.as_str()) {
            Ok(()) =>{
                match self.insert(newkey, start, length){
                    Ok(()) => Ok(()) ,
                    Err(err) => Err(err) ,
                }
            },
            Err(_) => {
                let format = format!("Not found key:{}",oldkey);
                Err(format)
            },
        }
    }
    pub fn get_param(&self , key : &str) -> Result<&BitfieldValue,String> {
        match self.bitmap.get(key) {
            Some(value) => Ok(value),
            None => {
                let format = format!("Not found key:{}",key);
                Err(format)
            },
        }
    }
    pub fn del_param(& mut self,key:&str) ->Result<(),String>{
        match self.bitmap.remove(key) {
            Some(_) => Ok(()),
            None => Err(String::from("Delete failed , key not found")),
        }
    }
    pub fn get_value(& self,key:&str) ->Result<u64 , String> {
        let valuetype = self.bitmap.get(key);
        match valuetype {
            Some(value) => {
                let size = mem::size_of::<u64>() as u8;
                if value.length == size * 8 {
                    let resul = (self.bitfield.bitfielddata >> value.start) & 0xffff_ffff_ffff_ffff;
                    Ok(resul)
                }
                else {
                    let resul = (self.bitfield.bitfielddata >> value.start) & ((2u64.pow(value.length as u32)) - 1); //取出指定的位数
                    Ok(resul)
                }
            },
            None => {
                let format = format!("Not found key:{}",key);
                Err(format)
            },
        }
    }
    pub fn set_value(& mut self,key:&str , value : u64) -> Result<(),String> {
        match self.bitmap.get(key) {
            Some(mvalue) => {
                if value >= 2u64.pow(mvalue.length as u32)  { //防止超范围
                    let format = format!("The value is out of range,the maximum value is:{},value:{}",(2u64.pow(mvalue.length as u32)) - 1,value);
                    Err(format)
                }
                else {
                    let tmp = (2u64.pow(mvalue.length as u32)) - 1 << mvalue.start;      //计算预数据
                    let tmp1 = !tmp & self.bitfield.bitfielddata;    //清空指定位数据
                    let tmp2 = value << mvalue.start;
                    self.bitfield.bitfielddata = tmp1 | tmp2 ;
                    Ok(())
                }
            },
            None => {
                let format = format!("Not found key:{}",key);
                Err(format)
            },
        }
    }
    pub fn get_totalbit(& self) -> usize {
        64
    }
}

#[allow(dead_code)]
impl Bitfield<i64> {
    pub fn create_i64(key:String, bitfielddata : i64 , start : u8 , length : u8) -> Result<Bitfield<i64>,String> {
        let size = mem::size_of::<i64>() as u8;
        if start > size * 8 - 1 {
            let format = format!("Illegal starting position;start{}",start);
            return Err(format);
        }
        else if (start + length) > size * 8 {
            let format = format!("Illegal total length;start:{},length:{}",start ,length);
            return Err(format);
        }
        let valuetype = BitfieldValue{start:start,length:length};
        let mut bitmap:HashMap<String,BitfieldValue> = HashMap::new();
        bitmap.insert(key, valuetype);
        Ok(Bitfield{
            bitfield:BitfieldBase{bitfielddata:bitfielddata},
            bitmap:bitmap,
        })
    }
    pub fn traversal(& self){
        println!("{:#?}",self);
    }
    pub fn set_data(& mut self , bitfielddata :i64){
        self.bitfield.bitfielddata = bitfielddata ;
    }
    pub fn get_data(& self) -> i64 {
        self.bitfield.bitfielddata
    }
    pub fn insert(& mut self , key:String , start : u8 , length : u8) -> Result<(), String>{
        let size = mem::size_of::<i64>() as u8;
        if start > size * 8 - 1 {
            let format = format!("Insert failed,Illegal starting position;start{}",start);
            return Err(format);
        }
        else if (start + length) > size * 8  {
            let format = format!("Insert failed,Illegal total length;start:{},length:{}",start ,length);
            return Err(format);
        }
        match self.bitmap.insert(key, BitfieldValue{start:start,length:length}){
            Some(_) =>{
                let format = format!("Insert failed,Key already exists");
                Err(format)
            },
            None => {Ok(())}
        }
        
    }
    pub fn modify(& mut self ,oldkey:String , newkey:String , start :u8 , length : u8) -> Result<(),String> {
        match self.del_param(oldkey.as_str()) {
            Ok(()) =>{
                match self.insert(newkey, start, length){
                    Ok(()) => Ok(()) ,
                    Err(err) => Err(err) ,
                }
            },
            Err(_) => {
                let format = format!("Not found key:{}",oldkey);
                Err(format)
            },
        }
    }
    pub fn get_param(&self , key : &str) -> Result<&BitfieldValue,String> {
        match self.bitmap.get(key) {
            Some(value) => Ok(value),
            None => {
                let format = format!("Not found key:{}",key);
                Err(format)
            },
        }
    }
    pub fn del_param(& mut self,key:&str) ->Result<(),String>{
        match self.bitmap.remove(key) {
            Some(_) => Ok(()),
            None => Err(String::from("Delete failed , key not found")),
        }
    }
    pub fn get_value(& self,key:&str) ->Result<i64 , String> {
        let valuetype = self.bitmap.get(key);
        match valuetype {
            Some(value) => {
                let size = mem::size_of::<i64>() as u8;
                if value.length == size * 8 {
                    let resul = (self.bitfield.bitfielddata >> value.start) & -1;
                    Ok(resul)
                }
                else {
                    let resul = (self.bitfield.bitfielddata >> value.start) & ((2i64.pow(value.length as u32)) - 1); //取出指定的位数
                    Ok(resul)
                }
            },
            None => {
                let format = format!("Not found key:{}",key);
                Err(format)
            },
        }
    }
    pub fn set_value(& mut self,key:&str , value : i64) -> Result<(),String> {
        match self.bitmap.get(key) {
            Some(mvalue) => {
                if value >= 2i64.pow(mvalue.length as u32)  { //防止超范围
                    let format = format!("The value is out of range,the maximum value is:{},value:{}",(2i64.pow(mvalue.length as u32)) - 1,value);
                    Err(format)
                }
                else {
                    let tmp = (2i64.pow(mvalue.length as u32)) - 1 << mvalue.start;      //计算预数据
                    let tmp1 = !tmp & self.bitfield.bitfielddata;    //清空指定位数据
                    let tmp2 = value << mvalue.start;
                    self.bitfield.bitfielddata = tmp1 | tmp2 ;
                    Ok(())
                }
            },
            None => {
                let format = format!("Not found key:{}",key);
                Err(format)
            },
        }
    }
    pub fn get_totalbit(& self) -> usize {
        64
    }
}

#[allow(dead_code)]
impl Bitfield<u128> {
    pub fn create_u128(key:String, bitfielddata : u128 , start : u8 , length : u8) -> Result<Bitfield<u128>,String> {
        let size = mem::size_of::<u128>() as u8;
        if start > size * 8 - 1 {
            let format = format!("Illegal starting position;start{}",start);
            return Err(format);
        }
        else if (start + length) > size * 8  {
            let format = format!("Illegal total length;start:{},length:{}",start ,length);
            return Err(format);
        }
        let valuetype = BitfieldValue{start:start,length:length};
        let mut bitmap:HashMap<String,BitfieldValue> = HashMap::new();
        bitmap.insert(key, valuetype);
        Ok(Bitfield{
            bitfield:BitfieldBase{bitfielddata:bitfielddata},
            bitmap:bitmap,
        })
    }
    pub fn traversal(& self){
        println!("{:#?}",self);
    }
    pub fn set_data(& mut self , bitfielddata :u128){
        self.bitfield.bitfielddata = bitfielddata ;
    }
    pub fn get_data(& self) -> u128 {
        self.bitfield.bitfielddata
    }
    pub fn insert(& mut self , key:String , start : u8 , length : u8) -> Result<(), String>{
        let size = mem::size_of::<u128>() as u8;
        if start > size * 8 - 1 {
            let format = format!("Insert failed,Illegal starting position;start{}",start);
            return Err(format);
        }
        else if (start + length) > size * 8  {
            let format = format!("Insert failed,Illegal total length;start:{},length:{}",start ,length);
            return Err(format);
        }
        match self.bitmap.insert(key, BitfieldValue{start:start,length:length}){
            Some(_) =>{
                let format = format!("Insert failed,Key already exists",);
                Err(format)
            },
            None => {Ok(())}
        }
        
    }
    pub fn modify(& mut self ,oldkey:String , newkey:String , start :u8 , length : u8) -> Result<(),String> {
        match self.del_param(oldkey.as_str()) {
            Ok(()) =>{
                match self.insert(newkey, start, length){
                    Ok(()) => Ok(()) ,
                    Err(err) => Err(err) ,
                }
            },
            Err(_) => {
                let format = format!("Not found key:{}",oldkey);
                Err(format)
            },
        }
    }
    pub fn get_param(&self , key : &str) -> Result<&BitfieldValue,String> {
        match self.bitmap.get(key) {
            Some(value) => Ok(value),
            None => {
                let format = format!("Not found key:{}",key);
                Err(format)
            },
        }
    }
    pub fn del_param(& mut self,key:&str) ->Result<(),String>{
        match self.bitmap.remove(key) {
            Some(_) => Ok(()),
            None => Err(String::from("Delete failed , key not found")),
        }
    }
    pub fn get_value(& self,key:&str) ->Result<u128 , String> {
        let valuetype = self.bitmap.get(key);
        match valuetype {
            Some(value) => {
                let size = mem::size_of::<u128>() as u8;
                if value.length == size * 8 {
                    let resul = (self.bitfield.bitfielddata >> value.start) & 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff;
                    Ok(resul)
                }
                else {
                    let resul = (self.bitfield.bitfielddata >> value.start) & ((2u128.pow(value.length as u32)) - 1); //取出指定的位数
                    Ok(resul)
                }
            },
            None => {
                let format = format!("Not found key:{}",key);
                Err(format)
            },
        }
    }
    pub fn set_value(& mut self,key:&str , value : u128) -> Result<(),String> {
        match self.bitmap.get(key) {
            Some(mvalue) => {
                if value >= 2u128.pow(mvalue.length as u32)  { //防止超范围
                    let format = format!("The value is out of range,the maximum value is:{},value:{}",(2u128.pow(mvalue.length as u32)) - 1,value);
                    Err(format)
                }
                else {
                    let tmp = (2u128.pow(mvalue.length as u32)) - 1 << mvalue.start;      //计算预数据
                    let tmp1 = !tmp & self.bitfield.bitfielddata;    //清空指定位数据
                    let tmp2 = value << mvalue.start;
                    self.bitfield.bitfielddata = tmp1 | tmp2 ;
                    Ok(())
                }
            },
            None => {
                let format = format!("Not found key:{}",key);
                Err(format)
            },
        }
    }
    pub fn get_totalbit(& self) -> usize {
        128
    }
}

#[allow(dead_code)]
impl Bitfield<i128> {
    pub fn create_i128(key:String, bitfielddata : i128 , start : u8 , length : u8) -> Result<Bitfield<i128>,String> {
        let size = mem::size_of::<i128>() as u8;
        if start > size * 8 - 1 {
            let format = format!("Illegal starting position;start{}",start);
            return Err(format);
        }
        else if (start + length) > size * 8 {
            let format = format!("Illegal total length;start:{},length:{}",start ,length);
            return Err(format);
        }
        let valuetype = BitfieldValue{start:start,length:length};
        let mut bitmap:HashMap<String,BitfieldValue> = HashMap::new();
        bitmap.insert(key, valuetype);
        Ok(Bitfield{
            bitfield:BitfieldBase{bitfielddata:bitfielddata},
            bitmap:bitmap,
        })
    }
    pub fn traversal(& self){
        println!("{:#?}",self);
    }
    pub fn set_data(& mut self , bitfielddata :i128){
        self.bitfield.bitfielddata = bitfielddata ;
    }
    pub fn get_data(& self) -> i128 {
        self.bitfield.bitfielddata
    }
    pub fn insert(& mut self , key:String , start : u8 , length : u8) -> Result<(), String>{
        let size = mem::size_of::<i128>() as u8;
        if start > size * 8 - 1 {
            let format = format!("Insert failed,Illegal starting position;start{}",start);
            return Err(format);
        }
        else if (start + length) > size * 8  {
            let format = format!("Insert failed,Illegal total length;start:{},length:{}",start ,length);
            return Err(format);
        }
        match self.bitmap.insert(key, BitfieldValue{start:start,length:length}){
            Some(_) =>{
                let format = format!("Insert failed,Key already exists");
                Err(format)
            },
            None => {Ok(())}
        }
        
    }
    pub fn modify(& mut self ,oldkey:String , newkey:String , start :u8 , length : u8) -> Result<(),String> {
        match self.del_param(oldkey.as_str()) {
            Ok(()) =>{
                match self.insert(newkey, start, length){
                    Ok(()) => Ok(()) ,
                    Err(err) => Err(err) ,
                }
            },
            Err(_) => {
                let format = format!("Not found key:{}",oldkey);
                Err(format)
            },
        }
    }
    pub fn get_param(&self , key : &str) -> Result<&BitfieldValue,String> {
        match self.bitmap.get(key) {
            Some(value) => Ok(value),
            None => {
                let format = format!("Not found key:{}",key);
                Err(format)
            },
        }
    }
    pub fn del_param(& mut self,key:&str) ->Result<(),String>{
        match self.bitmap.remove(key) {
            Some(_) => Ok(()),
            None => Err(String::from("Delete failed , key not found")),
        }
    }
    pub fn get_value(& self,key:&str) ->Result<i128 , String> {
        let valuetype = self.bitmap.get(key);
        match valuetype {
            Some(value) => {
                let size = mem::size_of::<i128>() as u8;
                if value.length == size * 8 {
                    let resul = (self.bitfield.bitfielddata >> value.start) & -1;
                    Ok(resul)
                }
                else {
                    let resul = (self.bitfield.bitfielddata >> value.start) & ((2i128.pow(value.length as u32)) - 1); //取出指定的位数
                    Ok(resul)
                }
            },
            None => {
                let format = format!("Not found key:{}",key);
                Err(format)
            },
        }
    }
    pub fn set_value(& mut self,key:&str , value : i128) -> Result<(),String> {
        match self.bitmap.get(key) {
            Some(mvalue) => {
                if value >= 2i128.pow(mvalue.length as u32)  { //防止超范围
                    let format = format!("The value is out of range,the maximum value is:{},value:{}",(2i128.pow(mvalue.length as u32)) - 1,value);
                    Err(format)
                }
                else {
                    let tmp = (2i128.pow(mvalue.length as u32)) - 1 << mvalue.start;      //计算预数据
                    let tmp1 = !tmp & self.bitfield.bitfielddata;    //清空指定位数据
                    let tmp2 = value << mvalue.start;
                    self.bitfield.bitfielddata = tmp1 | tmp2 ;
                    Ok(())
                }
            },
            None => {
                let format = format!("Not found key:{}",key);
                Err(format)
            },
        }
    }
    pub fn get_totalbit(& self) -> usize {
        128
    }
}