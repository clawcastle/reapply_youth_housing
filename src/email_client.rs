extern crate lettre;
use chrono::prelude::*;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, SmtpTransportBuilder, Transport, SecurityLevel};

fn create_confirmation_email(recipient_address: &str) {
    let email_body = format!("We have successfully logged in and reapplied for your application at UngdomsBolig Aarhus, at {}", Local::now())

    let email = Message::builder()
        .from("test@gmail.com")
        .to(recipient_address)
        .subject("Successfully reapplied for at ungdomsboligaarhus.")
        .body(email_body);

    let email_client = SmtpTransportBuilder::new(("smtp.googlemail.com", 587u16))
        .unwrap()
        .credentials("test@gmail.com", "password")
        .security_level(SecurityLevel::AlwaysEncrypt)
        .smtp_utf8(true)
        .build();

    let result = email_client.send(email.clone());
    match result {
        Ok(_) => println!("Confirmation email sent!"),
        Err(err) => println!("Error happened while trying to send confirmation email: {}", err)
    }

}
