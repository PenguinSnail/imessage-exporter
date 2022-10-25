/*!
  App messages are a specific type of message that developers can generate with their apps.
  Some built-in functionality also uses App Messages, like Apple Pay or Handwriting.
*/

use std::collections::HashMap;

use plist::Value;

use crate::{
    error::plist::PlistParseError, message_types::variants::BalloonProvider,
    util::plist::extract_parsed_str,
};

/// This struct represents Apple's [`MSMessageTemplateLayout`](https://developer.apple.com/documentation/messages/msmessagetemplatelayout).
#[derive(Debug, PartialEq, Eq)]
struct AppMessage<'a> {
    /// An image used to represent the message in the transcript
    image: Option<&'a str>,
    /// A media file used to represent the message in the transcript
    url: Option<&'a str>,
    /// The title for the image or media file
    title: Option<&'a str>,
    /// The subtitle for the image or media file
    subtitle: Option<&'a str>,
    /// A left-aligned caption for the message bubble
    caption: Option<&'a str>,
    /// A left-aligned subcaption for the message bubble
    subcaption: Option<&'a str>,
    /// A right-aligned caption for the message bubble
    trailing_caption: Option<&'a str>,
    /// A right-aligned subcaption for the message bubble
    trailing_subcaption: Option<&'a str>,
    /// The name of the app that created this message
    app_name: Option<&'a str>,
    /// This property is set only for Apple Pay system messages
    /// It represents the text that displays in the center of the bubble
    ldtext: Option<&'a str>,
}

impl<'a> BalloonProvider<'a> for AppMessage<'a> {
    fn from_map(payload: &'a HashMap<&'a str, &'a Value>) -> Result<Self, PlistParseError<'a>> {
        Ok(AppMessage {
            image: extract_parsed_str(payload, "image"),
            url: extract_parsed_str(payload, "URL"),
            title: extract_parsed_str(payload, "image-title"),
            subtitle: extract_parsed_str(payload, "image-subtitle"),
            caption: extract_parsed_str(payload, "caption"),
            subcaption: extract_parsed_str(payload, "subcaption"),
            trailing_caption: extract_parsed_str(payload, "secondary-subcaption"),
            trailing_subcaption: extract_parsed_str(payload, "tertiary-subcaption"),
            app_name: extract_parsed_str(payload, "an"),
            ldtext: extract_parsed_str(payload, "ldtext"),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        message_types::{app::AppMessage, variants::BalloonProvider},
        util::plist::parse_plist,
    };
    use plist::Value;
    use std::env::current_dir;
    use std::fs::File;

    #[test]
    fn test_parse_apple_pay_sent_265() {
        let plist_path = current_dir()
            .unwrap()
            .as_path()
            .join("test_data/Sent265.plist");
        let plist_data = File::open(plist_path).unwrap();
        let plist = Value::from_reader(plist_data).unwrap();
        let parsed = parse_plist(&plist).unwrap();

        let balloon = AppMessage::from_map(&parsed).unwrap();
        let expected = AppMessage {
            image: None,
            url: Some("data:application/vnd.apple.pkppm;base64,FAKE_BASE64_DATA="),
            title: None,
            subtitle: None,
            caption: Some("Apple\u{a0}Cash"),
            subcaption: Some("$265\u{a0}Payment"),
            trailing_caption: None,
            trailing_subcaption: None,
            app_name: Some("Apple\u{a0}Pay"),
            ldtext: Some("Sent $265 with Apple\u{a0}Pay."),
        };

        assert_eq!(balloon, expected);
    }

    #[test]
    fn test_parse_opentable_invite() {
        let plist_path = current_dir()
            .unwrap()
            .as_path()
            .join("test_data/OpenTableInvited.plist");
        let plist_data = File::open(plist_path).unwrap();
        let plist = Value::from_reader(plist_data).unwrap();
        println!("{:?}", parse_plist(&plist).unwrap());
    }

    #[test]
    fn test_parse_slideshow() {
        let plist_path = current_dir()
            .unwrap()
            .as_path()
            .join("test_data/Slideshow.plist");
        let plist_data = File::open(plist_path).unwrap();
        let plist = Value::from_reader(plist_data).unwrap();
        println!("{:?}", parse_plist(&plist).unwrap());
    }

    #[test]
    fn test_parse_game() {
        let plist_path = current_dir()
            .unwrap()
            .as_path()
            .join("test_data/Game.plist");
        let plist_data = File::open(plist_path).unwrap();
        let plist = Value::from_reader(plist_data).unwrap();
        println!("{:?}", parse_plist(&plist).unwrap());
    }

    #[test]
    fn test_parse_business() {
        let plist_path = current_dir()
            .unwrap()
            .as_path()
            .join("test_data/Business.plist");
        let plist_data = File::open(plist_path).unwrap();
        let plist = Value::from_reader(plist_data).unwrap();
        println!("{:?}", parse_plist(&plist).unwrap());
    }
}
