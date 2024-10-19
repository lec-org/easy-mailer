use crate::config::Config;
use crate::email::EmailService;
use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::Local;
use serde_json::json;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from easy-mailer!")
}

#[post("/send-emails")]
async fn send_emails(
    config: web::Data<Config>,
    email_data: web::Json<serde_json::Value>,
) -> impl Responder {
    println!("接收到发送邮件请求");

    let email_service = EmailService::new(config.get_ref().clone());

    let subject = email_data
        .get("subject")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    println!("邮件主题: {}", subject);

    let mut template_data = email_data
        .get("template_data")
        .cloned()
        .unwrap_or_else(|| json!({}));

    // 如果没有提供日期，使用当前日期
    if !template_data.get("date").is_some() {
        let current_date = Local::now().format("%Y年%m月%d日").to_string();
        template_data["date"] = json!(current_date);
    }

    // 如果没有提供消息，使用默认消息
    if !template_data.get("message").is_some() {
        template_data["message"] = json!("欢迎加入乐程LEC！我们期待与您一起创造精彩。");
    }

    println!("模板数据: {:?}", template_data);

    match email_service.send_emails(subject, &template_data).await {
        Ok(_) => {
            println!("邮件发送成功");
            HttpResponse::Ok().body("Emails sent successfully")
        }
        Err(e) => {
            println!("邮件发送失败: {}", e);
            HttpResponse::InternalServerError().body(format!("Failed to send emails: {}", e))
        }
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    println!("配置路由");
    cfg.service(hello).service(send_emails);
}
