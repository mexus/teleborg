use marker::ReplyMarkup;

/// Represents `ForceReply` from the Telegram bot API.
#[derive(Serialize, Debug)]
pub struct ForceReply {
    force_reply: bool,
    #[serde(skip_serializing_if = "Option::is_none")] selective: Option<bool>,
}

impl ForceReply {
    /// Constructs a new `ForceReply`.
    pub fn new(force_reply: bool, selective: Option<bool>) -> ForceReply {
        ForceReply {
            force_reply: force_reply,
            selective: selective,
        }
    }
}

impl ReplyMarkup for ForceReply {}
