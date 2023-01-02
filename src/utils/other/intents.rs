pub enum Intents {
    Guilds = 1 << 0,
    GuildMembers = 1 << 1,
    GuildBans = 1 << 2,
    GuildEmojis = 1 << 3,
    GuildIntegrations = 1 << 4,
    GuildWebhooks = 1 << 5,
    GuildInvites = 1 << 6,
    GuildVoiceStates = 1 << 7,
    GuildPresences = 1 << 8,
    GuildMessagesMetadata = 1 << 9,
    GuildMessageReactions = 1 << 10,
    GuildMessageTyping = 1 << 11,
    DirectMessages = 1 << 12,
    DirectMessageReactions = 1 << 13,
    DirectMessageTyping = 1 << 14,
    GuildMessagesContent = 1 << 15,
    GuildPolicies = 1 << 20,
    GuildPolicyExecution = 1 << 21,
    LiveMessageComposition = 1 << 32,
    GuildRoutes = 1 << 41,
    DirectMessagesThreads = 1 << 42,
    JumboEvents = 1 << 43,
    Lobbies = 1 << 44,
    InstanceRoutes = 1 << 60,
    InstanceGuildChanges = 1 << 61,
    InstancePolicyUpdates = 1 << 62,
    InstanceUserUpdates = 1 << 63
}