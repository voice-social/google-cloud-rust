use std::ops::{Deref, DerefMut};
use tokio_util::sync::CancellationToken;
use crate::http::entity::{Notification, NotificationCreationConfig};
use crate::http::storage_client::{Error, StorageClient};

pub struct NotificationHandle<'a> {
    bucket: &'a str,
    storage_client: &'a StorageClient,
    entity: Notification
}

impl Deref for NotificationHandle {
    type Target = Notification;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}

impl <'a> NotificationHandle<'a> {
    pub(crate) fn new(
        bucket: &'a str,
        storage_client: &'a StorageClient,
        entity: Notification
    ) -> Self {
        Self {
            bucket,
            storage_client,
            entity
        }
    }

    pub async fn delete(self, cancel: Option<CancellationToken>) -> Result<(), Error> {
        self.storage_client.delete_notification(self.name.as_str(), &self.entity.id, cancel).await
    }
}

