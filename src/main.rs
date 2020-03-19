use std::path::Path;
use std::thread
use chrono::prelude::*;
use std::fs::OpenOptions;


fn main() {
    // 启动检查
    if Path::new("tmp").exists() {
        // 读→写
        let data = fs::read_to_string("tmp").unwarp()
        let file = OpenOptions::new().append(true).open("report.txt").unwrap()
    }
    write_to_report("",data)
    // 写入现在
    // 轮询
    loop{
        thread::sleep_ms(1000*60)
        let dt= Utc::now();
        let tmp=dt.format("%y%m%d-%H%M").to_string()
        println!("{:?}",tmp)
    }
}
fn read_from_tmp(){ 

}