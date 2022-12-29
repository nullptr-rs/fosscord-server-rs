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
            avatar: user.avatar.clone(),
            discriminator: user.discriminator.clone(),
            id: user.id.clone(),
            public_flags: user.public_flags,
            username: user.name.clone(),
        }
    }*/
}
