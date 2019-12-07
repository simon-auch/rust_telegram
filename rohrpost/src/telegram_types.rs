use crate::helpers::Or;
use serde::{Deserialize, Serialize};
type Integer = i64;
type Boolean = bool;
type True = bool;
type Float = f64;
//unsupported types
type Game = ();
type CallbackGame = ();
type InputFile = ();
///
///Contains information about the current status of a webhook.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct WebhookInfo {
    ///Webhook URL, may be empty if webhook is not set up
    pub url: String,
    ///True, if a custom certificate was provided for webhook certificate checks
    pub has_custom_certificate: Boolean,
    ///Number of updates awaiting delivery
    pub pending_update_count: Integer,
    ///Optional. Unix time for the most recent error that happened when trying to deliver an update via webhook
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_date: Option<Integer>,
    ///Optional. Error message in human-readable format for the most recent error that happened when trying to deliver an update via webhook
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
    ///Optional. Maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<Integer>,
    ///Optional. A list of update types the bot is subscribed to. Defaults to all update types
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}
///
///This object represents an incoming inline query. When the user sends an empty query, your bot could return some default or trending results.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InlineQuery {
    ///Unique identifier for this query
    pub id: String,
    ///Sender
    pub from: User,
    ///Optional. Sender location, only for bots that request user location
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    ///Text of the query (up to 512 characters)
    pub query: String,
    ///Offset of the results to be returned, can be controlled by the bot
    pub offset: String,
}
///
///Represents a link to an article or web page.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InlineQueryResultArticle {
    ///Type of the result, must be article
    #[serde(rename = "type")]
    pub type_: String,
    ///Unique identifier for this result, 1-64 Bytes
    pub id: String,
    ///Title of the result
    pub title: String,
    ///Content of the message to be sent
    pub input_message_content: InputMessageContent,
    ///Optional. Inline keyboard attached to the message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    ///Optional. URL of the result
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    ///Optional. Pass True, if you don't want the URL to be shown in the message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_url: Option<Boolean>,
    ///Optional. Short description of the result
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Optional. Url of the thumbnail for the result
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    ///Optional. Thumbnail width
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<Integer>,
    ///Optional. Thumbnail height
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<Integer>,
}
///
///Represents a link to a photo. By default, this photo will be sent by the user with optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the photo.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InlineQueryResultPhoto {
    ///Type of the result, must be photo
    #[serde(rename = "type")]
    pub type_: String,
    ///Unique identifier for this result, 1-64 bytes
    pub id: String,
    ///A valid URL of the photo. Photo must be in jpeg format. Photo size must not exceed 5MB
    pub photo_url: String,
    ///URL of the thumbnail for the photo
    pub thumb_url: String,
    ///Optional. Width of the photo
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<Integer>,
    ///Optional. Height of the photo
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<Integer>,
    ///Optional. Title for the result
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    ///Optional. Short description of the result
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Optional. Caption of the photo to be sent, 0-1024 characters
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    ///Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    ///Optional. Inline keyboard attached to the message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    ///Optional. Content of the message to be sent instead of the photo
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
///
///Represents a link to an animated GIF file. By default, this animated GIF file will be sent by the user with optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the animation.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InlineQueryResultGif {
    ///Type of the result, must be gif
    #[serde(rename = "type")]
    pub type_: String,
    ///Unique identifier for this result, 1-64 bytes
    pub id: String,
    ///A valid URL for the GIF file. File size must not exceed 1MB
    pub gif_url: String,
    ///Optional. Width of the GIF
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_width: Option<Integer>,
    ///Optional. Height of the GIF
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_height: Option<Integer>,
    ///Optional. Duration of the GIF
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_duration: Option<Integer>,
    ///URL of the static thumbnail for the result (jpeg or gif)
    pub thumb_url: String,
    ///Optional. Title for the result
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    ///Optional. Caption of the GIF file to be sent, 0-1024 characters
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    ///Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    ///Optional. Inline keyboard attached to the message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    ///Optional. Content of the message to be sent instead of the GIF animation
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
///
///Represents a link to a video animation (H.264/MPEG-4 AVC video without sound). By default, this animated MPEG-4 file will be sent by the user with optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the animation.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InlineQueryResultMpeg4Gif {
    ///Type of the result, must be mpeg4_gif
    #[serde(rename = "type")]
    pub type_: String,
    ///Unique identifier for this result, 1-64 bytes
    pub id: String,
    ///A valid URL for the MP4 file. File size must not exceed 1MB
    pub mpeg4_url: String,
    ///Optional. Video width
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_width: Option<Integer>,
    ///Optional. Video height
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_height: Option<Integer>,
    ///Optional. Video duration
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_duration: Option<Integer>,
    ///URL of the static thumbnail (jpeg or gif) for the result
    pub thumb_url: String,
    ///Optional. Title for the result
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    ///Optional. Caption of the MPEG-4 file to be sent, 0-1024 characters
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    ///Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    ///Optional. Inline keyboard attached to the message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    ///Optional. Content of the message to be sent instead of the video animation
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
///
///Represents a link to a page containing an embedded video player or a video file. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the video.
///
///    If an InlineQueryResultVideo message contains an embedded video (e.g., YouTube), you must replace its content using input_message_content.
///
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InlineQueryResultVideo {
    ///Type of the result, must be video
    #[serde(rename = "type")]
    pub type_: String,
    ///Unique identifier for this result, 1-64 bytes
    pub id: String,
    ///A valid URL for the embedded video player or video file
    pub video_url: String,
    ///Mime type of the content of video url, “text/html” or “video/mp4”
    pub mime_type: String,
    ///URL of the thumbnail (jpeg only) for the video
    pub thumb_url: String,
    ///Title for the result
    pub title: String,
    ///Optional. Caption of the video to be sent, 0-1024 characters
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    ///Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    ///Optional. Video width
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_width: Option<Integer>,
    ///Optional. Video height
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_height: Option<Integer>,
    ///Optional. Video duration in seconds
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_duration: Option<Integer>,
    ///Optional. Short description of the result
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Optional. Inline keyboard attached to the message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    ///Optional. Content of the message to be sent instead of the video. This field is required if InlineQueryResultVideo is used to send an HTML-page as a result (e.g., a YouTube video).
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
///
///Represents a link to an MP3 audio file. By default, this audio file will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the audio.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InlineQueryResultAudio {
    ///Type of the result, must be audio
    #[serde(rename = "type")]
    pub type_: String,
    ///Unique identifier for this result, 1-64 bytes
    pub id: String,
    ///A valid URL for the audio file
    pub audio_url: String,
    ///Title
    pub title: String,
    ///Optional. Caption, 0-1024 characters
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    ///Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    ///Optional. Performer
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    ///Optional. Audio duration in seconds
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_duration: Option<Integer>,
    ///Optional. Inline keyboard attached to the message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    ///Optional. Content of the message to be sent instead of the audio
    #[builder(default)]
    ///
    ///Note: This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
///
///Represents a link to a voice recording in an .ogg container encoded with OPUS. By default, this voice recording will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the the voice message.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InlineQueryResultVoice {
    ///Type of the result, must be voice
    #[serde(rename = "type")]
    pub type_: String,
    ///Unique identifier for this result, 1-64 bytes
    pub id: String,
    ///A valid URL for the voice recording
    pub voice_url: String,
    ///Recording title
    pub title: String,
    ///Optional. Caption, 0-1024 characters
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    ///Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    ///Optional. Recording duration in seconds
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_duration: Option<Integer>,
    ///Optional. Inline keyboard attached to the message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    ///Optional. Content of the message to be sent instead of the voice recording
    #[builder(default)]
    ///
    ///Note: This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
///
///Represents a link to a file. By default, this file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the file. Currently, only .PDF and .ZIP files can be sent using this method.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InlineQueryResultDocument {
    ///Type of the result, must be document
    #[serde(rename = "type")]
    pub type_: String,
    ///Unique identifier for this result, 1-64 bytes
    pub id: String,
    ///Title for the result
    pub title: String,
    ///Optional. Caption of the document to be sent, 0-1024 characters
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    ///Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    ///A valid URL for the file
    pub document_url: String,
    ///Mime type of the content of the file, either “application/pdf” or “application/zip”
    pub mime_type: String,
    ///Optional. Short description of the result
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Optional. Inline keyboard attached to the message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    ///Optional. Content of the message to be sent instead of the file
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    ///Optional. URL of the thumbnail (jpeg only) for the file
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    ///Optional. Thumbnail width
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<Integer>,
    ///Optional. Thumbnail height
    #[builder(default)]
    ///
    ///Note: This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<Integer>,
}
///
///Represents a location on a map. By default, the location will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the location.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InlineQueryResultLocation {
    ///Type of the result, must be location
    #[serde(rename = "type")]
    pub type_: String,
    ///Unique identifier for this result, 1-64 Bytes
    pub id: String,
    ///Location latitude in degrees
    pub latitude: Float,
    ///Location longitude in degrees
    pub longitude: Float,
    ///Location title
    pub title: String,
    ///Optional. Period in seconds for which the location can be updated, should be between 60 and 86400.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<Integer>,
    ///Optional. Inline keyboard attached to the message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    ///Optional. Content of the message to be sent instead of the location
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    ///Optional. Url of the thumbnail for the result
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    ///Optional. Thumbnail width
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<Integer>,
    ///Optional. Thumbnail height
    #[builder(default)]
    ///
    ///Note: This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<Integer>,
}
///
///Represents a venue. By default, the venue will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the venue.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InlineQueryResultVenue {
    ///Type of the result, must be venue
    #[serde(rename = "type")]
    pub type_: String,
    ///Unique identifier for this result, 1-64 Bytes
    pub id: String,
    ///Latitude of the venue location in degrees
    pub latitude: Float,
    ///Longitude of the venue location in degrees
    pub longitude: Float,
    ///Title of the venue
    pub title: String,
    ///Address of the venue
    pub address: String,
    ///Optional. Foursquare identifier of the venue if known
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    ///Optional. Foursquare type of the venue, if known. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    ///Optional. Inline keyboard attached to the message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    ///Optional. Content of the message to be sent instead of the venue
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    ///Optional. Url of the thumbnail for the result
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    ///Optional. Thumbnail width
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<Integer>,
    ///Optional. Thumbnail height
    #[builder(default)]
    ///
    ///Note: This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<Integer>,
}
///
///Represents a contact with a phone number. By default, this contact will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the contact.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InlineQueryResultContact {
    ///Type of the result, must be contact
    #[serde(rename = "type")]
    pub type_: String,
    ///Unique identifier for this result, 1-64 Bytes
    pub id: String,
    ///Contact's phone number
    pub phone_number: String,
    ///Contact's first name
    pub first_name: String,
    ///Optional. Contact's last name
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    ///Optional. Additional data about the contact in the form of a vCard, 0-2048 bytes
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
    ///Optional. Inline keyboard attached to the message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    ///Optional. Content of the message to be sent instead of the contact
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    ///Optional. Url of the thumbnail for the result
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    ///Optional. Thumbnail width
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<Integer>,
    ///Optional. Thumbnail height
    #[builder(default)]
    ///
    ///Note: This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<Integer>,
}
///
///Represents a Game.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InlineQueryResultGame {
    ///Type of the result, must be game
    #[serde(rename = "type")]
    pub type_: String,
    ///Unique identifier for this result, 1-64 bytes
    pub id: String,
    ///Short name of the game
    pub game_short_name: String,
    ///Optional. Inline keyboard attached to the message
    #[builder(default)]
    ///
    ///Note: This will only work in Telegram versions released after October 1, 2016. Older clients will not display any inline results if a game result is among them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
///
///Represents a link to a photo stored on the Telegram servers. By default, this photo will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the photo.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InlineQueryResultCachedPhoto {
    ///Type of the result, must be photo
    #[serde(rename = "type")]
    pub type_: String,
    ///Unique identifier for this result, 1-64 bytes
    pub id: String,
    ///A valid file identifier of the photo
    pub photo_file_id: String,
    ///Optional. Title for the result
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    ///Optional. Short description of the result
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Optional. Caption of the photo to be sent, 0-1024 characters
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    ///Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    ///Optional. Inline keyboard attached to the message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    ///Optional. Content of the message to be sent instead of the photo
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
///
///Represents a link to an animated GIF file stored on the Telegram servers. By default, this animated GIF file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with specified content instead of the animation.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InlineQueryResultCachedGif {
    ///Type of the result, must be gif
    #[serde(rename = "type")]
    pub type_: String,
    ///Unique identifier for this result, 1-64 bytes
    pub id: String,
    ///A valid file identifier for the GIF file
    pub gif_file_id: String,
    ///Optional. Title for the result
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    ///Optional. Caption of the GIF file to be sent, 0-1024 characters
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    ///Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    ///Optional. Inline keyboard attached to the message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    ///Optional. Content of the message to be sent instead of the GIF animation
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
///
///Represents a link to a video animation (H.264/MPEG-4 AVC video without sound) stored on the Telegram servers. By default, this animated MPEG-4 file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the animation.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InlineQueryResultCachedMpeg4Gif {
    ///Type of the result, must be mpeg4_gif
    #[serde(rename = "type")]
    pub type_: String,
    ///Unique identifier for this result, 1-64 bytes
    pub id: String,
    ///A valid file identifier for the MP4 file
    pub mpeg4_file_id: String,
    ///Optional. Title for the result
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    ///Optional. Caption of the MPEG-4 file to be sent, 0-1024 characters
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    ///Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    ///Optional. Inline keyboard attached to the message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    ///Optional. Content of the message to be sent instead of the video animation
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
///
///Represents a link to a sticker stored on the Telegram servers. By default, this sticker will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the sticker.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InlineQueryResultCachedSticker {
    ///Type of the result, must be sticker
    #[serde(rename = "type")]
    pub type_: String,
    ///Unique identifier for this result, 1-64 bytes
    pub id: String,
    ///A valid file identifier of the sticker
    pub sticker_file_id: String,
    ///Optional. Inline keyboard attached to the message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    ///Optional. Content of the message to be sent instead of the sticker
    #[builder(default)]
    ///
    ///Note: This will only work in Telegram versions released after 9 April, 2016 for static stickers and after 06 July, 2019 for animated stickers. Older clients will ignore them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
///
///Represents a link to a file stored on the Telegram servers. By default, this file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the file.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InlineQueryResultCachedDocument {
    ///Type of the result, must be document
    #[serde(rename = "type")]
    pub type_: String,
    ///Unique identifier for this result, 1-64 bytes
    pub id: String,
    ///Title for the result
    pub title: String,
    ///A valid file identifier for the file
    pub document_file_id: String,
    ///Optional. Short description of the result
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Optional. Caption of the document to be sent, 0-1024 characters
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    ///Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    ///Optional. Inline keyboard attached to the message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    ///Optional. Content of the message to be sent instead of the file
    #[builder(default)]
    ///
    ///Note: This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
///
///Represents a link to a video file stored on the Telegram servers. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the video.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InlineQueryResultCachedVideo {
    ///Type of the result, must be video
    #[serde(rename = "type")]
    pub type_: String,
    ///Unique identifier for this result, 1-64 bytes
    pub id: String,
    ///A valid file identifier for the video file
    pub video_file_id: String,
    ///Title for the result
    pub title: String,
    ///Optional. Short description of the result
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Optional. Caption of the video to be sent, 0-1024 characters
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    ///Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    ///Optional. Inline keyboard attached to the message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    ///Optional. Content of the message to be sent instead of the video
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
///
///Represents a link to a voice message stored on the Telegram servers. By default, this voice message will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the voice message.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InlineQueryResultCachedVoice {
    ///Type of the result, must be voice
    #[serde(rename = "type")]
    pub type_: String,
    ///Unique identifier for this result, 1-64 bytes
    pub id: String,
    ///A valid file identifier for the voice message
    pub voice_file_id: String,
    ///Voice message title
    pub title: String,
    ///Optional. Caption, 0-1024 characters
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    ///Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    ///Optional. Inline keyboard attached to the message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    ///Optional. Content of the message to be sent instead of the voice message
    #[builder(default)]
    ///
    ///Note: This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
///
///Represents a link to an MP3 audio file stored on the Telegram servers. By default, this audio file will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the audio.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InlineQueryResultCachedAudio {
    ///Type of the result, must be audio
    #[serde(rename = "type")]
    pub type_: String,
    ///Unique identifier for this result, 1-64 bytes
    pub id: String,
    ///A valid file identifier for the audio file
    pub audio_file_id: String,
    ///Optional. Caption, 0-1024 characters
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    ///Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    ///Optional. Inline keyboard attached to the message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    ///Optional. Content of the message to be sent instead of the audio
    #[builder(default)]
    ///
    ///Note: This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
///
///This object represents the content of a message to be sent as a result of an inline query. Telegram clients currently support the following 4 types:
///
///    InputTextMessageContent
///    InputLocationMessageContent
///    InputVenueMessageContent
///    InputContactMessageContent
///
///InputTextMessageContent
///
///Represents the content of a text message to be sent as the result of an inline query.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InputMessageContent {
    ///Text of the message to be sent, 1-4096 characters
    pub message_text: String,
    ///Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in your bot's message.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    ///Optional. Disables link previews for links in the sent message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<Boolean>,
}
///
///Represents the content of a location message to be sent as the result of an inline query.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InputLocationMessageContent {
    ///Latitude of the location in degrees
    pub latitude: Float,
    ///Longitude of the location in degrees
    pub longitude: Float,
    ///Optional. Period in seconds for which the location can be updated, should be between 60 and 86400.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<Integer>,
}
///
///Represents the content of a venue message to be sent as the result of an inline query.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InputVenueMessageContent {
    ///Latitude of the venue in degrees
    pub latitude: Float,
    ///Longitude of the venue in degrees
    pub longitude: Float,
    ///Name of the venue
    pub title: String,
    ///Address of the venue
    pub address: String,
    ///Optional. Foursquare identifier of the venue, if known
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    ///Optional. Foursquare type of the venue, if known. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
}
///
///Represents the content of a contact message to be sent as the result of an inline query.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InputContactMessageContent {
    ///Contact's phone number
    pub phone_number: String,
    ///Contact's first name
    pub first_name: String,
    ///Optional. Contact's last name
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    ///Optional. Additional data about the contact in the form of a vCard, 0-2048 bytes
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
}
///
///Represents a result of an inline query that was chosen by the user and sent to their chat partner.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct ChosenInlineResult {
    ///The unique identifier for the result that was chosen
    pub result_id: String,
    ///The user that chose the result
    pub from: User,
    ///Optional. Sender location, only for bots that require user location
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    ///Optional. Identifier of the sent inline message. Available only if there is an inline keyboard attached to the message. Will be also received in callback queries and can be used to edit the message.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    ///
    ///Note: It is necessary to enable inline feedback via @Botfather in order to receive these objects in updates.
    ///The query that was used to obtain the result
    pub query: String,
}
///
///This object represents an incoming update.
///At most one of the optional parameters can be present in any given update.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Update {
    ///The update‘s unique identifier. Update identifiers start from a certain positive number and increase sequentially. This ID becomes especially handy if you’re using Webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: Integer,
    ///Optional. New incoming message of any kind — text, photo, sticker, etc.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
    ///Optional. New version of a message that is known to the bot and was edited
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_message: Option<Message>,
    ///Optional. New incoming channel post of any kind — text, photo, sticker, etc.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_post: Option<Message>,
    ///Optional. New version of a channel post that is known to the bot and was edited
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_channel_post: Option<Message>,
    ///Optional. New incoming inline query
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_query: Option<InlineQuery>,
    ///Optional. The result of an inline query that was chosen by a user and sent to their chat partner. Please see our documentation on the feedback collecting for details on how to enable these updates for your bot.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chosen_inline_result: Option<ChosenInlineResult>,
    ///Optional. New incoming callback query
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_query: Option<CallbackQuery>,
    ///Optional. New incoming shipping query. Only for invoices with flexible price
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_query: Option<ShippingQuery>,
    ///Optional. New incoming pre-checkout query. Contains full information about checkout
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_checkout_query: Option<PreCheckoutQuery>,
    ///Optional. New poll state. Bots receive only updates about stopped polls and polls, which are sent by the bot
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,
}
///
///This object represents a Telegram user or bot.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct User {
    ///Unique identifier for this user or bot
    pub id: Integer,
    ///True, if this user is a bot
    pub is_bot: Boolean,
    ///User‘s or bot’s first name
    pub first_name: String,
    ///Optional. User‘s or bot’s last name
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    ///Optional. User‘s or bot’s username
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    ///Optional. IETF language tag of the user's language
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}
///
///This object represents a chat.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Chat {
    ///Unique identifier for this chat. This number may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    pub id: Integer,
    ///Type of chat, can be either “private”, “group”, “supergroup” or “channel”
    #[serde(rename = "type")]
    pub type_: String,
    ///Optional. Title, for supergroups, channels and group chats
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    ///Optional. Username, for private chats, supergroups and channels if available
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    ///Optional. First name of the other party in a private chat
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    ///Optional. Last name of the other party in a private chat
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    ///Optional. Chat photo. Returned only in getChat.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<ChatPhoto>,
    ///Optional. Description, for groups, supergroups and channel chats. Returned only in getChat.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Optional. Chat invite link, for groups, supergroups and channel chats. Each administrator in a chat generates their own invite links, so the bot must first generate the link using exportChatInviteLink. Returned only in getChat.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<String>,
    ///Optional. Pinned message, for groups, supergroups and channels. Returned only in getChat.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<Message>>,
    ///Optional. Default chat member permissions, for groups and supergroups. Returned only in getChat.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ChatPermissions>,
    ///Optional. For supergroups, name of group sticker set. Returned only in getChat.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_set_name: Option<String>,
    ///Optional. True, if the bot can change the group sticker set. Returned only in getChat.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_set_sticker_set: Option<Boolean>,
}
///
///This object represents a message.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Message {
    ///Unique message identifier inside this chat
    pub message_id: Integer,
    ///Optional. Sender, empty for messages sent to channels
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<User>,
    ///Date the message was sent in Unix time
    pub date: Integer,
    ///Conversation the message belongs to
    pub chat: Chat,
    ///Optional. For forwarded messages, sender of the original message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_from: Option<User>,
    ///Optional. For messages forwarded from channels, information about the original channel
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_from_chat: Option<Chat>,
    ///Optional. For messages forwarded from channels, identifier of the original message in the channel
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_from_message_id: Option<Integer>,
    ///Optional. For messages forwarded from channels, signature of the post author if present
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_signature: Option<String>,
    ///Optional. Sender's name for messages forwarded from users who disallow adding a link to their account in forwarded messages
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_sender_name: Option<String>,
    ///Optional. For forwarded messages, date the original message was sent in Unix time
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_date: Option<Integer>,
    ///Optional. For replies, the original message. Note that the Message object in this field will not contain further reply_to_message fields even if it itself is a reply.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message: Option<Box<Message>>,
    ///Optional. Date the message was last edited in Unix time
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_date: Option<Integer>,
    ///Optional. The unique identifier of a media message group this message belongs to
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_group_id: Option<String>,
    ///Optional. Signature of the post author for messages in channels
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,
    ///Optional. For text messages, the actual UTF-8 text of the message, 0-4096 characters.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    ///Optional. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    ///Optional. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    ///Optional. Message is an audio file, information about the file
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,
    ///Optional. Message is a general file, information about the file
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<Document>,
    ///Optional. Message is an animation, information about the animation. For backward compatibility, when this field is set, the document field will also be set
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
    ///Optional. Message is a game, information about the game. More about games »
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<Game>,
    ///Optional. Message is a photo, available sizes of the photo
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
    ///Optional. Message is a sticker, information about the sticker
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,
    ///Optional. Message is a video, information about the video
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,
    ///Optional. Message is a voice message, information about the file
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Voice>,
    ///Optional. Message is a video note, information about the video message
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_note: Option<VideoNote>,
    ///Optional. Caption for the animation, audio, document, photo, video or voice, 0-1024 characters
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    ///Optional. Message is a shared contact, information about the contact
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    ///Optional. Message is a shared location, information about the location
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    ///Optional. Message is a venue, information about the venue
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue: Option<Venue>,
    ///Optional. Message is a native poll, information about the poll
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,
    ///Optional. New members that were added to the group or supergroup and information about them (the bot itself may be one of these members)
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_members: Option<Vec<User>>,
    ///Optional. A member was removed from the group, information about them (this member may be the bot itself)
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_chat_member: Option<User>,
    ///Optional. A chat title was changed to this value
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_title: Option<String>,
    ///Optional. A chat photo was change to this value
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    ///Optional. Service message: the chat photo was deleted
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_chat_photo: Option<True>,
    ///Optional. Service message: the group has been created
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_chat_created: Option<True>,
    ///Optional. Service message: the supergroup has been created. This field can‘t be received in a message coming through updates, because bot can’t be a member of a supergroup when it is created. It can only be found in reply_to_message if someone replies to a very first message in a directly created supergroup.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supergroup_chat_created: Option<True>,
    ///Optional. Service message: the channel has been created. This field can‘t be received in a message coming through updates, because bot can’t be a member of a channel when it is created. It can only be found in reply_to_message if someone replies to a very first message in a channel.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_chat_created: Option<True>,
    ///Optional. The group has been migrated to a supergroup with the specified identifier. This number may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<Integer>,
    ///Optional. The supergroup has been migrated from a group with the specified identifier. This number may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_from_chat_id: Option<Integer>,
    ///Optional. Specified message was pinned. Note that the Message object in this field will not contain further reply_to_message fields even if it is itself a reply.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<Message>>,
    ///Optional. Message is an invoice for a payment, information about the invoice. More about payments »
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Invoice>,
    ///Optional. Message is a service message about a successful payment, information about the payment. More about payments »
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_payment: Option<SuccessfulPayment>,
    ///Optional. The domain name of the website on which the user has logged in. More about Telegram Login »
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_website: Option<String>,
    ///Optional. Telegram Passport data
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport_data: Option<PassportData>,
    ///Optional. Inline keyboard attached to the message. login_url buttons are represented as ordinary url buttons.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
///
///This object represents one special entity in a text message. For example, hashtags, usernames, URLs, etc.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct MessageEntity {
    ///Type of the entity. Can be mention (@username), hashtag, cashtag, bot_command, url, email, phone_number, bold (bold text), italic (italic text), code (monowidth string), pre (monowidth block), text_link (for clickable text URLs), text_mention (for users without usernames)
    #[serde(rename = "type")]
    pub type_: String,
    ///Offset in UTF-16 code units to the start of the entity
    pub offset: Integer,
    ///Length of the entity in UTF-16 code units
    pub length: Integer,
    ///Optional. For “text_link” only, url that will be opened after user taps on the text
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    ///Optional. For “text_mention” only, the mentioned user
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}
///
///This object represents one size of a photo or a file / sticker thumbnail.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct PhotoSize {
    ///Identifier for this file
    pub file_id: String,
    ///Photo width
    pub width: Integer,
    ///Photo height
    pub height: Integer,
    ///Optional. File size
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Integer>,
}
///
///This object represents an audio file to be treated as music by the Telegram clients.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Audio {
    ///Identifier for this file
    pub file_id: String,
    ///Duration of the audio in seconds as defined by sender
    pub duration: Integer,
    ///Optional. Performer of the audio as defined by sender or by audio tags
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    ///Optional. Title of the audio as defined by sender or by audio tags
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    ///Optional. MIME type of the file as defined by sender
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    ///Optional. File size
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Integer>,
    ///Optional. Thumbnail of the album cover to which the music file belongs
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,
}
///
///This object represents a general file (as opposed to photos, voice messages and audio files).
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Document {
    ///Identifier for this file
    pub file_id: String,
    ///Optional. Document thumbnail as defined by sender
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,
    ///Optional. Original filename as defined by sender
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    ///Optional. MIME type of the file as defined by sender
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    ///Optional. File size
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Integer>,
}
///
///This object represents a video file.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Video {
    ///Identifier for this file
    pub file_id: String,
    ///Video width as defined by sender
    pub width: Integer,
    ///Video height as defined by sender
    pub height: Integer,
    ///Duration of the video in seconds as defined by sender
    pub duration: Integer,
    ///Optional. Video thumbnail
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,
    ///Optional. Mime type of a file as defined by sender
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    ///Optional. File size
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Integer>,
}
///
///This object represents an animation file (GIF or H.264/MPEG-4 AVC video without sound).
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Animation {
    ///Identifier for this file
    pub file_id: String,
    ///Video width as defined by sender
    pub width: Integer,
    ///Video height as defined by sender
    pub height: Integer,
    ///Duration of the video in seconds as defined by sender
    pub duration: Integer,
    ///Optional. Animation thumbnail as defined by sender
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,
    ///Optional. Original animation filename as defined by sender
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    ///Optional. MIME type of the file as defined by sender
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    ///Optional. File size
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Integer>,
}
///
///This object represents a voice note.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Voice {
    ///Identifier for this file
    pub file_id: String,
    ///Duration of the audio in seconds as defined by sender
    pub duration: Integer,
    ///Optional. MIME type of the file as defined by sender
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    ///Optional. File size
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Integer>,
}
///
///This object represents a video message (available in Telegram apps as of v.4.0).
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct VideoNote {
    ///Identifier for this file
    pub file_id: String,
    ///Video width and height (diameter of the video message) as defined by sender
    pub length: Integer,
    ///Duration of the video in seconds as defined by sender
    pub duration: Integer,
    ///Optional. Video thumbnail
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,
    ///Optional. File size
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Integer>,
}
///
///This object represents a phone contact.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Contact {
    ///Contact's phone number
    pub phone_number: String,
    ///Contact's first name
    pub first_name: String,
    ///Optional. Contact's last name
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    ///Optional. Contact's user identifier in Telegram
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Integer>,
    ///Optional. Additional data about the contact in the form of a vCard
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
}
///
///This object represents a point on the map.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Location {
    ///Longitude as defined by sender
    pub longitude: Float,
    ///Latitude as defined by sender
    pub latitude: Float,
}
///
///This object represents a venue.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Venue {
    ///Venue location
    pub location: Location,
    ///Name of the venue
    pub title: String,
    ///Address of the venue
    pub address: String,
    ///Optional. Foursquare identifier of the venue
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    ///Optional. Foursquare type of the venue. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
}
///
///This object contains information about one answer option in a poll.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct PollOption {
    ///Option text, 1-100 characters
    pub text: String,
    ///Number of users that voted for this option
    pub voter_count: Integer,
}
///
///This object contains information about a poll.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Poll {
    ///Unique poll identifier
    pub id: String,
    ///Poll question, 1-255 characters
    pub question: String,
    ///List of poll options
    pub options: Vec<PollOption>,
    ///True, if the poll is closed
    pub is_closed: Boolean,
}
///
///This object represent a user's profile pictures.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct UserProfilePhotos {
    ///Total number of profile pictures the target user has
    pub total_count: Integer,
    ///Requested profile pictures (in up to 4 sizes each)
    pub photos: Vec<Vec<PhotoSize>>,
}
///
///This object represents a file ready to be downloaded. The file can be downloaded via the link https://api.telegram.org/file/bot<token>/<file_path>. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling getFile.
///
///    Maximum file size to download is 20 MB
///
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct File {
    ///Identifier for this file
    pub file_id: String,
    ///Optional. File size, if known
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Integer>,
    ///Optional. File path. Use https://api.telegram.org/file/bot<token>/<file_path> to get the file.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
}
///
///This object represents a custom keyboard with reply options (see Introduction to bots for details and examples).
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct ReplyKeyboardMarkup {
    ///Array of button rows, each represented by an Array of KeyboardButton objects
    pub keyboard: Vec<Vec<KeyboardButton>>,
    ///Optional. Requests clients to resize the keyboard vertically for optimal fit (e.g., make the keyboard smaller if there are just two rows of buttons). Defaults to false, in which case the custom keyboard is always of the same height as the app's standard keyboard.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_keyboard: Option<Boolean>,
    ///Optional. Requests clients to hide the keyboard as soon as it's been used. The keyboard will still be available, but clients will automatically display the usual letter-keyboard in the chat – the user can press a special button in the input field to see the custom keyboard again. Defaults to false.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_keyboard: Option<Boolean>,
    ///Optional. Use this parameter if you want to show the keyboard to specific users only. Targets: 1) users that are @mentioned in the text of the Message object; 2) if the bot's message is a reply (has reply_to_message_id), sender of the original message.
    #[builder(default)]
    ///
    ///Example: A user requests to change the bot‘s language, bot replies to the request with a keyboard to select the new language. Other users in the group don’t see the keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<Boolean>,
}
///
///This object represents one button of the reply keyboard. For simple text buttons String can be used instead of this object to specify text of the button. Optional fields are mutually exclusive.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct KeyboardButton {
    ///Text of the button. If none of the optional fields are used, it will be sent as a message when the button is pressed
    pub text: String,
    ///Optional. If True, the user's phone number will be sent as a contact when the button is pressed. Available in private chats only
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_contact: Option<Boolean>,
    ///Optional. If True, the user's current location will be sent when the button is pressed. Available in private chats only
    #[builder(default)]
    ///
    ///Note: request_contact and request_location options will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_location: Option<Boolean>,
}
///
///Upon receiving a message with this object, Telegram clients will remove the current custom keyboard and display the default letter-keyboard. By default, custom keyboards are displayed until a new keyboard is sent by a bot. An exception is made for one-time keyboards that are hidden immediately after the user presses a button (see ReplyKeyboardMarkup).
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct ReplyKeyboardRemove {
    ///Requests clients to remove the custom keyboard (user will not be able to summon this keyboard; if you want to hide the keyboard from sight but keep it accessible, use one_time_keyboard in ReplyKeyboardMarkup)
    pub remove_keyboard: True,
    ///Optional. Use this parameter if you want to remove the keyboard for specific users only. Targets: 1) users that are @mentioned in the text of the Message object; 2) if the bot's message is a reply (has reply_to_message_id), sender of the original message.
    #[builder(default)]
    ///
    ///Example: A user votes in a poll, bot returns confirmation message in reply to the vote and removes the keyboard for that user, while still showing the keyboard with poll options to users who haven't voted yet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<Boolean>,
}
///
///This object represents an inline keyboard that appears right next to the message it belongs to.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InlineKeyboardMarkup {
    ///
    ///Note: This will only work in Telegram versions released after 9 April, 2016. Older clients will display unsupported message.
    ///Array of button rows, each represented by an Array of InlineKeyboardButton objects
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}
///
///This object represents one button of an inline keyboard. You must use exactly one of the optional fields.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InlineKeyboardButton {
    ///Label text on the button
    pub text: String,
    ///Optional. HTTP or tg:// url to be opened when button is pressed
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    ///Optional. An HTTP URL used to automatically authorize the user. Can be used as a replacement for the Telegram Login Widget.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_url: Option<LoginUrl>,
    ///Optional. Data to be sent in a callback query to the bot when button is pressed, 1-64 bytes
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,
    ///Optional. If set, pressing the button will prompt the user to select one of their chats, open that chat and insert the bot‘s username and the specified inline query in the input field. Can be empty, in which case just the bot’s username will be inserted.
    #[builder(default)]
    ///
    ///Note: This offers an easy way for users to start using your bot in inline mode when they are currently in a private chat with it. Especially useful when combined with switch_pm… actions – in this case the user will be automatically returned to the chat they switched from, skipping the chat selection screen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query: Option<String>,
    ///Optional. If set, pressing the button will insert the bot‘s username and the specified inline query in the current chat's input field. Can be empty, in which case only the bot’s username will be inserted.
    #[builder(default)]
    ///
    ///This offers a quick way for the user to open your bot in inline mode in the same chat – good for selecting something from multiple options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_current_chat: Option<String>,
    ///Optional. Description of the game that will be launched when the user presses the button.
    #[builder(default)]
    ///
    ///NOTE: This type of button must always be the first button in the first row.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_game: Option<CallbackGame>,
    ///Optional. Specify True, to send a Pay button.
    #[builder(default)]
    ///
    ///NOTE: This type of button must always be the first button in the first row.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay: Option<Boolean>,
}
///
///This object represents a parameter of the inline keyboard button used to automatically authorize a user. Serves as a great replacement for the Telegram Login Widget when the user is coming from Telegram. All the user needs to do is tap/click a button and confirm that they want to log in:
///TITLE
///
///Telegram apps support these buttons as of version 5.7.
///
///    Sample bot: @discussbot
///
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct LoginUrl {
    ///
    ///NOTE: You must always check the hash of the received data to verify the authentication and the integrity of the data as described in Checking authorization.
    ///An HTTP URL to be opened with user authorization data added to the query string when the button is pressed. If the user refuses to provide authorization data, the original URL without information about the user will be opened. The data added is the same as described in Receiving authorization data.
    pub url: String,
    ///Optional. New text of the button in forwarded messages.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_text: Option<String>,
    ///Optional. Username of a bot, which will be used for user authorization. See Setting up a bot for more details. If not specified, the current bot's username will be assumed. The url's domain must be the same as the domain linked with the bot. See Linking your domain to the bot for more details.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_username: Option<String>,
    ///Optional. Pass True to request the permission for your bot to send messages to the user.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_write_access: Option<Boolean>,
}
///
///This object represents an incoming callback query from a callback button in an inline keyboard. If the button that originated the query was attached to a message sent by the bot, the field message will be present. If the button was attached to a message sent via the bot (in inline mode), the field inline_message_id will be present. Exactly one of the fields data or game_short_name will be present.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct CallbackQuery {
    ///Unique identifier for this query
    pub id: String,
    ///Sender
    pub from: User,
    ///Optional. Message with the callback button that originated the query. Note that message content and message date will not be available if the message is too old
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
    ///Optional. Identifier of the message sent via the bot in inline mode, that originated the query.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    ///Global identifier, uniquely corresponding to the chat to which the message with the callback button was sent. Useful for high scores in games.
    pub chat_instance: String,
    ///Optional. Data associated with the callback button. Be aware that a bad client can send arbitrary data in this field.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    ///Optional. Short name of a Game to be returned, serves as the unique identifier for the game
    #[builder(default)]
    ///
    ///    NOTE: After the user presses a callback button, Telegram clients will display a progress bar until you call answerCallbackQuery. It is, therefore, necessary to react by calling answerCallbackQuery even if no notification to the user is needed (e.g., without specifying any of the optional parameters).
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_short_name: Option<String>,
}
///
///Upon receiving a message with this object, Telegram clients will display a reply interface to the user (act as if the user has selected the bot‘s message and tapped ’Reply'). This can be extremely useful if you want to create user-friendly step-by-step interfaces without having to sacrifice privacy mode.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct ForceReply {
    ///Shows reply interface to the user, as if they manually selected the bot‘s message and tapped ’Reply'
    pub force_reply: True,
    ///Optional. Use this parameter if you want to force reply from specific users only. Targets: 1) users that are @mentioned in the text of the Message object; 2) if the bot's message is a reply (has reply_to_message_id), sender of the original message.
    #[builder(default)]
    ///
    ///    Example: A poll bot for groups runs in privacy mode (only receives commands, replies to its messages and mentions). There could be two ways to create a new poll:
    ///
    ///        Explain the user how to send a command with parameters (e.g. /newpoll question answer1 answer2). May be appealing for hardcore users but lacks modern day polish.
    ///        Guide the user through a step-by-step process. ‘Please send me your question’, ‘Cool, now let’s add the first answer option‘, ’Great. Keep adding answer options, then send /done when you‘re ready’.
    ///
    ///    The last option is definitely more attractive. And if you use ForceReply in your bot‘s questions, it will receive the user’s answers even if it only receives replies, commands and mentions — without any extra work for the user.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<Boolean>,
}
///
///This object represents a chat photo.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct ChatPhoto {
    ///File identifier of small (160x160) chat photo. This file_id can be used only for photo download and only for as long as the photo is not changed.
    pub small_file_id: String,
    ///File identifier of big (640x640) chat photo. This file_id can be used only for photo download and only for as long as the photo is not changed.
    pub big_file_id: String,
}
///
///This object contains information about one member of a chat.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct ChatMember {
    ///Information about the user
    pub user: User,
    ///The member's status in the chat. Can be “creator”, “administrator”, “member”, “restricted”, “left” or “kicked”
    pub status: String,
    ///Optional. Restricted and kicked only. Date when restrictions will be lifted for this user; unix time
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<Integer>,
    ///Optional. Administrators only. True, if the bot is allowed to edit administrator privileges of that user
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_be_edited: Option<Boolean>,
    ///Optional. Administrators only. True, if the administrator can post in the channel; channels only
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<Boolean>,
    ///Optional. Administrators only. True, if the administrator can edit messages of other users and can pin messages; channels only
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<Boolean>,
    ///Optional. Administrators only. True, if the administrator can delete messages of other users
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_messages: Option<Boolean>,
    ///Optional. Administrators only. True, if the administrator can restrict, ban or unban chat members
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_restrict_members: Option<Boolean>,
    ///Optional. Administrators only. True, if the administrator can add new administrators with a subset of his own privileges or demote administrators that he has promoted, directly or indirectly (promoted by administrators that were appointed by the user)
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_promote_members: Option<Boolean>,
    ///Optional. Administrators and restricted only. True, if the user is allowed to change the chat title, photo and other settings
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<Boolean>,
    ///Optional. Administrators and restricted only. True, if the user is allowed to invite new users to the chat
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<Boolean>,
    ///Optional. Administrators and restricted only. True, if the user is allowed to pin messages; groups and supergroups only
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<Boolean>,
    ///Optional. Restricted only. True, if the user is a member of the chat at the moment of the request
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_member: Option<Boolean>,
    ///Optional. Restricted only. True, if the user is allowed to send text messages, contacts, locations and venues
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_messages: Option<Boolean>,
    ///Optional. Restricted only. True, if the user is allowed to send audios, documents, photos, videos, video notes and voice notes
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_media_messages: Option<Boolean>,
    ///Optional. Restricted only. True, if the user is allowed to send polls
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_polls: Option<Boolean>,
    ///Optional. Restricted only. True, if the user is allowed to send animations, games, stickers and use inline bots
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_other_messages: Option<Boolean>,
    ///Optional. Restricted only. True, if the user is allowed to add web page previews to their messages
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_add_web_page_previews: Option<Boolean>,
}
///
///Describes actions that a non-administrator user is allowed to take in a chat.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct ChatPermissions {
    ///Optional. True, if the user is allowed to send text messages, contacts, locations and venues
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_messages: Option<Boolean>,
    ///Optional. True, if the user is allowed to send audios, documents, photos, videos, video notes and voice notes, implies can_send_messages
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_media_messages: Option<Boolean>,
    ///Optional. True, if the user is allowed to send polls, implies can_send_messages
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_polls: Option<Boolean>,
    ///Optional. True, if the user is allowed to send animations, games, stickers and use inline bots, implies can_send_media_messages
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_other_messages: Option<Boolean>,
    ///Optional. True, if the user is allowed to add web page previews to their messages, implies can_send_media_messages
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_add_web_page_previews: Option<Boolean>,
    ///Optional. True, if the user is allowed to change the chat title, photo and other settings. Ignored in public supergroups
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<Boolean>,
    ///Optional. True, if the user is allowed to invite new users to the chat
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<Boolean>,
    ///Optional. True, if the user is allowed to pin messages. Ignored in public supergroups
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<Boolean>,
}
///
///Contains information about why a request was unsuccessful.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct ResponseParameters {
    ///Optional. The group has been migrated to a supergroup with the specified identifier. This number may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<Integer>,
    ///Optional. In case of exceeding flood control, the number of seconds left to wait before the request can be repeated
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<Integer>,
}
///
///Represents a photo to be sent.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InputMediaPhoto {
    ///Type of the result, must be photo
    #[serde(rename = "type")]
    pub type_: String,
    ///File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More info on Sending Files »
    pub media: String,
    ///Optional. Caption of the photo to be sent, 0-1024 characters
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    ///Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
}
///
///Represents a video to be sent.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InputMediaVideo {
    ///Type of the result, must be video
    #[serde(rename = "type")]
    pub type_: String,
    ///File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More info on Sending Files »
    pub media: String,
    ///Optional. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail‘s width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can’t be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More info on Sending Files »
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<Or<InputFile, String>>,
    ///Optional. Caption of the video to be sent, 0-1024 characters
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    ///Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    ///Optional. Video width
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Integer>,
    ///Optional. Video height
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Integer>,
    ///Optional. Video duration
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Integer>,
    ///Optional. Pass True, if the uploaded video is suitable for streaming
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<Boolean>,
}
///
///Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InputMediaAnimation {
    ///Type of the result, must be animation
    #[serde(rename = "type")]
    pub type_: String,
    ///File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More info on Sending Files »
    pub media: String,
    ///Optional. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail‘s width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can’t be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More info on Sending Files »
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<Or<InputFile, String>>,
    ///Optional. Caption of the animation to be sent, 0-1024 characters
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    ///Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    ///Optional. Animation width
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Integer>,
    ///Optional. Animation height
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Integer>,
    ///Optional. Animation duration
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Integer>,
}
///
///Represents an audio file to be treated as music to be sent.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InputMediaAudio {
    ///Type of the result, must be audio
    #[serde(rename = "type")]
    pub type_: String,
    ///File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More info on Sending Files »
    pub media: String,
    ///Optional. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail‘s width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can’t be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More info on Sending Files »
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<Or<InputFile, String>>,
    ///Optional. Caption of the audio to be sent, 0-1024 characters
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    ///Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    ///Optional. Duration of the audio in seconds
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Integer>,
    ///Optional. Performer of the audio
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    ///Optional. Title of the audio
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
///
///Represents a general file to be sent.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct InputMediaDocument {
    ///Type of the result, must be document
    #[serde(rename = "type")]
    pub type_: String,
    ///File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More info on Sending Files »
    pub media: String,
    ///Optional. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail‘s width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can’t be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More info on Sending Files »
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<Or<InputFile, String>>,
    ///Optional. Caption of the document to be sent, 0-1024 characters
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    ///Optional. Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
}
///
///This object represents a sticker.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Sticker {
    ///Identifier for this file
    pub file_id: String,
    ///Sticker width
    pub width: Integer,
    ///Sticker height
    pub height: Integer,
    ///True, if the sticker is animated
    pub is_animated: Boolean,
    ///Optional. Sticker thumbnail in the .webp or .jpg format
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,
    ///Optional. Emoji associated with the sticker
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    ///Optional. Name of the sticker set to which the sticker belongs
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_name: Option<String>,
    ///Optional. For mask stickers, the position where the mask should be placed
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
    ///Optional. File size
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Integer>,
}
///
///This object represents a sticker set.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct StickerSet {
    ///Sticker set name
    pub name: String,
    ///Sticker set title
    pub title: String,
    ///True, if the sticker set contains animated stickers
    pub is_animated: Boolean,
    ///True, if the sticker set contains masks
    pub contains_masks: Boolean,
    ///List of all set stickers
    pub stickers: Vec<Sticker>,
}
///
///This object describes the position on faces where a mask should be placed by default.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct MaskPosition {
    ///The part of the face relative to which the mask should be placed. One of “forehead”, “eyes”, “mouth”, or “chin”.
    pub point: String,
    ///Shift by X-axis measured in widths of the mask scaled to the face size, from left to right. For example, choosing -1.0 will place mask just to the left of the default mask position.
    pub x_shift: Float,
    ///Shift by Y-axis measured in heights of the mask scaled to the face size, from top to bottom. For example, 1.0 will place the mask just below the default mask position.
    pub y_shift: Float,
    ///Mask scaling coefficient. For example, 2.0 means double size.
    pub scale: Float,
}
///
///This object represents a portion of the price for goods or services.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct LabeledPrice {
    ///Portion label
    pub label: String,
    ///Price of the product in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub amount: Integer,
}
///
///This object contains basic information about an invoice.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Invoice {
    ///Product name
    pub title: String,
    ///Product description
    pub description: String,
    ///Unique bot deep-linking parameter that can be used to generate this invoice
    pub start_parameter: String,
    ///Three-letter ISO 4217 currency code
    pub currency: String,
    ///Total price in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub total_amount: Integer,
}
///
///This object represents a shipping address.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct ShippingAddress {
    ///ISO 3166-1 alpha-2 country code
    pub country_code: String,
    ///State, if applicable
    pub state: String,
    ///City
    pub city: String,
    ///First line for the address
    pub street_line1: String,
    ///Second line for the address
    pub street_line2: String,
    ///Address post code
    pub post_code: String,
}
///
///This object represents information about an order.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct OrderInfo {
    ///Optional. User name
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Optional. User's phone number
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    ///Optional. User email
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    ///Optional. User shipping address
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddress>,
}
///
///This object represents one shipping option.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct ShippingOption {
    ///Shipping option identifier
    pub id: String,
    ///Option title
    pub title: String,
    ///List of price portions
    pub prices: Vec<LabeledPrice>,
}
///
///This object contains basic information about a successful payment.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct SuccessfulPayment {
    ///Three-letter ISO 4217 currency code
    pub currency: String,
    ///Total price in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub total_amount: Integer,
    ///Bot specified invoice payload
    pub invoice_payload: String,
    ///Optional. Identifier of the shipping option chosen by the user
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,
    ///Optional. Order info provided by the user
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<OrderInfo>,
    ///Telegram payment identifier
    pub telegram_payment_charge_id: String,
    ///Provider payment identifier
    pub provider_payment_charge_id: String,
}
///
///This object contains information about an incoming shipping query.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct ShippingQuery {
    ///Unique query identifier
    pub id: String,
    ///User who sent the query
    pub from: User,
    ///Bot specified invoice payload
    pub invoice_payload: String,
    ///User specified shipping address
    pub shipping_address: ShippingAddress,
}
///
///This object contains information about an incoming pre-checkout query.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct PreCheckoutQuery {
    ///Unique query identifier
    pub id: String,
    ///User who sent the query
    pub from: User,
    ///Three-letter ISO 4217 currency code
    pub currency: String,
    ///Total price in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub total_amount: Integer,
    ///Bot specified invoice payload
    pub invoice_payload: String,
    ///Optional. Identifier of the shipping option chosen by the user
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,
    ///Optional. Order info provided by the user
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<OrderInfo>,
}
///
///Contains information about Telegram Passport data shared with the bot by the user.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct PassportData {
    ///Array with information about documents and other Telegram Passport elements that was shared with the bot
    pub data: Vec<EncryptedPassportElement>,
    ///Encrypted credentials required to decrypt the data
    pub credentials: EncryptedCredentials,
}
///
///This object represents a file uploaded to Telegram Passport. Currently all Telegram Passport files are in JPEG format when decrypted and don't exceed 10MB.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct PassportFile {
    ///Identifier for this file
    pub file_id: String,
    ///File size
    pub file_size: Integer,
    ///Unix time when the file was uploaded
    pub file_date: Integer,
}
///
///Contains information about documents or other Telegram Passport elements shared with the bot by the user.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct EncryptedPassportElement {
    ///Element type. One of “personal_details”, “passport”, “driver_license”, “identity_card”, “internal_passport”, “address”, “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”, “phone_number”, “email”.
    #[serde(rename = "type")]
    pub type_: String,
    ///Optional. Base64-encoded encrypted Telegram Passport element data provided by the user, available for “personal_details”, “passport”, “driver_license”, “identity_card”, “internal_passport” and “address” types. Can be decrypted and verified using the accompanying EncryptedCredentials.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    ///Optional. User's verified phone number, available only for “phone_number” type
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    ///Optional. User's verified email address, available only for “email” type
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    ///Optional. Array of encrypted files with documents provided by the user, available for “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration” and “temporary_registration” types. Files can be decrypted and verified using the accompanying EncryptedCredentials.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<PassportFile>>,
    ///Optional. Encrypted file with the front side of the document, provided by the user. Available for “passport”, “driver_license”, “identity_card” and “internal_passport”. The file can be decrypted and verified using the accompanying EncryptedCredentials.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_side: Option<PassportFile>,
    ///Optional. Encrypted file with the reverse side of the document, provided by the user. Available for “driver_license” and “identity_card”. The file can be decrypted and verified using the accompanying EncryptedCredentials.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_side: Option<PassportFile>,
    ///Optional. Encrypted file with the selfie of the user holding a document, provided by the user; available for “passport”, “driver_license”, “identity_card” and “internal_passport”. The file can be decrypted and verified using the accompanying EncryptedCredentials.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfie: Option<PassportFile>,
    ///Optional. Array of encrypted files with translated versions of documents provided by the user. Available if requested for “passport”, “driver_license”, “identity_card”, “internal_passport”, “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration” and “temporary_registration” types. Files can be decrypted and verified using the accompanying EncryptedCredentials.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<Vec<PassportFile>>,
    ///Base64-encoded element hash for using in PassportElementErrorUnspecified
    pub hash: String,
}
///
///Contains data required for decrypting and authenticating EncryptedPassportElement. See the Telegram Passport Documentation for a complete description of the data decryption and authentication processes.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct EncryptedCredentials {
    ///Base64-encoded encrypted JSON-serialized data with unique user's payload, data hashes and secrets required for EncryptedPassportElement decryption and authentication
    pub data: String,
    ///Base64-encoded data hash for data authentication
    pub hash: String,
    ///Base64-encoded secret, encrypted with the bot's public RSA key, required for data decryption
    pub secret: String,
}
///
///Represents an issue in one of the data fields that was provided by the user. The error is considered resolved when the field's value changes.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct PassportElementErrorDataField {
    ///Error source, must be data
    pub source: String,
    ///The section of the user's Telegram Passport which has the error, one of “personal_details”, “passport”, “driver_license”, “identity_card”, “internal_passport”, “address”
    #[serde(rename = "type")]
    pub type_: String,
    ///Name of the data field which has the error
    pub field_name: String,
    ///Base64-encoded data hash
    pub data_hash: String,
    ///Error message
    pub message: String,
}
///
///Represents an issue with the front side of a document. The error is considered resolved when the file with the front side of the document changes.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct PassportElementErrorFrontSide {
    ///Error source, must be front_side
    pub source: String,
    ///The section of the user's Telegram Passport which has the issue, one of “passport”, “driver_license”, “identity_card”, “internal_passport”
    #[serde(rename = "type")]
    pub type_: String,
    ///Base64-encoded hash of the file with the front side of the document
    pub file_hash: String,
    ///Error message
    pub message: String,
}
///
///Represents an issue with the reverse side of a document. The error is considered resolved when the file with reverse side of the document changes.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct PassportElementErrorReverseSide {
    ///Error source, must be reverse_side
    pub source: String,
    ///The section of the user's Telegram Passport which has the issue, one of “driver_license”, “identity_card”
    #[serde(rename = "type")]
    pub type_: String,
    ///Base64-encoded hash of the file with the reverse side of the document
    pub file_hash: String,
    ///Error message
    pub message: String,
}
///
///Represents an issue with the selfie with a document. The error is considered resolved when the file with the selfie changes.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct PassportElementErrorSelfie {
    ///Error source, must be selfie
    pub source: String,
    ///The section of the user's Telegram Passport which has the issue, one of “passport”, “driver_license”, “identity_card”, “internal_passport”
    #[serde(rename = "type")]
    pub type_: String,
    ///Base64-encoded hash of the file with the selfie
    pub file_hash: String,
    ///Error message
    pub message: String,
}
///
///Represents an issue with a document scan. The error is considered resolved when the file with the document scan changes.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct PassportElementErrorFile {
    ///Error source, must be file
    pub source: String,
    ///The section of the user's Telegram Passport which has the issue, one of “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”
    #[serde(rename = "type")]
    pub type_: String,
    ///Base64-encoded file hash
    pub file_hash: String,
    ///Error message
    pub message: String,
}
///
///Represents an issue with a list of scans. The error is considered resolved when the list of files containing the scans changes.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct PassportElementErrorFiles {
    ///Error source, must be files
    pub source: String,
    ///The section of the user's Telegram Passport which has the issue, one of “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”
    #[serde(rename = "type")]
    pub type_: String,
    ///List of base64-encoded file hashes
    pub file_hashes: Vec<String>,
    ///Error message
    pub message: String,
}
///
///Represents an issue with one of the files that constitute the translation of a document. The error is considered resolved when the file changes.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct PassportElementErrorTranslationFile {
    ///Error source, must be translation_file
    pub source: String,
    ///Type of element of the user's Telegram Passport which has the issue, one of “passport”, “driver_license”, “identity_card”, “internal_passport”, “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”
    #[serde(rename = "type")]
    pub type_: String,
    ///Base64-encoded file hash
    pub file_hash: String,
    ///Error message
    pub message: String,
}
///
///Represents an issue with the translated version of a document. The error is considered resolved when a file with the document translation change.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct PassportElementErrorTranslationFiles {
    ///Error source, must be translation_files
    pub source: String,
    ///Type of element of the user's Telegram Passport which has the issue, one of “passport”, “driver_license”, “identity_card”, “internal_passport”, “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”
    #[serde(rename = "type")]
    pub type_: String,
    ///List of base64-encoded file hashes
    pub file_hashes: Vec<String>,
    ///Error message
    pub message: String,
}
///
///Represents an issue in an unspecified place. The error is considered resolved when new data is added.
#[derive(Serialize, Deserialize, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct PassportElementErrorUnspecified {
    ///Error source, must be unspecified
    pub source: String,
    ///Type of element of the user's Telegram Passport which has the issue
    #[serde(rename = "type")]
    pub type_: String,
    ///Base64-encoded element hash
    pub element_hash: String,
    ///Error message
    pub message: String,
}
