CREATE TABLE user_settings (
    id varchar(255) NOT NULL,
    afk_timeout int NULL,
    allow_accessibility_detection tinyint NULL,
    animate_emoji tinyint NULL,
    animate_stickers int NULL,
    contact_sync_enabled tinyint NULL,
    convert_emoticons tinyint NULL,
    custom_status text NULL,
    default_guilds_restricted tinyint NULL,
    detect_platform_accounts tinyint NULL,
    developer_mode tinyint NULL,
    disable_games_tab tinyint NULL,
    enable_tts_command tinyint NULL,
    explicit_content_filter int NULL,
    friend_source_flags text NULL,
    gateway_connected tinyint NULL,
    gif_auto_play tinyint NULL,
    guild_folders text NULL,
    guild_positions text NULL,
    inline_attachment_media tinyint NULL,
    inline_embed_media tinyint NULL,
    locale varchar(255) NULL,
    message_display_compact tinyint NULL,
    native_phone_integration_enabled tinyint NULL,
    render_embeds tinyint NULL,
    render_reactions tinyint NULL,
    restricted_guilds text NULL,
    show_current_game tinyint NULL,
    status varchar(255) NULL,
    stream_notifications_enabled tinyint NULL,
    theme varchar(255) NULL,
    timezone_offset int NULL,

    PRIMARY KEY (id)
) ENGINE = InnoDB