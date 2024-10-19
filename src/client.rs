use chrono::Local;
use reqwest::Client;
use serde_json::json;

pub async fn send_email_request() -> Result<(), Box<dyn std::error::Error>> {
    println!("客户端: 开始批量发送邮件请求");

    let client = Client::new();
    let url = "http://localhost:8080/send-emails";

    let current_date = Local::now().format("%Y年%m月%d日").to_string();

    let config = crate::config::Config::new().expect("Failed to load configuration");

    for recipient in &config.recipients {
        let payload = json!({
            "subject": recipient.subject,
            "template_data": {
                "message": recipient.message,
                "date": current_date
            }
        });

        println!("客户端: 发送请求到 {} 给 {}", url, recipient.name);
        println!("客户端: 请求负载 {:?}", payload);

        let response = client.post(url).json(&payload).send().await?;

        println!("客户端: 收到响应，状态码: {}", response.status());

        if response.status().is_success() {
            println!("客户端: 邮件发送成功给 {}！", recipient.name);
            println!("客户端: 响应内容: {}", response.text().await?);
        } else {
            println!(
                "客户端: 邮件发送失败给 {}。状态码: {}",
                recipient.name,
                response.status()
            );
            println!("客户端: 错误信息: {}", response.text().await?);
        }
    }

    println!("客户端: 批量邮件发送完成");

    Ok(())
}
