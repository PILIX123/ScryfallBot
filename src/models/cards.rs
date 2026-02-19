use chrono::NaiveDate;
use http::Uri;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{DisplayFromStr, serde_as};
use std::collections::HashMap;
use uuid::Uuid;

#[serde_as]
#[derive(Serialize, Deserialize, Default)]
pub struct Card {
    pub arena_id: Option<i32>,
    pub id: Uuid,
    pub lang: String,
    pub mtgo_id: Option<i32>,
    pub mtgo_foil_id: Option<i32>,
    pub multiverse_ids: Option<Vec<i32>>,
    pub resource_id: Option<String>,
    pub tcgplayer_id: Option<i32>,
    pub tcgplayer_etched_id: Option<i32>,
    pub cardmarket_id: Option<i32>,
    pub object: String,
    pub layout: String,
    pub oracle_id: Option<Uuid>,
    #[serde_as(as = "DisplayFromStr")]
    pub prints_search_uri: Uri,
    #[serde_as(as = "DisplayFromStr")]
    pub rulings_uri: Uri,
    #[serde_as(as = "DisplayFromStr")]
    pub scryfall_uri: Uri,
    #[serde_as(as = "DisplayFromStr")]
    pub uri: Uri,
    pub all_parts: Option<Vec<RelatedCardObjects>>,
    pub card_faces: Option<Vec<CardFace>>,
    pub cmc: f32,
    pub color_identity: Vec<Colors>,
    pub colors: Option<Vec<Colors>>,
    pub color_indicator: Option<Vec<Colors>>,
    pub defense: Option<String>,
    pub edhrec_rank: Option<i32>,
    pub game_changer: Option<bool>,
    pub hand_modifier: Option<String>,
    pub keywords: Vec<String>,
    pub legalities: HashMap<String, Legalities>,
    pub life_modifier: Option<String>,
    pub loyalty: Option<String>,
    pub mana_cost: Option<String>,
    pub name: String,
    pub oracle_text: Option<String>,
    pub penny_rank: Option<i32>,
    pub power: Option<String>,
    pub produced_mana: Option<Vec<Colors>>,
    pub reserved: bool,
    pub toughness: Option<String>,
    pub type_line: String,
    pub artist: Option<String>,
    pub artist_ids: Option<Vec<Uuid>>,
    pub attraction_lights: Option<Vec<i32>>,
    pub booster: bool,
    pub border_color: BorderColor,
    pub card_back_id: Uuid,
    pub collector_number: String,
    pub content_warning: Option<bool>,
    pub digital: bool,
    pub finishes: Vec<Finishes>,
    pub flavor_name: Option<String>,
    pub flavor_text: Option<String>,
    pub frame_effects: Option<Vec<FrameEffects>>,
    pub frame: Frames,
    pub full_art: bool,
    pub games: Vec<Games>,
    pub highres_image: bool,
    pub illustration_id: Option<Uuid>,
    pub image_status: ImageStatuses,
    pub image_uris: Option<HashMap<String, Value>>,
    pub oversized: bool,
    pub prices: HashMap<String, Value>,
    pub printed_name: Option<String>,
    pub printed_text: Option<String>,
    pub printed_type_line: Option<String>,
    pub promo: bool,
    pub promo_types: Option<Vec<String>>,
    pub purchase_uris: Option<HashMap<String, Value>>,
    pub rarity: Rarities,
    pub related_uris: HashMap<String, Value>,
    pub released_at: NaiveDate,
    pub reprint: bool,
    #[serde_as(as = "DisplayFromStr")]
    pub scryfall_set_uri: Uri,
    pub set_name: String,
    #[serde_as(as = "DisplayFromStr")]
    pub set_search_uri: Uri,
    pub set_type: String,
    #[serde_as(as = "DisplayFromStr")]
    pub set_uri: Uri,
    pub set: String,
    pub set_id: Uuid,
    pub story_spotlight: bool,
    pub textless: bool,
    pub variation: bool,
    pub variation_of: Option<Uuid>,
    pub security_stamp: Option<String>,
    pub watermark: Option<String>,
    pub preview: Option<Preview>,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Preview {
    pub previewed_at: NaiveDate,
    #[serde_as(as = "DisplayFromStr")]
    pub source_uri: Uri,
    pub source: String,
}

#[derive(Serialize, Deserialize)]
pub struct CardFace {
    pub artist: Option<String>,
    pub artist_id: Option<String>,
    pub cmc: Option<f32>,
    pub color_indicator: Option<Vec<Colors>>,
    pub colors: Option<Vec<Colors>>,
    pub defense: Option<String>,
    pub flavor_text: Option<String>,
    pub illustration_id: Option<Uuid>,
    pub image_uris: Option<HashMap<String, Value>>,
    pub layout: Option<String>,
    pub loyalty: Option<String>,
    pub mana_cost: String,
    pub name: String,
    pub object: String,
    pub oracle_id: Option<Uuid>,
    pub oracle_text: Option<String>,
    pub power: Option<String>,
    pub printed_name: Option<String>,
    pub printed_type_line: Option<String>,
    pub thoughness: Option<String>,
    pub type_line: Option<String>,
    pub watermark: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Legalities {
    NotLegal,
    Legal,
    Restricted,
    Banned,
}

#[derive(Deserialize, Serialize, Default)]
pub enum Colors {
    #[default]
    #[serde(rename = "W")]
    White,
    #[serde(rename = "U")]
    Blue,
    #[serde(rename = "B")]
    Black,
    #[serde(rename = "G")]
    Green,
    #[serde(rename = "R")]
    Red,
    #[serde(rename = "C")]
    Colorless,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum BorderColor {
    Silver,
    #[default]
    Black,
    White,
    Borderless,
    Yellow,
    Gold,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Finishes {
    Foil,
    #[default]
    NonFoil,
    Etched,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Games {
    #[default]
    Paper,
    Arena,
    MTGO,
    Astral,
    Sega,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ImageStatuses {
    Missing,
    Placeholder,
    Lowres,
    #[default]
    HighresScan,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Rarities {
    #[default]
    Common,
    Uncommon,
    Rare,
    Special,
    Mythic,
    Bonus,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FrameEffects {
    Legendary,
    Miracle,
    Enchantment,
    Draft,
    Devoid,
    Tombstone,
    Colorshifted,
    Inverted,
    SunMoonDFC,
    CompassLandDFC,
    OriginPWDFC,
    MoonEldraziDFC,
    WaxingAndWaningMoonDFC,
    Showcase,
    ExtendedArt,
    Companion,
    Etched,
    Snow,
    Lesson,
    ShatteredGlass,
    ConvertDFC,
    FanDFC,
    UpsideDownDFC,
    Spree,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Frames {
    #[serde(rename = "1993")]
    _1993,
    #[serde(rename = "1997")]
    _1997,
    #[serde(rename = "2003")]
    _2003,
    #[default]
    #[serde(rename = "2015")]
    _2015,
    Future,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct RelatedCardObjects {
    pub id: Uuid,
    pub object: String,
    pub component: String,
    pub name: String,
    pub type_line: String,
    #[serde_as(as = "DisplayFromStr")]
    pub uri: Uri,
}
