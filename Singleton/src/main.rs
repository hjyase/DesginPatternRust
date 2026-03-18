mod singleton; // 有一个singleton的模块即文件夹

use crate::singleton::CarrierConfig; //使用该模块文件下下导出的结构

fn main() {
    println!("Hello, Singleton!");
    {
        let mut cfg = CarrierConfig::instance().lock().unwrap();
        let _s = cfg.set_max_connections(5);

        let n = cfg.max_connections;
        println!("cfg max connection is {}", n);
    }

    let cfg2 = CarrierConfig::instance().lock().unwrap();
    let n2 = cfg2.max_connections;
    println!("cfg2 max connection is {}", n2);
}
