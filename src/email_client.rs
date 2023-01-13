use crate::domain::SubscriberEmail;
use aws_sdk_sesv2::model::{Body, Content, Destination, EmailContent, Message};

pub struct EmailClient {
    email_client: aws_sdk_sesv2::Client,
    sender: SubscriberEmail,
}

impl EmailClient {
    pub fn new(email_client: aws_sdk_sesv2::Client, sender: SubscriberEmail) -> Self {
        EmailClient {
            email_client,
            sender,
        }
    }
    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_body: &str,
        text_body: &str,
    ) -> Result<(), aws_sdk_sesv2::Error> {
        self.email_client
            .send_email()
            .from_email_address(self.sender.as_ref())
            .destination(
                Destination::builder()
                    .to_addresses(recipient.as_ref())
                    .build(),
            )
            .content(
                EmailContent::builder()
                    .simple(
                        Message::builder()
                            .subject(Content::builder().data(subject).charset("UTF-8").build())
                            .body(
                                Body::builder()
                                    .text(
                                        Content::builder().data(text_body).charset("UTF-8").build(),
                                    )
                                    .html(
                                        Content::builder().data(html_body).charset("UTF-8").build(),
                                    )
                                    .build(),
                            )
                            .build(),
                    )
                    .build(),
            )
            .send()
            .await?;
        Ok(())
    }
}
