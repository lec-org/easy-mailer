use crate::config::Config;
use handlebars::Handlebars;
use lettre::message::{header, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use serde_json::Value;

pub struct EmailService {
    config: Config,
    handlebars: Handlebars<'static>,
    mailer: AsyncSmtpTransport<Tokio1Executor>,
}

impl EmailService {
    pub fn new(config: Config) -> Self {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_file("email_template", "src/templates/email_template.hbs")
            .expect("Failed to register email template");

        let creds = Credentials::new(
            config.from_email.clone(),
            config.smtp_authorization_code.clone(),
        );

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&config.smtp_server)
            .unwrap()
            .credentials(creds)
            .port(config.smtp_port)
            .build();

        EmailService {
            config,
            handlebars,
            mailer,
        }
    }

    pub async fn send_emails(
        &self,
        subject: &str,
        name: &str,
        email: &str,
        template_data: &Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("将发送给 {}({}) {:?}", name, email, template_data);
        let body = self.handlebars.render("email_template", template_data)?;

        let email = Message::builder()
            .from(self.config.from_email.parse()?)
            .to(format!("{} <{}>", name, email).parse()?)
            .subject(subject)
            .multipart(
                MultiPart::mixed().singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(body),
                ),
            )?;

        self.mailer.send(email).await?;

        Ok(())
    }
}
