use crate::key_config::KeyConfig;

#[derive(Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct CommandText {
    pub name: String,
    pub group: &'static str,
}

impl CommandText {
    pub const fn new(name: String, group: &'static str) -> Self {
        Self { name, group }
    }
}

pub struct CommandInfo {
    pub text: CommandText,
}

impl CommandInfo {
    pub const fn new(text: CommandText) -> Self {
        Self { text }
    }
}

static CMD_GROUP_GENERAL: &str = "-- General --";
static CMD_GROUP_INBOX: &str = "-- Inbox --";
// static CMD_GROUP_SEARCH: &str = "-- Search --";

pub fn scroll(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!("Scroll up/down [{},{}]", "up", "down"),
        CMD_GROUP_GENERAL,
    )
}

pub fn open_pdf(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!("Open [{}]", ""),
        // NOTE: CMD_GROUP_SEARCH may be needed
        CMD_GROUP_INBOX,
    )
}
