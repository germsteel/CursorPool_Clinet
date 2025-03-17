use crate::api::client::ApiClient;
use crate::api::endpoints::report_bug;
use tauri::State;
use tracing::{error, warn};

pub struct ErrorReporter;

impl ErrorReporter {
    /// 上报错误信息
    pub async fn report_error(
        client: State<'_, ApiClient>,
        function_name: &str,
        error: &str,
        api_key: Option<String>,
        severity: Option<String>,
    ) {
        // 记录错误到日志系统
        let severity_str = severity.as_deref().unwrap_or("low");
        match severity_str {
            "high" => error!(target: "error_report", "严重错误 - 函数: {}, 错误: {}", function_name, error),
            "medium" => error!(target: "error_report", "中等错误 - 函数: {}, 错误: {}", function_name, error),
            _ => warn!(target: "error_report", "轻微错误 - 函数: {}, 错误: {}", function_name, error),
        }
        
        // 构建错误描述
        let bug_description = format!("函数: {}\n错误: {}", function_name, error);

        // 使用 report_bug 函数上报错误
        if let Err(e) = report_bug(
            client,
            severity.unwrap_or("low".to_string()),
            bug_description,
            api_key,
            None,
            None,
        )
        .await
        {
            error!(target: "error_report", "错误上报失败: {}", e);
        }
    }
}
