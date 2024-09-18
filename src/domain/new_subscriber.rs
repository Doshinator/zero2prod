//! src/domain/new_subscriber.rs

use crate domain::subscriber_name::SubscriberName;

pub struct NewSubscriber {
    email: SubscriberEmail,
    name: SubscriberName,
}
