use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor, message::header::ContentType,
};

#[derive(Clone)]
pub struct Mailer {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    from: String,
}

impl Mailer {
    pub fn from_env() -> Self {
        let host = std::env::var("SMTP_HOST").expect("SMTP_HOST manquante");
        let port: u16 = std::env::var("SMTP_PORT")
            .expect("SMTP_PORT manquante")
            .parse()
            .expect("SMTP_PORT invalide");
        let from = std::env::var("MAIL_FROM").expect("MAIL_FROM manquante");

        // Sans TLS : uniquement pour Mailpit en dev
        let transport = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(host)
            .port(port)
            .build();

        Self { transport, from }
    }

    pub async fn send(
        &self,
        to: &str,
        subject: &str,
        body: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let email = Message::builder()
            .from(self.from.parse()?)
            .to(to.parse()?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body)?;

        self.transport.send(email).await?;
        Ok(())
    }
}
