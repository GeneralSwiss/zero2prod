use crate::domain::subscriber_name::SubscriberName;
use crate::domain::SubscriberEmail;

#[derive(serde::Deserialize)]
pub struct Subscriber {
    pub name: SubscriberName,
    pub email: SubscriberEmail,
}
