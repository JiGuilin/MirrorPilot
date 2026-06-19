use std::time::Instant;

use reqwest::Client;
use tauri::{AppHandle, Emitter};

use crate::modules::types::{SpeedTestProgress, SpeedTestResult};

/// 网络测速模块
pub struct NetworkTester {
    client: Client,
}

impl NetworkTester {
    pub fn new(timeout_seconds: u64) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(timeout_seconds))
            .no_proxy()
            .build()
            .unwrap_or_default();

        Self { client }
    }

    /// 快速检测 - 只测延迟
    pub async fn test_latency(&self, url: &str) -> Result<f64, String> {
        let test_url = Self::build_test_url(url);
        let start = Instant::now();

        let result = self
            .client
            .head(&test_url)
            .send()
            .await
            .map_err(|e| format!("请求失败: {}", e))?;

        if !result.status().is_success() && result.status().as_u16() != 302 {
            // 有些源不支持 HEAD，尝试 GET
            let start2 = Instant::now();
            self.client
                .get(&test_url)
                .send()
                .await
                .map_err(|e| format!("请求失败: {}", e))?;
            return Ok(start2.elapsed().as_millis() as f64);
        }

        Ok(start.elapsed().as_millis() as f64)
    }

    /// 完整测速 - 延迟 + 下载速度
    pub async fn test_speed(&self, url: &str) -> Result<(f64, f64), String> {
        // 先测延迟
        let latency = self.test_latency(url).await?;

        // 再测下载速度
        let test_url = Self::build_test_url(url);
        let start = Instant::now();

        let response = self
            .client
            .get(&test_url)
            .send()
            .await
            .map_err(|e| format!("下载失败: {}", e))?;

        let bytes = response
            .bytes()
            .await
            .map_err(|e| format!("读取响应失败: {}", e))?;

        let elapsed_secs = start.elapsed().as_secs_f64();
        let speed_kbps = if elapsed_secs > 0.0 {
            (bytes.len() as f64 / 1024.0) / elapsed_secs
        } else {
            0.0
        };

        Ok((latency, speed_kbps))
    }

    /// 批量测速
    pub async fn test_sources(
        &self,
        sources: Vec<(String, String, String)>, // (id, name, url)
        app_handle: &AppHandle,
    ) -> Vec<SpeedTestResult> {
        let total = sources.len();
        let mut results = Vec::new();

        for (i, (id, name, url)) in sources.iter().enumerate() {
            // 发送进度事件
            let _ = app_handle.emit(
                "speed-test-progress",
                SpeedTestProgress {
                    current: i + 1,
                    total,
                    current_source_name: name.clone(),
                    results: results.clone(),
                },
            );

            let result = match self.test_speed(url).await {
                Ok((latency, speed)) => SpeedTestResult {
                    source_id: id.clone(),
                    source_name: name.clone(),
                    source_url: url.clone(),
                    latency_ms: Some(latency),
                    speed_kbps: Some(speed),
                    success: true,
                    error_message: None,
                },
                Err(e) => SpeedTestResult {
                    source_id: id.clone(),
                    source_name: name.clone(),
                    source_url: url.clone(),
                    latency_ms: None,
                    speed_kbps: None,
                    success: false,
                    error_message: Some(e),
                },
            };

            results.push(result);
        }

        // 发送完成事件
        let _ = app_handle.emit(
            "speed-test-progress",
            SpeedTestProgress {
                current: total,
                total,
                current_source_name: "完成".to_string(),
                results: results.clone(),
            },
        );

        results
    }

    /// 构建测试 URL - 在源地址基础上添加一个可访问的路径
    fn build_test_url(url: &str) -> String {
        let url = url.trim_end_matches('/');

        // 根据包管理器类型构建测试路径
        if url.contains("npm") || url.contains("npmmirror") {
            format!("{}/lodash", url)
        } else if url.contains("pypi") || url.contains("pip") {
            format!("{}/pip/", url)
        } else if url.contains("goproxy") || url.contains("proxy.golang") {
            format!("{}/github.com/gin-gonic/gin/@v/list", url)
        } else if url.contains("maven") || url.contains("maven2") {
            format!("{}/maven-metadata.xml", url)
        } else if url.contains("docker") {
            format!("{}/v2/", url)
        } else {
            format!("{}/", url)
        }
    }
}
