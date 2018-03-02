use super::Method;
use types::Message;
use marker::ReplyMarkup;

#[derive(Debug, Builder, Serialize)]
pub struct SendMessage<M: ReplyMarkup> {
    chat_id: i32,
    text: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")] parse_mode: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")] disable_web_page_view: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")] disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")] reply_to_message_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")] reply_markup: Option<M>,
}

impl<M: ReplyMarkup> Method for SendMessage<M> {
    type Response = Message;
    const PATH: &'static str = "sendMessage";
}
