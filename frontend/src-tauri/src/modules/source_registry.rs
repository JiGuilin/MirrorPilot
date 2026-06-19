use rusqlite::{params, Connection};
use std::path::PathBuf;
use std::sync::Mutex;

use crate::modules::types::Source;

/// 源注册中心 - 管理内置源和自定义源
pub struct SourceRegistry {
    conn: Mutex<Connection>,
}

impl SourceRegistry {
    /// 初始化数据库
    pub fn new() -> Result<Self, String> {
        let db_path = Self::get_db_path()?;
        let conn = Connection::open(&db_path)
            .map_err(|e| format!("无法打开数据库: {}", e))?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS custom_sources (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                url TEXT NOT NULL,
                package_manager TEXT NOT NULL,
                is_builtin INTEGER NOT NULL DEFAULT 0,
                is_custom INTEGER NOT NULL DEFAULT 1,
                region TEXT NOT NULL DEFAULT 'custom',
                status TEXT NOT NULL DEFAULT 'active',
                latency REAL,
                speed REAL,
                last_tested TEXT
            );

            CREATE TABLE IF NOT EXISTS app_config (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );"
        ).map_err(|e| format!("创建表失败: {}", e))?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    fn get_db_path() -> Result<PathBuf, String> {
        let home = dirs::home_dir().ok_or("无法获取用户主目录")?;
        let dir = home.join(".mirrorpilot");
        std::fs::create_dir_all(&dir).map_err(|e| format!("创建目录失败: {}", e))?;
        Ok(dir.join("sources.db"))
    }

    /// 获取指定包管理器的所有源（内置 + 自定义）
    pub fn get_sources(&self, package_manager: &str) -> Vec<Source> {
        let mut builtin = crate::modules::builtin_sources::get_builtin_sources(
            &crate::modules::types::PackageManager::from_id(package_manager)
                .unwrap_or(crate::modules::types::PackageManager::Npm)
        );

        let custom = self.get_custom_sources(package_manager);
        builtin.extend(custom);
        builtin
    }

    /// 获取自定义源
    pub fn get_custom_sources(&self, package_manager: &str) -> Vec<Source> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT id, name, url, package_manager, is_builtin, is_custom, region, status, latency, speed, last_tested FROM custom_sources WHERE package_manager = ?1"
            )
            .unwrap();

        let rows = stmt
            .query_map(params![package_manager], |row| {
                Ok(Source {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    url: row.get(2)?,
                    package_manager: row.get(3)?,
                    is_builtin: row.get::<_, i32>(4)? != 0,
                    is_custom: row.get::<_, i32>(5)? != 0,
                    region: row.get(6)?,
                    status: row.get(7)?,
                    latency: row.get(8)?,
                    speed: row.get(9)?,
                    last_tested: row.get(10)?,
                })
            })
            .unwrap();

        rows.filter_map(|r| r.ok()).collect()
    }

    /// 添加自定义源
    pub fn add_custom_source(&self, source: &Source) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO custom_sources (id, name, url, package_manager, is_builtin, is_custom, region, status, latency, speed, last_tested) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                source.id,
                source.name,
                source.url,
                source.package_manager,
                source.is_builtin as i32,
                source.is_custom as i32,
                source.region,
                source.status,
                source.latency,
                source.speed,
                source.last_tested,
            ],
        )
        .map_err(|e| format!("添加自定义源失败: {}", e))?;
        Ok(())
    }

    /// 删除自定义源
    pub fn delete_custom_source(&self, id: &str) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM custom_sources WHERE id = ?1", params![id])
            .map_err(|e| format!("删除自定义源失败: {}", e))?;
        Ok(())
    }

    // ponytail: update_source_speed() removed, was dead code

    /// 获取配置值
    pub fn get_config(&self, key: &str) -> Option<String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT value FROM app_config WHERE key = ?1").ok()?;
        let mut rows = stmt.query(params![key]).ok()?;
        rows.next().ok()?.and_then(|row| row.get(0).ok())
    }

    /// 设置配置值
    pub fn set_config(&self, key: &str, value: &str) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO app_config (key, value) VALUES (?1, ?2)",
            params![key, value],
        )
        .map_err(|e| format!("设置配置失败: {}", e))?;
        Ok(())
    }
}
