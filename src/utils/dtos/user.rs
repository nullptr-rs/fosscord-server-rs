pub struct MinimalPublicUserDTO {
    pub avatar: Option<String>,
    pub discriminator: String,
    pub id: String,
    pub public_flags: u64,
    pub username: String,
}

impl MinimalPublicUserDTO {
    /*pub fn from_user(user: &User) -> MinimalPublicUserDTO {
        MinimalPublicUserDTO {
            Avatar: user.Avatar.clone(),
            discriminator: user.discriminator.clone(),
            id: user.id.clone(),
            PublicFlags: user.PublicFlags,
            username: user.name.clone(),
        }
    }*/
}
