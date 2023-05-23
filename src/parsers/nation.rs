use crate::pretty_name;
use serde::de::Error;
use serde::{Deserialize, Deserializer};

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
    pub name: String,
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

fn handle_name<'de, D>(deserializer: D) -> Result<String, D::Error>
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
    }
    Ok(match AnyName::deserialize(deserializer)? {
        AnyName::FromIdAttr { name } => pretty_name(name),
        AnyName::FromNameTag { name } => name,
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
