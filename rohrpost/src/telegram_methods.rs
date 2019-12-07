use crate::helpers::Or;
use crate::telegram_types::*;
use serde::Serialize;
type Integer = i64;
type Boolean = bool;
type True = bool;
type Float = f64;
//unsupported types
type InputFile = ();
///Marker Trait for TelegramMethod structs
pub trait TelegramMethod {
    const method_name: &'static str;
}
///
///Use this method to specify a url and receive incoming updates via an outgoing webhook. Whenever there is an update for the bot, we will send an HTTPS POST request to the specified url, containing a JSON-serialized Update. In case of an unsuccessful request, we will give up after a reasonable amount of attempts. Returns True on success.
///
///If you'd like to make sure that the Webhook request comes from Telegram, we recommend using a secret path in the URL, e.g. https://www.example.com/<token>. Since nobody else knows your bot‘s token, you can be pretty sure it’s us.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct setWebhook {
    ///HTTPS url to send updates to. Use an empty string to remove webhook integration
    pub url: String,
    ///Upload your public key certificate so that the root certificate in use can be checked. See our self-signed guide for details.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<InputFile>,
    ///Maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery, 1-100. Defaults to 40. Use lower values to limit the load on your bot‘s server, and higher values to increase your bot’s throughput.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<Integer>,
    ///List the types of updates you want your bot to receive. For example, specify [“message”, “edited_channel_post”, “callback_query”] to only receive updates of these types. See Update for a complete list of available update types. Specify an empty list to receive all updates regardless of type (default). If not specified, the previous setting will be used.
    #[builder(default)]
    ///
    ///Please note that this parameter doesn't affect updates created before the call to the setWebhook, so unwanted updates may be received for a short period of time.
    ///
    ///    Notes
    ///    1. You will not be able to receive updates using getUpdates for as long as an outgoing webhook is set up.
    ///    2. To use a self-signed certificate, you need to upload your public key certificate using certificate parameter. Please upload as InputFile, sending a String will not work.
    ///    3. Ports currently supported for Webhooks: 443, 80, 88, 8443.
    ///
    ///    NEW! If you're having any trouble setting up webhooks, please check out this amazing guide to Webhooks.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}
impl TelegramMethod for setWebhook {
    const method_name: &'static str = "setWebhook";
}
///
///Use this method to remove webhook integration if you decide to switch back to getUpdates. Returns True on success. Requires no parameters.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct deleteWebhook {}
impl TelegramMethod for deleteWebhook {
    const method_name: &'static str = "deleteWebhook";
}
///
///Use this method to get current webhook status. Requires no parameters. On success, returns a WebhookInfo object. If the bot is using getUpdates, will return an object with the url field empty.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct getWebhookInfo {}
impl TelegramMethod for getWebhookInfo {
    const method_name: &'static str = "getWebhookInfo";
}
///
///Contains information about the current status of a webhook.
///Field 	Type 	Description
///url 	String 	Webhook URL, may be empty if webhook is not set up
///has_custom_certificate 	Boolean 	True, if a custom certificate was provided for webhook certificate checks
///pending_update_count 	Integer 	Number of updates awaiting delivery
///last_error_date 	Integer 	Optional. Unix time for the most recent error that happened when trying to deliver an update via webhook
///last_error_message 	String 	Optional. Error message in human-readable format for the most recent error that happened when trying to deliver an update via webhook
///max_connections 	Integer 	Optional. Maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery
///allowed_updates 	Array of String 	Optional. A list of update types the bot is subscribed to. Defaults to all update types
///getMe
///
///A simple method for testing your bot's auth token. Requires no parameters. Returns basic information about the bot in form of a User object.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct WebhookInfo {}
impl TelegramMethod for WebhookInfo {
    const method_name: &'static str = "WebhookInfo";
}
///
///Use this method to send text messages. On success, the sent Message is returned.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct sendMessage {
    ///Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Or<Integer, String>,
    ///Text of the message to be sent
    pub text: String,
    ///Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in your bot's message.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    ///Disables link previews for links in this message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<Boolean>,
    ///Sends the message silently. Users will receive a notification with no sound.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    ///If the message is a reply, ID of the original message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<Integer>,
    ///Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[builder(default)]
    ///Formatting options
    ///
    ///The Bot API supports basic formatting for messages. You can use bold and italic text, as well as inline links and pre-formatted code in your bots' messages. Telegram clients will render them accordingly. You can use either markdown-style or HTML-style formatting.
    ///
    ///Note that Telegram clients will display an alert to the user before opening an inline link (‘Open this link?’ together with the full URL).
    ///
    ///Links tg://user?id=<user_id> can be used to mention a user by their id without using a username. Please note:
    ///
    ///    These links will work only if they are used inside an inline link. For example, they will not work, when used in an inline keyboard button or in a message text.
    ///    These mentions are only guaranteed to work if the user has contacted the bot in the past, has sent a callback query to the bot via inline button or is a member in the group where he was mentioned.
    ///
    ///Markdown style
    ///
    ///To use this mode, pass Markdown in the parse_mode field when using sendMessage. Use the following syntax in your message:
    ///
    ///*bold text*
    ///_italic text_
    ///[inline URL](http://www.example.com/)
    ///[inline mention of a user](tg://user?id=123456789)
    ///`inline fixed-width code`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<
        Or<InlineKeyboardMarkup, Or<ReplyKeyboardMarkup, Or<ReplyKeyboardRemove, ForceReply>>>,
    >,
}
impl TelegramMethod for sendMessage {
    const method_name: &'static str = "sendMessage";
}
///pre-formatted fixed-width code block
///```
///
///HTML style
///
///To use this mode, pass HTML in the parse_mode field when using sendMessage. The following tags are currently supported:
///
///<b>bold</b>, <strong>bold</strong>
///<i>italic</i>, <em>italic</em>
///<a href="http://www.example.com/">inline URL</a>
///<a href="tg://user?id=123456789">inline mention of a user</a>
///<code>inline fixed-width code</code>
///<pre>pre-formatted fixed-width code block</pre>
///
///Please note:
///
///    Only the tags mentioned above are currently supported.
///    Tags must not be nested.
///    All <, > and & symbols that are not a part of a tag or an HTML entity must be replaced with the corresponding HTML entities (< with &lt;, > with &gt; and & with &amp;).
///    All numerical HTML entities are supported.
///    The API currently supports only the following named HTML entities: &lt;, &gt;, &amp; and &quot;.
///
///forwardMessage
///
///Use this method to forward messages of any kind. On success, the sent Message is returned.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct forwardMessage {
    ///Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Or<Integer, String>,
    ///Unique identifier for the chat where the original message was sent (or channel username in the format @channelusername)
    pub from_chat_id: Or<Integer, String>,
    ///Sends the message silently. Users will receive a notification with no sound.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    ///Message identifier in the chat specified in from_chat_id
    pub message_id: Integer,
}
impl TelegramMethod for forwardMessage {
    const method_name: &'static str = "forwardMessage";
}
///
///Use this method to send photos. On success, the sent Message is returned.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct sendPhoto {
    ///Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Or<Integer, String>,
    ///Photo to send. Pass a file_id as String to send a photo that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a photo from the Internet, or upload a new photo using multipart/form-data. More info on Sending Files »
    pub photo: Or<InputFile, String>,
    ///Photo caption (may also be used when resending photos by file_id), 0-1024 characters
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    ///Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    ///Sends the message silently. Users will receive a notification with no sound.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    ///If the message is a reply, ID of the original message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<Integer>,
    ///Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<
        Or<InlineKeyboardMarkup, Or<ReplyKeyboardMarkup, Or<ReplyKeyboardRemove, ForceReply>>>,
    >,
}
impl TelegramMethod for sendPhoto {
    const method_name: &'static str = "sendPhoto";
}
///
///Use this method to send audio files, if you want Telegram clients to display them in the music player. Your audio must be in the .MP3 or .M4A format. On success, the sent Message is returned. Bots can currently send audio files of up to 50 MB in size, this limit may be changed in the future.
///
///For sending voice messages, use the sendVoice method instead.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct sendAudio {
    ///Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Or<Integer, String>,
    ///Audio file to send. Pass a file_id as String to send an audio file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get an audio file from the Internet, or upload a new one using multipart/form-data. More info on Sending Files »
    pub audio: Or<InputFile, String>,
    ///Audio caption, 0-1024 characters
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    ///Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    ///Duration of the audio in seconds
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Integer>,
    ///Performer
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    ///Track name
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    ///Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail‘s width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can’t be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More info on Sending Files »
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<Or<InputFile, String>>,
    ///Sends the message silently. Users will receive a notification with no sound.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    ///If the message is a reply, ID of the original message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<Integer>,
    ///Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<
        Or<InlineKeyboardMarkup, Or<ReplyKeyboardMarkup, Or<ReplyKeyboardRemove, ForceReply>>>,
    >,
}
impl TelegramMethod for sendAudio {
    const method_name: &'static str = "sendAudio";
}
///
///Use this method to send general files. On success, the sent Message is returned. Bots can currently send files of any type of up to 50 MB in size, this limit may be changed in the future.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct sendDocument {
    ///Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Or<Integer, String>,
    ///File to send. Pass a file_id as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More info on Sending Files »
    pub document: Or<InputFile, String>,
    ///Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail‘s width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can’t be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More info on Sending Files »
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<Or<InputFile, String>>,
    ///Document caption (may also be used when resending documents by file_id), 0-1024 characters
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    ///Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    ///Sends the message silently. Users will receive a notification with no sound.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    ///If the message is a reply, ID of the original message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<Integer>,
    ///Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<
        Or<InlineKeyboardMarkup, Or<ReplyKeyboardMarkup, Or<ReplyKeyboardRemove, ForceReply>>>,
    >,
}
impl TelegramMethod for sendDocument {
    const method_name: &'static str = "sendDocument";
}
///
///Use this method to send video files, Telegram clients support mp4 videos (other formats may be sent as Document). On success, the sent Message is returned. Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct sendVideo {
    ///Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Or<Integer, String>,
    ///Video to send. Pass a file_id as String to send a video that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a video from the Internet, or upload a new video using multipart/form-data. More info on Sending Files »
    pub video: Or<InputFile, String>,
    ///Duration of sent video in seconds
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Integer>,
    ///Video width
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Integer>,
    ///Video height
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Integer>,
    ///Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail‘s width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can’t be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More info on Sending Files »
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<Or<InputFile, String>>,
    ///Video caption (may also be used when resending videos by file_id), 0-1024 characters
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    ///Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    ///Pass True, if the uploaded video is suitable for streaming
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<Boolean>,
    ///Sends the message silently. Users will receive a notification with no sound.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    ///If the message is a reply, ID of the original message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<Integer>,
    ///Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<
        Or<InlineKeyboardMarkup, Or<ReplyKeyboardMarkup, Or<ReplyKeyboardRemove, ForceReply>>>,
    >,
}
impl TelegramMethod for sendVideo {
    const method_name: &'static str = "sendVideo";
}
///
///Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound). On success, the sent Message is returned. Bots can currently send animation files of up to 50 MB in size, this limit may be changed in the future.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct sendAnimation {
    ///Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Or<Integer, String>,
    ///Animation to send. Pass a file_id as String to send an animation that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get an animation from the Internet, or upload a new animation using multipart/form-data. More info on Sending Files »
    pub animation: Or<InputFile, String>,
    ///Duration of sent animation in seconds
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Integer>,
    ///Animation width
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Integer>,
    ///Animation height
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Integer>,
    ///Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail‘s width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can’t be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More info on Sending Files »
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<Or<InputFile, String>>,
    ///Animation caption (may also be used when resending animation by file_id), 0-1024 characters
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    ///Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    ///Sends the message silently. Users will receive a notification with no sound.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    ///If the message is a reply, ID of the original message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<Integer>,
    ///Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<
        Or<InlineKeyboardMarkup, Or<ReplyKeyboardMarkup, Or<ReplyKeyboardRemove, ForceReply>>>,
    >,
}
impl TelegramMethod for sendAnimation {
    const method_name: &'static str = "sendAnimation";
}
///
///Use this method to send audio files, if you want Telegram clients to display the file as a playable voice message. For this to work, your audio must be in an .ogg file encoded with OPUS (other formats may be sent as Audio or Document). On success, the sent Message is returned. Bots can currently send voice messages of up to 50 MB in size, this limit may be changed in the future.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct sendVoice {
    ///Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Or<Integer, String>,
    ///Audio file to send. Pass a file_id as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More info on Sending Files »
    pub voice: Or<InputFile, String>,
    ///Voice message caption, 0-1024 characters
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    ///Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    ///Duration of the voice message in seconds
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Integer>,
    ///Sends the message silently. Users will receive a notification with no sound.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    ///If the message is a reply, ID of the original message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<Integer>,
    ///Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<
        Or<InlineKeyboardMarkup, Or<ReplyKeyboardMarkup, Or<ReplyKeyboardRemove, ForceReply>>>,
    >,
}
impl TelegramMethod for sendVoice {
    const method_name: &'static str = "sendVoice";
}
///
///As of v.4.0, Telegram clients support rounded square mp4 videos of up to 1 minute long. Use this method to send video messages. On success, the sent Message is returned.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct sendVideoNote {
    ///Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Or<Integer, String>,
    ///Video note to send. Pass a file_id as String to send a video note that exists on the Telegram servers (recommended) or upload a new video using multipart/form-data. More info on Sending Files ». Sending video notes by a URL is currently unsupported
    pub video_note: Or<InputFile, String>,
    ///Duration of sent video in seconds
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Integer>,
    ///Video width and height, i.e. diameter of the video message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<Integer>,
    ///Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail‘s width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can’t be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More info on Sending Files »
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<Or<InputFile, String>>,
    ///Sends the message silently. Users will receive a notification with no sound.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    ///If the message is a reply, ID of the original message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<Integer>,
    ///Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<
        Or<InlineKeyboardMarkup, Or<ReplyKeyboardMarkup, Or<ReplyKeyboardRemove, ForceReply>>>,
    >,
}
impl TelegramMethod for sendVideoNote {
    const method_name: &'static str = "sendVideoNote";
}
///
///Use this method to send a group of photos or videos as an album. On success, an array of the sent Messages is returned.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct sendMediaGroup {
    ///Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Or<Integer, String>,
    //    ///A JSON-serialized array describing photos and videos to be sent, must include 2–10 items
    //    pub media: Vec,
    ///Sends the messages silently. Users will receive a notification with no sound.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    ///If the messages are a reply, ID of the original message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<Integer>,
}
impl TelegramMethod for sendMediaGroup {
    const method_name: &'static str = "sendMediaGroup";
}
///
///Use this method to send point on the map. On success, the sent Message is returned.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct sendLocation {
    ///Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Or<Integer, String>,
    ///Latitude of the location
    pub latitude: Float,
    ///Longitude of the location
    pub longitude: Float,
    ///Period in seconds for which the location will be updated (see Live Locations, should be between 60 and 86400.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<Integer>,
    ///Sends the message silently. Users will receive a notification with no sound.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    ///If the message is a reply, ID of the original message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<Integer>,
    ///Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<
        Or<InlineKeyboardMarkup, Or<ReplyKeyboardMarkup, Or<ReplyKeyboardRemove, ForceReply>>>,
    >,
}
impl TelegramMethod for sendLocation {
    const method_name: &'static str = "sendLocation";
}
///
///Use this method to edit live location messages. A location can be edited until its live_period expires or editing is explicitly disabled by a call to stopMessageLiveLocation. On success, if the edited message was sent by the bot, the edited Message is returned, otherwise True is returned.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct editMessageLiveLocation {
    ///Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<Or<Integer, String>>,
    ///Required if inline_message_id is not specified. Identifier of the message to edit
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Integer>,
    ///Required if chat_id and message_id are not specified. Identifier of the inline message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    ///Latitude of new location
    pub latitude: Float,
    ///Longitude of new location
    pub longitude: Float,
    ///A JSON-serialized object for a new inline keyboard.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
impl TelegramMethod for editMessageLiveLocation {
    const method_name: &'static str = "editMessageLiveLocation";
}
///
///Use this method to stop updating a live location message before live_period expires. On success, if the message was sent by the bot, the sent Message is returned, otherwise True is returned.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct stopMessageLiveLocation {
    ///Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<Or<Integer, String>>,
    ///Required if inline_message_id is not specified. Identifier of the message with live location to stop
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Integer>,
    ///Required if chat_id and message_id are not specified. Identifier of the inline message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    ///A JSON-serialized object for a new inline keyboard.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
impl TelegramMethod for stopMessageLiveLocation {
    const method_name: &'static str = "stopMessageLiveLocation";
}
///
///Use this method to send information about a venue. On success, the sent Message is returned.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct sendVenue {
    ///Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Or<Integer, String>,
    ///Latitude of the venue
    pub latitude: Float,
    ///Longitude of the venue
    pub longitude: Float,
    ///Name of the venue
    pub title: String,
    ///Address of the venue
    pub address: String,
    ///Foursquare identifier of the venue
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    ///Foursquare type of the venue, if known. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    ///Sends the message silently. Users will receive a notification with no sound.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    ///If the message is a reply, ID of the original message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<Integer>,
    ///Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<
        Or<InlineKeyboardMarkup, Or<ReplyKeyboardMarkup, Or<ReplyKeyboardRemove, ForceReply>>>,
    >,
}
impl TelegramMethod for sendVenue {
    const method_name: &'static str = "sendVenue";
}
///
///Use this method to send phone contacts. On success, the sent Message is returned.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct sendContact {
    ///Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Or<Integer, String>,
    ///Contact's phone number
    pub phone_number: String,
    ///Contact's first name
    pub first_name: String,
    ///Contact's last name
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    ///Additional data about the contact in the form of a vCard, 0-2048 bytes
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
    ///Sends the message silently. Users will receive a notification with no sound.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    ///If the message is a reply, ID of the original message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<Integer>,
    ///Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove keyboard or to force a reply from the user.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<
        Or<InlineKeyboardMarkup, Or<ReplyKeyboardMarkup, Or<ReplyKeyboardRemove, ForceReply>>>,
    >,
}
impl TelegramMethod for sendContact {
    const method_name: &'static str = "sendContact";
}
///
///Use this method to send a native poll. A native poll can't be sent to a private chat. On success, the sent Message is returned.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct sendPoll {
    ///Unique identifier for the target chat or username of the target channel (in the format @channelusername). A native poll can't be sent to a private chat.
    pub chat_id: Or<Integer, String>,
    ///Poll question, 1-255 characters
    pub question: String,
    ///List of answer options, 2-10 strings 1-100 characters each
    pub options: Vec<String>,
    ///Sends the message silently. Users will receive a notification with no sound.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
    ///If the message is a reply, ID of the original message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<Integer>,
    ///Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<
        Or<InlineKeyboardMarkup, Or<ReplyKeyboardMarkup, Or<ReplyKeyboardRemove, ForceReply>>>,
    >,
}
impl TelegramMethod for sendPoll {
    const method_name: &'static str = "sendPoll";
}
///
///Use this method when you need to tell the user that something is happening on the bot's side. The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status). Returns True on success.
///
///    Example: The ImageBot needs some time to process a request and upload the image. Instead of sending a text message along the lines of “Retrieving image, please wait…”, the bot may use sendChatAction with action = upload_photo. The user will see a “sending photo” status for the bot.
///
///We only recommend using this method when a response from the bot will take a noticeable amount of time to arrive.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct sendChatAction {
    ///Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Or<Integer, String>,
    ///Type of action to broadcast. Choose one, depending on what the user is about to receive: typing for text messages, upload_photo for photos, record_video or upload_video for videos, record_audio or upload_audio for audio files, upload_document for general files, find_location for location data, record_video_note or upload_video_note for video notes.
    pub action: String,
}
impl TelegramMethod for sendChatAction {
    const method_name: &'static str = "sendChatAction";
}
///
///Use this method to get a list of profile pictures for a user. Returns a UserProfilePhotos object.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct getUserProfilePhotos {
    ///Unique identifier of the target user
    pub user_id: Integer,
    ///Sequential number of the first photo to be returned. By default, all photos are returned.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<Integer>,
    ///Limits the number of photos to be retrieved. Values between 1—100 are accepted. Defaults to 100.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Integer>,
}
impl TelegramMethod for getUserProfilePhotos {
    const method_name: &'static str = "getUserProfilePhotos";
}
///
///Use this method to get basic info about a file and prepare it for downloading. For the moment, bots can download files of up to 20MB in size. On success, a File object is returned. The file can then be downloaded via the link https://api.telegram.org/file/bot<token>/<file_path>, where <file_path> is taken from the response. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling getFile again.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct getFile {
    ///
    ///Note: This function may not preserve the original file name and MIME type. You should save the file's MIME type and name (if available) when the File object is received.
    ///File identifier to get info about
    pub file_id: String,
}
impl TelegramMethod for getFile {
    const method_name: &'static str = "getFile";
}
///
///Use this method to kick a user from a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the group on their own using invite links, etc., unless unbanned first. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns True on success.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct kickChatMember {
    ///Unique identifier for the target group or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: Or<Integer, String>,
    ///Unique identifier of the target user
    pub user_id: Integer,
    ///Date when the user will be unbanned, unix time. If user is banned for more than 366 days or less than 30 seconds from the current time they are considered to be banned forever
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<Integer>,
}
impl TelegramMethod for kickChatMember {
    const method_name: &'static str = "kickChatMember";
}
///
///Use this method to unban a previously kicked user in a supergroup or channel. The user will not return to the group or channel automatically, but will be able to join via link, etc. The bot must be an administrator for this to work. Returns True on success.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct unbanChatMember {
    ///Unique identifier for the target group or username of the target supergroup or channel (in the format @username)
    pub chat_id: Or<Integer, String>,
    ///Unique identifier of the target user
    pub user_id: Integer,
}
impl TelegramMethod for unbanChatMember {
    const method_name: &'static str = "unbanChatMember";
}
///
///Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate admin rights. Pass True for all permissions to lift restrictions from a user. Returns True on success.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct restrictChatMember {
    ///Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: Or<Integer, String>,
    ///Unique identifier of the target user
    pub user_id: Integer,
    ///New user permissions
    pub permissions: ChatPermissions,
    ///Date when restrictions will be lifted for the user, unix time. If user is restricted for more than 366 days or less than 30 seconds from the current time, they are considered to be restricted forever
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<Integer>,
}
impl TelegramMethod for restrictChatMember {
    const method_name: &'static str = "restrictChatMember";
}
///
///Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Pass False for all boolean parameters to demote a user. Returns True on success.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct promoteChatMember {
    ///Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Or<Integer, String>,
    ///Unique identifier of the target user
    pub user_id: Integer,
    ///Pass True, if the administrator can change chat title, photo and other settings
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<Boolean>,
    ///Pass True, if the administrator can create channel posts, channels only
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<Boolean>,
    ///Pass True, if the administrator can edit messages of other users and can pin messages, channels only
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<Boolean>,
    ///Pass True, if the administrator can delete messages of other users
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_messages: Option<Boolean>,
    ///Pass True, if the administrator can invite new users to the chat
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<Boolean>,
    ///Pass True, if the administrator can restrict, ban or unban chat members
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_restrict_members: Option<Boolean>,
    ///Pass True, if the administrator can pin messages, supergroups only
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<Boolean>,
    ///Pass True, if the administrator can add new administrators with a subset of his own privileges or demote administrators that he has promoted, directly or indirectly (promoted by administrators that were appointed by him)
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_promote_members: Option<Boolean>,
}
impl TelegramMethod for promoteChatMember {
    const method_name: &'static str = "promoteChatMember";
}
///
///Use this method to set default chat permissions for all members. The bot must be an administrator in the group or a supergroup for this to work and must have the can_restrict_members admin rights. Returns True on success.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct setChatPermissions {
    ///Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: Or<Integer, String>,
    ///New default chat permissions
    pub permissions: ChatPermissions,
}
impl TelegramMethod for setChatPermissions {
    const method_name: &'static str = "setChatPermissions";
}
///
///Use this method to generate a new invite link for a chat; any previously generated link is revoked. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns the new invite link as String on success.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct exportChatInviteLink {
    ///
    ///    Note: Each administrator in a chat generates their own invite links. Bots can't use invite links generated by other administrators. If you want your bot to work with invite links, it will need to generate its own link using exportChatInviteLink – after this the link will become available to the bot via the getChat method. If your bot needs to generate a new invite link replacing its previous one, use exportChatInviteLink again.
    ///
    ///Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Or<Integer, String>,
}
impl TelegramMethod for exportChatInviteLink {
    const method_name: &'static str = "exportChatInviteLink";
}
///
///Use this method to set a new profile photo for the chat. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns True on success.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct setChatPhoto {
    ///Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Or<Integer, String>,
    ///New chat photo, uploaded using multipart/form-data
    pub photo: InputFile,
}
impl TelegramMethod for setChatPhoto {
    const method_name: &'static str = "setChatPhoto";
}
///
///Use this method to delete a chat photo. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns True on success.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct deleteChatPhoto {
    ///Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Or<Integer, String>,
}
impl TelegramMethod for deleteChatPhoto {
    const method_name: &'static str = "deleteChatPhoto";
}
///
///Use this method to change the title of a chat. Titles can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns True on success.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct setChatTitle {
    ///Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Or<Integer, String>,
    ///New chat title, 1-255 characters
    pub title: String,
}
impl TelegramMethod for setChatTitle {
    const method_name: &'static str = "setChatTitle";
}
///
///Use this method to change the description of a group, a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns True on success.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct setChatDescription {
    ///Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Or<Integer, String>,
    ///New chat description, 0-255 characters
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl TelegramMethod for setChatDescription {
    const method_name: &'static str = "setChatDescription";
}
///
///Use this method to pin a message in a group, a supergroup, or a channel. The bot must be an administrator in the chat for this to work and must have the ‘can_pin_messages’ admin right in the supergroup or ‘can_edit_messages’ admin right in the channel. Returns True on success.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct pinChatMessage {
    ///Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Or<Integer, String>,
    ///Identifier of a message to pin
    pub message_id: Integer,
    ///Pass True, if it is not necessary to send a notification to all chat members about the new pinned message. Notifications are always disabled in channels.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<Boolean>,
}
impl TelegramMethod for pinChatMessage {
    const method_name: &'static str = "pinChatMessage";
}
///
///Use this method to unpin a message in a group, a supergroup, or a channel. The bot must be an administrator in the chat for this to work and must have the ‘can_pin_messages’ admin right in the supergroup or ‘can_edit_messages’ admin right in the channel. Returns True on success.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct unpinChatMessage {
    ///Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Or<Integer, String>,
}
impl TelegramMethod for unpinChatMessage {
    const method_name: &'static str = "unpinChatMessage";
}
///
///Use this method for your bot to leave a group, supergroup or channel. Returns True on success.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct leaveChat {
    ///Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: Or<Integer, String>,
}
impl TelegramMethod for leaveChat {
    const method_name: &'static str = "leaveChat";
}
///
///Use this method to get up to date information about the chat (current name of the user for one-on-one conversations, current username of a user, group or channel, etc.). Returns a Chat object on success.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct getChat {
    ///Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: Or<Integer, String>,
}
impl TelegramMethod for getChat {
    const method_name: &'static str = "getChat";
}
///
///Use this method to get a list of administrators in a chat. On success, returns an Array of ChatMember objects that contains information about all chat administrators except other bots. If the chat is a group or a supergroup and no administrators were appointed, only the creator will be returned.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct getChatAdministrators {
    ///Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: Or<Integer, String>,
}
impl TelegramMethod for getChatAdministrators {
    const method_name: &'static str = "getChatAdministrators";
}
///
///Use this method to get the number of members in a chat. Returns Int on success.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct getChatMembersCount {
    ///Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: Or<Integer, String>,
}
impl TelegramMethod for getChatMembersCount {
    const method_name: &'static str = "getChatMembersCount";
}
///
///Use this method to get information about a member of a chat. Returns a ChatMember object on success.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct getChatMember {
    ///Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: Or<Integer, String>,
    ///Unique identifier of the target user
    pub user_id: Integer,
}
impl TelegramMethod for getChatMember {
    const method_name: &'static str = "getChatMember";
}
///
///Use this method to set a new group sticker set for a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Use the field can_set_sticker_set optionally returned in getChat requests to check if the bot can use this method. Returns True on success.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct setChatStickerSet {
    ///Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: Or<Integer, String>,
    ///Name of the sticker set to be set as the group sticker set
    pub sticker_set_name: String,
}
impl TelegramMethod for setChatStickerSet {
    const method_name: &'static str = "setChatStickerSet";
}
///
///Use this method to delete a group sticker set from a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Use the field can_set_sticker_set optionally returned in getChat requests to check if the bot can use this method. Returns True on success.
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct deleteChatStickerSet {
    ///Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: Or<Integer, String>,
}
impl TelegramMethod for deleteChatStickerSet {
    const method_name: &'static str = "deleteChatStickerSet";
}
///
///Use this method to send answers to callback queries sent from inline keyboards. The answer will be displayed to the user as a notification at the top of the chat screen or as an alert. On success, True is returned.
///
///    Alternatively, the user can be redirected to the specified Game URL. For this option to work, you must first create a game for your bot via @Botfather and accept the terms. Otherwise, you may use links like t.me/your_bot?start=XXXX that open your bot with a parameter.
///
#[derive(Serialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct answerCallbackQuery {
    ///Unique identifier for the query to be answered
    pub callback_query_id: String,
    ///Text of the notification. If not specified, nothing will be shown to the user, 0-200 characters
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    ///If true, an alert will be shown by the client instead of a notification at the top of the chat screen. Defaults to false.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_alert: Option<Boolean>,
    ///URL that will be opened by the user's client. If you have created a Game and accepted the conditions via @Botfather, specify the URL that opens your game – note that this will only work if the query comes from a callback_game button.
    #[builder(default)]
    ///
    ///Otherwise, you may use links like t.me/your_bot?start=XXXX that open your bot with a parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    ///The maximum amount of time in seconds that the result of the callback query may be cached client-side. Telegram apps will support caching starting in version 3.14. Defaults to 0.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<Integer>,
}
impl TelegramMethod for answerCallbackQuery {
    const method_name: &'static str = "answerCallbackQuery";
}
