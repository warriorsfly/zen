use lettre::transport::smtp::authentication::Credentials;
use lettre::SmtpTransport;

pub fn init_mailer<'a>(mail_server: &'a str, creds: Credentials) -> SmtpTransport {
    SmtpTransport::relay(mail_server)
        .unwrap()
        .credentials(creds)
        .build()
}

#[cfg(test)]
mod tests {

    use super::*;
    use lettre::{Message, Transport};
    const USER_NAME: &str = "519056575@qq.com";
    const PASSWORD: &str = "nudtgkjggnmybjff";
    const MAILER_SERVER: &str = "smtp.qq.com";

    #[test]
    fn test_init_mail() {
        let email = Message::builder()
            .from("Allen Walker<519056575@qq.com>".parse().unwrap())
            .to("Allen Walker<warriorsfly@gmail.com>".parse().unwrap())
            .subject("Happy new async year")
            .body(String::from("Be happy with panda!"))
            .unwrap();
        let creds = Credentials::new(USER_NAME.to_string(), PASSWORD.to_string());
        let mailer = init_mailer(MAILER_SERVER, creds);
        let res = mailer.send(&email);
        assert!(res.is_ok())
    }
}
