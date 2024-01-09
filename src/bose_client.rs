use anyhow::{Context, Error, Result};
use quick_xml::DeError;
use reqwest::IntoUrl;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Debug;

pub struct BoseClient {
    hostname: String,
}

impl BoseClient {
    pub fn new(hostname: &str) -> BoseClient {
        BoseClient {
            hostname: String::from(hostname),
        }
    }

    pub fn play(&self) -> Result<()> {
        self.press_and_release_key(&KeyValue::Play)
    }

    pub fn pause(&self) -> Result<()> {
        self.press_and_release_key(&KeyValue::Pause)
    }

    pub fn power(&self) -> Result<()> {
        self.press_and_release_key(&KeyValue::Power)
    }

    fn press_and_release_key(&self, key_value: &KeyValue) -> anyhow::Result<()> {
        let url = format!("http://{}:8090/key", &self.hostname);
        let _ = post_xml(&url, &PostKey::press(key_value))?;
        let _ = post_xml(&url, &PostKey::release(key_value))?;
        Ok(())
    }

    pub fn print_status(&self) -> Result<()> {
        let url = format!("http://{}:8090/now_playing", &self.hostname);
        let now_playing: NowPlaying = get_xml(url)?;
        if now_playing.source == "STANDBY" {
            println!("system is in standby mode.")
        } else {
            println!(
                "system is now playing: {} - {}",
                now_playing.station_name.unwrap(),
                now_playing.artist.unwrap()
            );
        }
        Ok(())
    }

    pub fn print_volume(&self) -> Result<()> {
        let url = format!("http://{}:8090/volume", &self.hostname);
        let volume: Volume = get_xml(url)?;
        print!("the volume is: {:?}", &volume);
        Ok(())
    }

    pub fn set_volume(&self, value: i32) -> Result<()> {
        let url = format!("http://{}:8090/volume", &self.hostname);
        let _ = post_xml(&url, &PostVolume::new(value))?;
        Ok(())
    }

    pub fn print_presets(&self) -> Result<()> {
        let url = format!("http://{}:8090/presets", &self.hostname);
        let presets: Presets = get_xml(url)?;
        println!("the presets are: ");
        for preset in presets.items {
            println!(
                "{} - {} ({})",
                preset.id, preset.content_item.name, preset.content_item.source
            )
        }

        Ok(())
    }

    pub fn set_preset(&self, value: i32) -> Result<()> {
        match value {
            1 => self.press_and_release_key(&KeyValue::Preset1),
            2 => self.press_and_release_key(&KeyValue::Preset2),
            3 => self.press_and_release_key(&KeyValue::Preset3),
            4 => self.press_and_release_key(&KeyValue::Preset4),
            5 => self.press_and_release_key(&KeyValue::Preset5),
            6 => self.press_and_release_key(&KeyValue::Preset6),
            _ => Err(Error::msg(format!(
                "{} is not a valid preset (1-6).",
                value
            ))),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "UPPERCASE")]
enum KeyValue {
    Play,
    Pause,
    //STOP,
    //PREV_TRACK,
    //NEXT_TRACK,
    //THUMBS_UP,
    //THUMBS_DOWN,
    //BOOKMARK,
    Power,
    //MUTE,
    //VOLUME_UP,
    //VOLUME_DOWN,
    #[serde(rename(serialize = "PRESET_1"))]
    Preset1,
    #[serde(rename(serialize = "PRESET_2"))]
    Preset2,
    #[serde(rename(serialize = "PRESET_3"))]
    Preset3,
    #[serde(rename(serialize = "PRESET_4"))]
    Preset4,
    #[serde(rename(serialize = "PRESET_5"))]
    Preset5,
    #[serde(rename(serialize = "PRESET_6"))]
    Preset6,
    //AUX_INPUT,
    //SHUFFLE_OFF,
    //SHUFFLE_ON,
    //REPEAT_OFF,
    //REPEAT_ONE,
    //REPEAT_ALL,
    //PLAY_PAUSE,
    //ADD_FAVORITE,
    //REMOVE_FAVORITE,
    //INVALID_KEY,
}

impl fmt::Display for KeyValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
enum KeyState {
    PRESS,
    RELEASE,
}

impl fmt::Display for KeyState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename(deserialize = "nowPlaying"))]
#[allow(dead_code)]
struct NowPlaying {
    #[serde(rename = "@deviceID")]
    device_id: String,
    #[serde(rename = "@source")]
    source: String,
    #[serde(rename = "@sourceAccount")]
    source_account: Option<String>,
    #[serde(rename = "ContentItem")]
    content_item: NowPlayingContentItem,
    track: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    #[serde(rename = "stationName")]
    station_name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct NowPlayingContentItem {
    #[serde(rename = "@source")]
    source: String,
    #[serde(rename = "@type")]
    content_type: Option<String>,
    #[serde(rename = "@location")]
    location: Option<String>,
    #[serde(rename = "@isPresetable")]
    is_presetable: bool,
    #[serde(rename = "itemName")]
    name: Option<String>,
    #[serde(rename = "containerArt")]
    container_art: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename(deserialize = "volume"))]
#[allow(dead_code)]
struct Volume {
    #[serde(rename = "targetvolume")]
    target: i32,
    #[serde(rename = "actualvolume")]
    actual: i32,
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "volume"))]
struct PostVolume {
    #[serde(rename = "$value")]
    value: i32,
}

impl PostVolume {
    pub fn new(value: i32) -> PostVolume {
        PostVolume { value: value }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "key"))]
struct PostKey {
    #[serde(rename = "@state")]
    state: KeyState,
    #[serde(rename = "@sender")]
    sender: String,
    #[serde(rename = "$text")]
    value: KeyValue,
}

impl PostKey {
    pub fn press(value: &KeyValue) -> PostKey {
        PostKey {
            state: KeyState::PRESS,
            sender: "Gabbo".to_string(),
            value: value.clone(),
        }
    }

    pub fn release(value: &KeyValue) -> PostKey {
        PostKey {
            state: KeyState::RELEASE,
            sender: "Gabbo".to_string(),
            value: value.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename(deserialize = "nowPlaying"))]
#[allow(dead_code)]
struct Presets {
    #[serde(rename = "$value", default)]
    items: Vec<Preset>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Preset {
    #[serde(rename = "@id")]
    id: i32,
    #[serde(rename = "@createdOn")]
    created_on: i32,
    #[serde(rename = "@updatedOn")]
    updated_on: i32,
    #[serde(rename = "$value")]
    content_item: PresetContentItem,
}

#[derive(Debug, Serialize, Deserialize)]
struct PresetContentItem {
    #[serde(rename = "@source")]
    source: String,
    #[serde(rename = "@type")]
    preset_type: String,
    #[serde(rename = "@location")]
    location: String,
    #[serde(rename = "@sourceAccount")]
    source_account: String,
    #[serde(rename = "@isPresetable")]
    is_presetable: bool,
    #[serde(rename = "itemName")]
    name: String,
    #[serde(rename = "containerArt")]
    container_art: String,
    //#[serde(rename = "$value")]
    //<itemName>RGR</itemName><containerArt>http://cdn-profiles.tunein.com/s214611/images/logoq.jpg?t=152269</containerArt>
}

#[derive(Debug, Serialize, Deserialize)]
struct PresetContentItemValue {
    //#[serde(rename = "itemName")]
    //item_name: String
}

fn serialize_xml<T>(value: &T) -> std::result::Result<String, DeError>
where
    T: ?Sized + Serialize,
{
    quick_xml::se::to_string(value)
}

fn post_xml<U: IntoUrl + Debug + Clone, T: ?Sized + Serialize + Debug>(
    url: U,
    data: &T,
) -> Result<()> {
    let client = reqwest::blocking::Client::new();
    let body =
        serialize_xml(data).with_context(|| format!("Failed to serialize {:?} into xml", data))?;
    let _ = client
        .post(url.clone())
        .body(body.clone())
        .send()
        .with_context(|| format!("Failed to post {:?} to {:?}", &body, &url))?;
    //println!("posted {:?} to {:?}", body, url);
    Ok(())
}

fn get_xml<U: IntoUrl + Debug + Clone, T: DeserializeOwned>(url: U) -> Result<T> {
    let response =
        reqwest::blocking::get(url.clone()).with_context(|| format!("Failed to get {:?}", url))?;
    let body = response
        .text()
        .with_context(|| format!("Failed to extract body from response"))?;
    //println!("received body: {}", &body);
    let value: T = quick_xml::de::from_str(&body)
        .with_context(|| format!("Failed to deserialize {} into struct", body))?;
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_serializer() {
        assert_eq!(
            "<key state=\"press\" sender=\"Gabbo\">POWER</key>".to_string(),
            serialize_xml(&PostKey::press(&KeyValue::Power)).unwrap()
        )
    }

    #[test]
    fn test_preset_key_serializer() {
        assert_eq!(
            "<key state=\"press\" sender=\"Gabbo\">PRESET_1</key>".to_string(),
            serialize_xml(&PostKey::press(&KeyValue::Preset1)).unwrap()
        )
    }

    #[test]
    fn test_volume_serializer() {
        assert_eq!(
            "<volume>9</volume>".to_string(),
            serialize_xml(&PostVolume { value: 9 }).unwrap()
        )
    }

    #[test]
    #[ignore]
    fn test_set_volume() {
        let client = BoseClient::new("localhost");
        client.set_volume(10).unwrap();
    }
}
