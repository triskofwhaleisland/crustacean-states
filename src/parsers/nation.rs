use crate::pretty_name;
use crate::shards::world_shards::{
    AccountCategory, BulletinCategory, DispatchCategory, FactbookCategory, MetaCategory,
};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};

use std::fmt::{Debug, Formatter};
use std::num::{NonZeroU16, NonZeroU32, NonZeroU64};
use std::str::FromStr;

#[derive(Debug, Deserialize)]
/// The Rust representation of a nation, as interpreted from a response to a request.
pub struct Nation {
    // default shards from ?nation=
    #[serde(rename = "$value", deserialize_with = "handle_name")]
    /// The name of the nation.
    ///
    /// Note: this is the *only* field that is ever guaranteed to be filled in.
    /// If the [`PublicNationShard::Name`] field was not requested,
    /// this is obtained from the results of [`pretty_name`], which can
    ///
    /// [`PublicNationShard::Name`]: crate::shards::public_nation_shards::PublicNationShard::Name
    pub name: Option<String>,
    #[serde(rename = "TYPE")]
    pub kind: Option<String>,
    #[serde(rename = "FULLNAME")]
    pub full_name: Option<String>,
    #[serde(rename = "MOTTO")]
    pub motto: Option<String>,
    #[serde(rename = "CATEGORY")]
    pub category: Option<String>,
    #[serde(rename = "UNSTATUS", deserialize_with = "handle_wa_status")]
    pub wa_status: Option<WAStatus>,
    #[serde(rename = "ENDORSEMENTS", deserialize_with = "unwrap_endorsement_list")]
    pub endorsements: Option<Vec<String>>,
    #[serde(rename = "ISSUES_ANSWERED")]
    pub issues_answered: Option<u32>,
    #[serde(rename = "FREEDOM")]
    pub freedom: Option<Freedoms>,
    #[serde(rename = "REGION")]
    pub region: Option<String>,
    #[serde(rename = "POPULATION")]
    pub population: Option<u32>,
    #[serde(rename = "TAX")]
    pub tax: Option<f32>,
    #[serde(rename = "ANIMAL")]
    pub animal: Option<String>,
    #[serde(rename = "CURRENCY")]
    pub currency: Option<String>,
    #[serde(rename = "DEMONYM")]
    pub demonym: Option<String>,
    #[serde(rename = "DEMONYM2")]
    pub demonym2: Option<String>,
    #[serde(rename = "DEMONYM2PLURAL")]
    pub demonym2_plural: Option<String>,
    #[serde(rename = "FLAG")]
    pub flag: Option<String>,
    #[serde(rename = "MAJORINDUSTRY")]
    pub major_industry: Option<String>,
    #[serde(rename = "GOVTPRIORITY")]
    pub government_priority: Option<String>,
    #[serde(rename = "GOVT")]
    pub government: Option<Government>,
    #[serde(rename = "FOUNDED")]
    pub founded: Option<String>,
    #[serde(rename = "FIRSTLOGIN")]
    pub first_login: Option<u64>,
    #[serde(rename = "LASTLOGIN")]
    pub last_login: Option<u64>,
    #[serde(rename = "LASTACTIVITY")]
    pub last_activity: Option<String>,
    #[serde(rename = "INFLUENCE")]
    pub influence: Option<String>,
    #[serde(rename = "FREEDOMSCORES")]
    pub freedom_scores: Option<FreedomScores>,
    #[serde(rename = "PUBLICSECTOR")]
    pub public_sector: Option<f32>,
    #[serde(rename = "DEATHS")]
    pub deaths: Option<Deaths>,
    #[serde(rename = "LEADER")]
    pub leader: Option<String>,
    #[serde(rename = "CAPITAL")]
    pub capital: Option<String>,
    #[serde(rename = "RELIGION")]
    pub religion: Option<String>,
    #[serde(rename = "FACTBOOKS")]
    pub factbooks: Option<u16>,
    #[serde(rename = "DISPATCHES")]
    pub dispatches: Option<u16>,
    #[serde(rename = "DBID")]
    pub dbid: Option<u32>,
    // END default
    #[serde(rename = "ADMIRABLE")]
    pub admirable: Option<String>,
    #[serde(rename = "ADMIRABLES")]
    pub admirables: Option<Vec<String>>,
    #[serde(rename = "ANIMALTRAIT")]
    pub animal_trait: Option<String>,
    #[serde(rename = "BANNER")]
    pub banner: Option<String>, // TODO: Option<BannerID>
    #[serde(rename = "BANNERS")]
    pub banners: Option<Vec<String>>, // TODO: Option<Vec<BannerID>>
    #[serde(rename = "CENSUS")]
    pub census: Option<Vec<CensusData>>,
    #[serde(rename = "CRIME")]
    pub crime: Option<String>,
    #[serde(rename = "DISPATCHLIST")]
    pub dispatch_list: Option<DispatchList>,
    // #[serde(rename = "FACTBOOKLIST")]
    // factbook_list: Option<Vec<Factbook>>,
    #[serde(rename = "FOUNDEDTIME")]
    pub founded_time: Option<u64>,
    // #[serde(rename = "GAVOTE")]
    // ga_vote: Option<WAVote>,
    #[serde(rename = "GDP")]
    pub gdp: Option<u64>,
    #[serde(rename = "GOVTDESC")]
    pub govt_desc: Option<String>,
    // happenings: Option<Vec<Event>>,
    // income: Option<u32>,
    industry_desc: Option<String>,
    legislation: Option<String>,
    notable: Option<String>,
    notables: Option<Vec<String>>,
    // policies: Option<Vec<Policy>>,
    poorest: Option<u32>,
    regional_census: Option<NonZeroU16>,
    richest: Option<u32>,
    // sc_vote: Option<WAVote>,
    // sectors: Option<GovernmentSectors>,
    sensibilities: Option<String>,
    tg_can_recruit: Option<bool>,
    tg_can_campaign: Option<bool>,
    world_census: Option<NonZeroU32>,
}

#[derive(Debug)]
pub enum WAStatus {
    Delegate,
    Member,
    NonMember,
}

#[derive(Debug, Deserialize)]
pub struct Government {
    #[serde(rename = "ADMINISTRATION")]
    pub administration: f32,
    #[serde(rename = "DEFENCE")]
    pub defence: f32,
    #[serde(rename = "EDUCATION")]
    pub education: f32,
    #[serde(rename = "ENVIRONMENT")]
    pub environment: f32,
    #[serde(rename = "HEALTHCARE")]
    pub healthcare: f32,
    #[serde(rename = "COMMERCE")]
    pub commerce: f32,
    #[serde(rename = "INTERNATIONALAID")]
    pub international_aid: f32,
    #[serde(rename = "LAWANDORDER")]
    pub law_and_order: f32,
    #[serde(rename = "PUBLICTRANSPORT")]
    pub public_transport: f32,
    #[serde(rename = "SOCIALEQUALITY")]
    pub social_equality: f32,
    #[serde(rename = "SPIRITUALITY")]
    pub spirituality: f32,
    #[serde(rename = "WELFARE")]
    pub welfare: f32,
}

#[derive(Debug, Deserialize)]
pub struct Freedoms {
    #[serde(rename = "CIVILRIGHTS")]
    pub civil_rights: String,
    #[serde(rename = "ECONOMY")]
    pub economy: String,
    #[serde(rename = "POLITICALFREEDOM")]
    pub political_freedom: String,
}

#[derive(Debug, Deserialize)]
pub struct FreedomScores {
    #[serde(rename = "CIVILRIGHTS")]
    pub civil_rights: u8,
    #[serde(rename = "ECONOMY")]
    pub economy: u8,
    #[serde(rename = "POLITICALFREEDOM")]
    pub political_freedom: u8,
}

#[derive(Debug, Deserialize)]
pub struct Deaths {
    #[serde(rename = "CAUSE")]
    pub causes: Vec<Cause>,
}

#[derive(Debug, Deserialize)]
pub struct Cause {
    #[serde(rename = "@type")]
    pub kind: String,
    #[serde(rename = "$value")]
    pub frequency: f32,
}

#[derive(Debug, Deserialize)]
pub struct Admirables {
    #[serde(rename = "ADMIRABLES", deserialize_with = "unwrap_list")]
    pub traits: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Census {
    #[serde(rename = "SCALE")]
    pub data: Vec<CensusData>,
}

#[derive(Debug, Deserialize)]
pub struct CensusData {
    #[serde(rename = "@id")]
    pub id: u8,
    #[serde(rename = "SCORE")]
    pub score: Option<f64>,
    #[serde(rename = "RANK")]
    pub world_rank: Option<u32>,
    #[serde(rename = "RRANK")]
    pub region_rank: Option<u32>,
    #[serde(rename = "PRANK")]
    pub percent_world_rank: Option<u8>,
    #[serde(rename = "PRRANK")]
    pub percent_region_rank: Option<u8>,
    #[serde(rename = "TIMESTAMP")]
    pub timestamp: Option<NonZeroU64>,
}

#[derive(Debug, Deserialize)]
pub struct DispatchList {
    #[serde(rename = "DISPATCH")]
    pub dispatches: Vec<Dispatch>,
}

#[derive(Debug)]
pub struct Dispatch {
    pub id: String,
    pub title: String,
    pub author: String,
    pub category: DispatchCategory,
    pub created: u64,
    pub edited: u64,
    pub views: u32,
    pub score: u32,
}

impl<'de> Deserialize<'de> for Dispatch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug)]
        enum Field {
            Id,
            Title,
            Author,
            Category,
            Subcategory,
            Created,
            Edited,
            Views,
            Score,
            Value,
        }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                        eprintln!("Calling Field `expecting`...");
                        formatter.write_str(
                            "`@id`, `title`, `author`, `category`, `subcategory`, `created`, `edited`, `views`, `score`"
                        )
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: Error,
                    {
                        eprintln!("Visiting {value}...");
                        match value {
                            "@id" => Ok(Field::Id),
                            "TITLE" => Ok(Field::Title),
                            "AUTHOR" => Ok(Field::Author),
                            "CATEGORY" => Ok(Field::Category),
                            "SUBCATEGORY" => Ok(Field::Subcategory),
                            "CREATED" => Ok(Field::Created),
                            "EDITED" => Ok(Field::Edited),
                            "VIEWS" => Ok(Field::Views),
                            "SCORE" => Ok(Field::Score),
                            "$value" => Ok(Field::Value),
                            _ => Err(Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                eprintln!("Deserializing the identifier");
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct DispatchVisitor;

        impl<'de> Visitor<'de> for DispatchVisitor {
            type Value = Dispatch;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                eprintln!("Calling Visitor `expecting`...");
                formatter.write_str("struct Dispatch")
            }

            // fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            // where
            //     A: SeqAccess<'de>,
            // {
            //     eprintln!("Visiting sequence...");
            //     let id = seq.next_element()?.ok_or_else(|| Error::invalid_length(0, &self))?;
            //     let title = seq
            //         .next_element()?
            //         .ok_or_else(|| Error::invalid_length(1, &self))?;
            //     let author = seq
            //         .next_element()?
            //         .ok_or_else(|| Error::invalid_length(2, &self))?;
            //     let main_category = seq
            //         .next_element()?
            //         .ok_or_else(|| Error::invalid_length(3, &self))?;
            //     let sub_category = seq
            //         .next_element()?
            //         .ok_or_else(|| Error::invalid_length(4, &self))?;
            //     let created = seq
            //         .next_element()?
            //         .ok_or_else(|| Error::invalid_length(5, &self))?;
            //     let edited = seq
            //         .next_element()?
            //         .ok_or_else(|| Error::invalid_length(6, &self))?;
            //     let views = seq
            //         .next_element()?
            //         .ok_or_else(|| Error::invalid_length(7, &self))?;
            //     let score = seq
            //         .next_element()?
            //         .ok_or_else(|| Error::invalid_length(8, &self))?;
            //
            //     let category = main_and_sub_to_rust(main_category, sub_category);
            //
            //     Ok(Self::Value {
            //         id,
            //         title,
            //         author,
            //         category,
            //         created,
            //         edited,
            //         views,
            //         score,
            //     })
            // }

            fn visit_map<V>(self, mut map: V) -> Result<Dispatch, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut id: Option<String> = None;
                let mut title: Option<String> = None;
                let mut author: Option<String> = None;
                let mut category: Option<String> = None;
                let mut subcategory: Option<String> = None;
                let mut created: Option<String> = None;
                let mut edited: Option<String> = None;
                let mut views: Option<String> = None;
                let mut score: Option<String> = None;
                let mut value: Option<String> = None;
                while let Some(key) = map.next_key()? {
                    eprintln!("Handling {key:?}");
                    match key {
                        Field::Id => handle_field("@id", &mut id, &mut map)?,
                        Field::Title => handle_field("TITLE", &mut title, &mut map)?,
                        Field::Author => handle_field("AUTHOR", &mut author, &mut map)?,
                        Field::Category => handle_field("CATEGORY", &mut category, &mut map)?,
                        Field::Subcategory => {
                            handle_field("SUBCATEGORY", &mut subcategory, &mut map)?
                        }
                        Field::Created => handle_field("CREATED", &mut created, &mut map)?,
                        Field::Edited => handle_field("EDITED", &mut edited, &mut map)?,
                        Field::Views => handle_field("VIEWS", &mut views, &mut map)?,
                        Field::Score => handle_field("SCORE", &mut score, &mut map)?,
                        Field::Value => handle_field("$value", &mut value, &mut map)?,
                    }
                }
                eprintln!("Checking to make sure all fields are filled in");
                let id = id.ok_or_else(|| Error::missing_field("@id"))?;
                let title = title.ok_or_else(|| Error::missing_field("title"))?;
                let author = author.ok_or_else(|| Error::missing_field("author"))?;
                let category = category.ok_or_else(|| Error::missing_field("category"))?;
                let subcategory = subcategory.ok_or_else(|| Error::missing_field("subcategory"))?;
                let created = created.ok_or_else(|| Error::missing_field("created"))?;
                let edited = edited.ok_or_else(|| Error::missing_field("edited"))?;
                let views = views.ok_or_else(|| Error::missing_field("views"))?;
                let score = score.ok_or_else(|| Error::missing_field("score"))?;

                eprintln!("Converting fields to proper types");
                let category = main_and_sub_to_rust(category, subcategory).unwrap();

                let created = u64::from_str(&created).unwrap();
                let edited = u64::from_str(&edited).unwrap();
                let views = u32::from_str(&views).unwrap();
                let score = u32::from_str(&score).unwrap();

                eprintln!("I'm gonna send it!!!");
                Ok(Self::Value {
                    id,
                    title,
                    author,
                    category,
                    created,
                    edited,
                    views,
                    score,
                })
            }
        }

        const FIELDS: &[&str] = &[
            "@id",
            "TITLE",
            "AUTHOR",
            "CATEGORY",
            "SUBCATEGORY",
            "CREATED",
            "EDITED",
            "VIEWS",
            "SCORE",
            "$value",
        ];
        eprintln!("About to deserialize!");
        deserializer.deserialize_struct("Dispatch", FIELDS, DispatchVisitor)
    }
}

fn handle_name<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    enum AnyName {
        #[serde(rename = "NAME")]
        FromNameTag {
            #[serde(rename = "$value")]
            name: String,
        },
        #[serde(rename = "@id")]
        FromIdAttr {
            #[serde(rename = "$value")]
            name: String,
        },

        #[serde(other)]
        Neither,
    }
    Ok(match AnyName::deserialize(deserializer)? {
        AnyName::FromIdAttr { name } => Some(pretty_name(name)),
        AnyName::FromNameTag { name } => Some(name),
        AnyName::Neither => None,
    })
}

fn handle_wa_status<'de, D>(deserializer: D) -> Result<Option<WAStatus>, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.as_str() {
        "WA Delegate" => Ok(Some(WAStatus::Delegate)),
        "WA Member" => Ok(Some(WAStatus::Member)),
        "Non-member" => Ok(Some(WAStatus::NonMember)),
        other => Err(Error::custom(format!("invalid status for WA: {other}"))),
    }
}

fn unwrap_endorsement_list<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Some(
        String::deserialize(deserializer)?
            .split(|c| c == ',')
            .map(pretty_name)
            .collect(),
    ))
}

fn unwrap_list<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct List {
        inner: Option<Vec<String>>,
    }
    Ok(List::deserialize(deserializer)?.inner)
}

fn handle_field<'de, 'a, T, D>(
    field: &'static str,
    value: &mut Option<T>,
    map: &mut D,
) -> Result<(), D::Error>
where
    T: serde::de::DeserializeOwned + Debug,
    D: MapAccess<'de>,
{
    eprintln!("Handling {field}!");
    if value.is_some() {
        eprintln!("Uh-oh, duplicate {field}");
        let problem = Error::duplicate_field(field);
        return Err(problem);
    } else {
        let v = map.next_value()?;
        eprintln!("{field} = {v:?}");
        *value = Some(v);
    }
    Ok(())
}

fn main_and_sub_to_rust(main_category: String, sub_category: String) -> Option<DispatchCategory> {
    match main_category.as_str() {
        // TODO allow for errors in parsing category
        "Factbook" => Some(DispatchCategory::Factbook(match sub_category.as_str() {
            "Overview" => Some(FactbookCategory::Overview),
            "History" => Some(FactbookCategory::History),
            "Geography" => Some(FactbookCategory::Geography),
            "Culture" => Some(FactbookCategory::Culture),
            "Politics" => Some(FactbookCategory::Politics),
            "Legislation" => Some(FactbookCategory::Legislation),
            "Religion" => Some(FactbookCategory::Religion),
            "Military" => Some(FactbookCategory::Military),
            "Economy" => Some(FactbookCategory::Economy),
            "International" => Some(FactbookCategory::International),
            "Trivia" => Some(FactbookCategory::Trivia),
            "Miscellaneous" => Some(FactbookCategory::Miscellaneous),
            _ => None,
        })),
        "Bulletin" => Some(DispatchCategory::Bulletin(match sub_category.as_str() {
            "Policy" => Some(BulletinCategory::Policy),
            "News" => Some(BulletinCategory::News),
            "Opinion" => Some(BulletinCategory::Opinion),
            "Campaign" => Some(BulletinCategory::Campaign),
            _ => None,
        })),
        "Account" => Some(DispatchCategory::Account(match sub_category.as_str() {
            "Military" => Some(AccountCategory::Military),
            "Trade" => Some(AccountCategory::Trade),
            "Sport" => Some(AccountCategory::Sport),
            "Drama" => Some(AccountCategory::Drama),
            "Diplomacy" => Some(AccountCategory::Diplomacy),
            "Science" => Some(AccountCategory::Science),
            "Culture" => Some(AccountCategory::Culture),
            "Other" => Some(AccountCategory::Other),
            _ => None,
        })),
        "Meta" => Some(DispatchCategory::Meta(match sub_category.as_str() {
            "Gameplay" => Some(MetaCategory::Gameplay),
            "Reference" => Some(MetaCategory::Reference),
            _ => None,
        })),
        _ => None,
    }
}
