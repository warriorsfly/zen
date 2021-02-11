use lettre::{transport::smtp::authentication::Credentials, SmtpTransport};

pub fn init_mailer<'a>(mail_server: &'a str, creds: Credentials) -> SmtpTransport {
    // Open a remote connection to the smtp server
    SmtpTransport::relay(mail_server)
        .unwrap()
        .credentials(creds)
        .build()
}
