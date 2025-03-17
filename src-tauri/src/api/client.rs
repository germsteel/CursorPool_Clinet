use crate::api::interceptor::{
    is_auth_required_url, save_auth_token, AuthInterceptor, Interceptor,
};
use crate::database::Database;
use reqwest::header::HeaderValue;
use reqwest::{Client, Request, Response};
use std::sync::Arc;
use std::time::Duration;
use tauri::AppHandle;
use tauri::Manager;
use tracing::error;

/// HTTP 请求客户端，支持拦截器机制
pub struct ApiClient {
    client: Arc<Client>,
    interceptors: Vec<Box<dyn Interceptor>>,
    app_handle: Option<Arc<AppHandle>>,
}

impl ApiClient {
    /// 创建 API 客户端实例
    pub fn new(app_handle: Option<AppHandle>) -> Self {
        let client = Arc::new(
            Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client"),
        );

        let mut interceptors = Vec::new();
        if let Some(handle) = &app_handle {
            interceptors
                .push(Box::new(AuthInterceptor::new(Arc::new(handle.clone())))
                    as Box<dyn Interceptor>);
        }

        Self {
            client,
            interceptors,
            app_handle: app_handle.map(Arc::new),
        }
    }

    /// 获取基础URL，优先使用inbound配置
    pub fn get_base_url(&self) -> String {
        use crate::api::inbound::get_current_inbound_url;
        
        // 如果有AppHandle，尝试获取当前线路URL
        if let Some(handle) = &self.app_handle {
            if let Some(db) = handle.try_state::<crate::database::Database>() {
                return get_current_inbound_url(&db);
            }
        }
        
        // 回退到默认URL
        "https://pool.52ai.org/api".to_string()
    }

    /// 发送 HTTP 请求
    pub async fn send(&self, mut request: Request) -> Result<Response, reqwest::Error> {
        let url: String = request.url().to_string();
        let method = request.method().to_string();

        if is_auth_required_url(&url) {
            for interceptor in &self.interceptors {
                if let Err(_) = interceptor.intercept(&mut request) {
                    continue;
                }
            }
        }

        let response = self.client.execute(request).await.map_err(|e| {
            error!(
                target: "http_client",
                "HTTP请求失败 - 方法: {}, URL: {}, 错误: {}",
                method, url, e
            );
            e
        })?;

        if self.app_handle.is_none() {
            return Ok(response);
        }

        let handle = self.app_handle.as_ref().unwrap();
        let db = handle.state::<Database>();
        let status = response.status();
        let url_str = url.clone();

        let response_text = response.text().await.map_err(|e| {
            error!(
                target: "http_client",
                "读取响应文本失败 - 方法: {}, URL: {}, 状态码: {}, 错误: {}",
                method, url_str, status, e
            );
            e
        })?;

        if url_str.contains("/user/updatePassword") {
            if let Ok(response_json) = serde_json::from_str::<serde_json::Value>(&response_text) {
                if response_json["status"] == 200 {
                    if let Err(e) = clear_auth_token(&db).await {
                        error!(
                            target: "http_client",
                            "清除认证令牌失败 - URL: {}, 错误: {}", 
                            url_str, e
                        );
                    }
                }
            }
        } else {
            if let Err(e) = save_auth_token(&db, &url_str, &response_text).await {
                error!(
                    target: "http_client",
                    "保存认证令牌失败 - URL: {}, 错误: {}", 
                    url_str, e
                );
            }
        }

        Ok(Response::from(
            http::Response::builder()
                .status(status)
                .body(response_text)
                .unwrap(),
        ))
    }

    /// 创建 GET 请求
    pub fn get(&self, url: impl AsRef<str>) -> RequestBuilder {
        RequestBuilder {
            inner: self.client.get(url.as_ref()),
            client: self,
        }
    }

    /// 创建 POST 请求
    pub fn post(&self, url: impl AsRef<str>) -> RequestBuilder {
        RequestBuilder {
            inner: self.client.post(url.as_ref()),
            client: self,
        }
    }

    /// 创建 PUT 请求
    pub fn put(&self, url: impl AsRef<str>) -> RequestBuilder {
        RequestBuilder {
            inner: self.client.put(url.as_ref()),
            client: self,
        }
    }

    /// 创建 DELETE 请求
    pub fn delete(&self, url: impl AsRef<str>) -> RequestBuilder {
        RequestBuilder {
            inner: self.client.delete(url.as_ref()),
            client: self,
        }
    }
}

/// HTTP 请求构建器
pub struct RequestBuilder<'a> {
    inner: reqwest::RequestBuilder,
    client: &'a ApiClient,
}

impl<'a> RequestBuilder<'a> {
    /// 发送请求
    pub async fn send(self) -> Result<Response, reqwest::Error> {
        // 在构建请求前获取内部构建器的调试信息
        let debug_info = format!("{:?}", self.inner);
        
        let request = match self.inner.build() {
            Ok(req) => req,
            Err(e) => {
                error!(
                    target: "http_client",
                    "构建HTTP请求失败 - 请求: {}, 错误: {}", 
                    debug_info, e
                );
                return Err(e);
            }
        };
        self.client.send(request).await
    }

    /// 添加表单数据
    pub fn form<T: serde::Serialize + ?Sized>(self, form: &T) -> Self {
        Self {
            inner: self.inner.form(form),
            client: self.client,
        }
    }

    /// 添加 JSON 数据
    pub fn json<T: serde::Serialize + ?Sized>(self, json: &T) -> Self {
        Self {
            inner: self.inner.json(json),
            client: self.client,
        }
    }

    /// 添加请求头
    pub fn header(self, key: &str, value: &str) -> Self {
        Self {
            inner: self
                .inner
                .header(key, HeaderValue::from_str(value).unwrap()),
            client: self.client,
        }
    }

    /// 添加 multipart 表单数据
    pub fn multipart<T: IntoIterator<Item = (String, String)>>(self, form: T) -> Self {
        let mut form_builder = reqwest::multipart::Form::new();
        for (key, value) in form {
            form_builder = form_builder.text(key, value);
        }

        Self {
            inner: self.inner.header("Accept", "*/*").multipart(form_builder),
            client: self.client,
        }
    }
}

/// 清除认证令牌
async fn clear_auth_token(db: &tauri::State<'_, Database>) -> Result<(), String> {
    db.delete_item("user.info.token")
        .map_err(|e| e.to_string())?;
    Ok(())
}
