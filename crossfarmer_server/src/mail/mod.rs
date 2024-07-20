mod credentials;

use lettre::{Message, SmtpTransport, Transport};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use credentials::*;

pub fn create() -> SmtpTransport {
	let credentials = Credentials::new(USERNAME.to_string(), PASSWORD.to_string());
	SmtpTransport::relay("smtp.gmail.com").unwrap().credentials(credentials).build()
}