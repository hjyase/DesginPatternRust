// src/singleton/config.rs
use std::sync::{Mutex, OnceLock};

/// 全局 Carrier 配置单例（替换原 AppConfig）
#[derive(Debug, Clone)]
#[warn(dead_code)]
#[warn(unused)]
pub struct CarrierConfig {
    pub app_name: String,
    pub app_version: String,
    pub max_connections: u32,
    // 可根据需要添加 Carrier 专属字段，比如 carrier_id、api_key 等
    pub carrier_id: String,
    pub api_key: String,
}

static INSTANCE: OnceLock<Mutex<CarrierConfig>> = OnceLock::new();

impl CarrierConfig {
    /// 获取单例实例（全局唯一，懒加载）
    pub fn instance() -> &'static Mutex<Self> {
        INSTANCE.get_or_init(|| {
            Mutex::new(Self {
                app_name: "DesignPatternRust".to_string(),
                app_version: "1.0.0".to_string(),
                max_connections: 100,
                carrier_id: "CAR-20260318".to_string(),
                api_key: "test_api_key_123456".to_string(),
            })
        })
    }

    /// 修改最大连接数（示例方法）
    pub fn set_max_connections(&mut self, num: u32) {
        self.max_connections = num;
    }

    /// 新增 Carrier 专属方法：更新 API Key
    pub fn update_api_key(&mut self, new_key: String) {
        self.api_key = new_key;
    }
}
