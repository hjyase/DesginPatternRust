pub mod carrier_config; //这里是文件名，让编译系统知道有一个carrier_cofig.rs的文件，让它参与编译

pub use carrier_config::CarrierConfig; // 可选，但强烈建议加上，方便外部模块调用
