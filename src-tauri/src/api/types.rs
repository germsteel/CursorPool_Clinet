use serde::{Deserialize, Serialize};

// RESTful API响应结构
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: i32,
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

// 登录响应
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    #[serde(default)]
    pub token: Option<String>,
    #[serde(default)]
    pub user_info: Option<UserInfo>,
}

// 用户信息
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    // 用户总额度
    #[serde(rename = "totalCount")]
    pub total_count: i32,
    // 已使用额度
    #[serde(rename = "usedCount")]
    pub used_count: i32,
    // 过期时间
    #[serde(rename = "expireTime")]
    pub expire_time: String,
    // 用户等级
    pub level: i32,
    // 是否已过期
    #[serde(rename = "isExpired")]
    pub is_expired: bool,
    // 用户名
    pub username: String,
    // 用户级别文本
    #[serde(rename = "code_level")]
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub code_level: String,
}

// 账户信息
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    // 账户ID
    pub id: i32,
    // 账户名
    pub account: String,
    // 密码
    pub password: String,
    // 令牌
    pub token: String,
    // 使用次数
    #[serde(rename = "usage_count")]
    pub usage_count: i32,
    // 状态
    pub status: i32,
    // 创建时间
    #[serde(rename = "create_time")]
    pub create_time: String,
    // 分配时间
    #[serde(rename = "distributed_time")]
    pub distributed_time: String,
    // 更新时间
    #[serde(rename = "update_time")]
    pub update_time: String,
}

// 账户详情
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountDetail {
    pub email: String,
    pub token: String,
}

// 登录请求
#[derive(Debug, Serialize)]
pub struct LoginRequest {
    pub account: String,
    pub password: String,
    pub spread: String,
}

// 检查用户请求
#[derive(Debug, Serialize)]
pub struct CheckUserRequest {
    pub email: String,
}

// 发送验证码请求
#[derive(Debug, Serialize)]
pub struct SendCodeRequest {
    pub email: String,
    pub r#type: String, // register或reset
}

// 注册请求
#[derive(Debug, Serialize)]
pub struct RegisterRequest {
    pub email: String,
    pub code: String,
    pub password: String,
    pub spread: String,
}

// 重置密码请求
#[derive(Debug, Serialize)]
pub struct ResetPasswordRequest {
    pub email: String,
    pub code: String,
    pub password: String,
}

// 激活请求
#[derive(Debug, Serialize)]
pub struct ActivateRequest {
    pub code: String,
}

// 激活响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ActivateResponse {
    #[serde(rename = "expireTime")]
    pub expire_time: i64,
    pub level: i32,
}

// 修改密码请求
#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordChangeRequest {
    #[serde(rename = "oldPassword")]
    pub old_password: String,
    #[serde(rename = "newPassword")]
    pub new_password: String,
    #[serde(rename = "confirmPassword")]
    pub confirm_password: String,
}

// 公告信息
#[derive(Debug, Serialize, Deserialize)]
pub struct PublicInfo {
    pub r#type: String,
    pub closeable: bool,
    pub props: PublicInfoProps,
    pub actions: Vec<PublicInfoAction>,
}

// 公告属性
#[derive(Debug, Serialize, Deserialize)]
pub struct PublicInfoProps {
    pub title: String,
    pub description: String,
}

// 公告动作
#[derive(Debug, Serialize, Deserialize)]
pub struct PublicInfoAction {
    pub r#type: String,
    pub text: String,
    pub url: String,
}

// GPT模型使用情况
#[derive(Debug, Serialize, Deserialize)]
pub struct GptModelUsage {
    // 请求次数
    #[serde(rename = "numRequests")]
    pub num_requests: i32,
    // 总请求次数
    #[serde(rename = "numRequestsTotal")]
    pub num_requests_total: i32,
    // Token数量
    #[serde(rename = "numTokens")]
    pub num_tokens: i32,
    // 最大请求使用量
    #[serde(rename = "maxRequestUsage")]
    pub max_request_usage: Option<i32>,
    // 最大Token使用量
    #[serde(rename = "maxTokenUsage")]
    pub max_token_usage: Option<i32>,
}

// 使用情况
#[derive(Debug, Serialize, Deserialize)]
pub struct UsageInfo {
    pub models: Vec<GptModelUsage>,
}

// 用户信息响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfoResponse {
    pub models: Vec<GptModelUsage>,
}

// Cursor模型使用情况
#[derive(Debug, Serialize, Deserialize)]
pub struct CursorModelUsage {
    #[serde(rename = "numRequests")]
    pub num_requests: i32,
    #[serde(rename = "numRequestsTotal")]
    pub num_requests_total: i32,
    #[serde(rename = "numTokens")]
    pub num_tokens: i64,
    #[serde(rename = "maxRequestUsage")]
    pub max_request_usage: Option<i32>,
    #[serde(rename = "maxTokenUsage")]
    pub max_token_usage: Option<i64>,
}

// Cursor使用情况
#[derive(Debug, Serialize, Deserialize)]
pub struct CursorUsageInfo {
    #[serde(rename = "gpt-4")]
    pub gpt4: CursorModelUsage,
    #[serde(rename = "gpt-3.5-turbo")]
    pub gpt35_turbo: CursorModelUsage,
    #[serde(rename = "gpt-4-32k")]
    pub gpt4_32k: CursorModelUsage,
    #[serde(rename = "startOfMonth")]
    pub start_of_month: String,
}

// Bug报告请求
#[derive(Serialize, Deserialize)]
pub struct BugReportRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "api_key")]
    pub api_key: Option<String>,
    #[serde(rename = "app_version")]
    pub app_version: String,
    #[serde(rename = "os_version")]
    pub os_version: String,
    #[serde(rename = "device_model")]
    pub device_model: String,
    #[serde(rename = "cursor_version")]
    pub cursor_version: String,
    #[serde(rename = "bug_description")]
    pub bug_description: String,
    pub occurrence_time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "screenshot_urls")]
    pub screenshot_urls: Option<Vec<String>>,
    #[serde(rename = "severity")]
    pub severity: String,
}

// 账户池信息
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountPoolInfo {
    pub success: bool,
    #[serde(rename = "account_info")]
    pub account_info: AccountInfo,
    #[serde(rename = "activation_code")]
    pub activation_code: ActivationCode,
}

// 激活码信息
#[derive(Debug, Serialize, Deserialize)]
pub struct ActivationCode {
    pub id: i32,
    pub code: String,
    pub r#type: i32,
    pub name: String,
    pub level: i32,
    pub duration: i32,
    pub max_uses: i32,
    pub used_count: i32,
    pub status: i32,
    pub notes: String,
    #[serde(rename = "activated_at")]
    pub activated_at: String,
    #[serde(rename = "expired_at")]
    pub expired_at: String,
}

// 历史账户记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistoryAccountRecord {
    pub email: String,
    pub token: String,
    pub machine_code: String,
    pub gpt4_count: i32,
    pub gpt35_count: i32,
    pub last_used: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpt4_max_usage: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpt35_max_usage: Option<i32>,
}

// 公告数据结构
#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub content: String,
}

// 公告列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleListResponse {
    pub articles: Vec<Article>,
}
