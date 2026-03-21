mod singleton; // 有一个singleton的模块即文件夹

use crate::singleton::{CarrierConfig, deparkment}; //使用该模块文件下下导出的结构
use crate::singleton::deparkment::Deparkment; //使用该模块文件下下导出的结构
use crate::singleton::group::Group; //使用该模块文件下下导出的结构

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

    let deparkment = Deparkment::get_instance();

    let deparkment_name = deparkment.get_name();
    println!("Deparkment name: {}", deparkment_name);

    let group_name = deparkment.get_group().get_group_name();
    println!("Deparkment group name: {}", group_name);


    let deparkment_ref = Deparkment::get_instance();
    let deparkment_ref_name = deparkment_ref.get_name();
    println!("Deparkment ref name: {}", deparkment_ref_name);

    deparkment_ref.get_group().set_group_name("New Platform".to_string());
    let updated_group_name = deparkment.get_group().get_group_name();
    println!("Deparkment ref Updated group name: {}", updated_group_name);
}
