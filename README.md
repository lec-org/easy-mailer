# easy-mailer

easy-mailer 是一个基于 Rust 的服务，用于通过 QQ 邮箱（或其他 SMTP 服务器）发送自定义样式的邮件

## 功能

- 使用 Actix-web 作为 Web 框架
- 使用 lettre 库支持 SMTP 协议，可通过 QQ 邮箱 API 发送邮件
- 使用 Handlebars 模板引擎生成自定义邮件内容
- 支持批量发送个性化邮件给多个收件人

## 配置

1. 复制 `config.template.toml` 文件，然后重命名为 `config.toml`

2. 编辑 `config.toml` 文件，填入对应的信息。 注意 `smtp_authorization_code` 是授权码而不是账号密码

## 自定义邮件样式

邮件模板位于 `src/templates/email_template.hbs`


## 使用方法

1. 确保您已经正确配置了 `config.toml` 文件

2. 在项目根目录下运行以下命令启动服务器：
   ```
   cargo run
   ```

3. 服务将在 `http://localhost:8080` 上运行。

4. 要使用内置的 Rust 客户端发送邮件，请在另一个终端窗口中运行：
   ```
   cargo run -- client
   ```

   这将发送一封测试邮件给 `config.toml` 中配置的所有收件人。

5. 如果邮件发送成功，您将看到 "邮件发送成功！" 的消息。

## 注意事项

- 确保您的 QQ 邮箱已开启 SMTP 服务，并使用正确的授权码
- 在生产环境中使用时，请确保妥善保管您的邮箱凭证

## 许可

本项目采用 MIT 许可证。详情请见 [LICENSE](LICENSE) 文件
