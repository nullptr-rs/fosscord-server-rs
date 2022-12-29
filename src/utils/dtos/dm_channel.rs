use crate::utils::dtos::user::MinimalPublicUserDTO;

pub struct DmChannelDTO {
    pub icon: String,
    pub id: String,
    pub last_message_id: String,
    pub name: String,
    pub origin_channel_id: Option<String>,
    pub owner_id: Option<String>,
    pub recipients: Vec<MinimalPublicUserDTO>,
    pub _type: u64,
}

impl DmChannelDTO {
    /*pub async fn from(channel: &Channel, excluded_recipients: Vec<String>, origin_channel_id: Option<String>) -> Self {

    }*/
}
