use std::ops::{Deref, DerefMut};
use chrono::format::Item;
use tokio_util::sync::CancellationToken;
use crate::http::entity::{Notification, NotificationCreationConfig};
use crate::http::entity::list_channels_response::Items;
use crate::http::storage_client::{Error, StorageClient};

pub struct ChannelHandle<'a> {
    bucket: &'a str,
    storage_client: &'a StorageClient,
    entity: Items
}

impl Deref for ChannelHandle {
    type Target = Items;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}

impl <'a> ChannelHandle<'a> {
    pub(crate) fn new(
        bucket: &'a str,
        storage_client: &'a StorageClient,
        entity: Items
    ) -> Self {
        Self {
            bucket,
            storage_client,
            entity
        }
    }

    pub async fn stop(&self, cancel: Option<CancellationToken>) -> Result<(), Error> {
        self.storage_client.stop_channel(&self.entity, cancel).await
    }
}

