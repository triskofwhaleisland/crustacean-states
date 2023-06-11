// use std::fmt::{Debug, Formatter};
// use std::str::FromStr;
// use serde::{Deserialize, Deserializer};
// use serde::de::{Error, MapAccess, Visitor};
// use crate::parsers::nation::Dispatch;
// use crate::shards::world_shards::{AccountCategory, BulletinCategory, DispatchCategory, FactbookCategory, MetaCategory};
// 
// impl<'de> Deserialize<'de> for Dispatch {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//         where
//             D: Deserializer<'de>,
//     {
//         #[derive(Debug)]
//         enum Field {
//             Id,
//             Title,
//             Author,
//             Category,
//             Subcategory,
//             Created,
//             Edited,
//             Views,
//             Score,
//             Value,
//         }
//         impl<'de> Deserialize<'de> for Field {
//             fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//                 where
//                     D: Deserializer<'de>,
//             {
//                 struct FieldVisitor;
// 
//                 impl<'de> Visitor<'de> for FieldVisitor {
//                     type Value = Field;
// 
//                     fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
//                         eprintln!("Calling Field `expecting`...");
//                         formatter.write_str(
//                             "`@id`, `title`, `author`, `category`, `subcategory`, `created`, `edited`, `views`, `score`"
//                         )
//                     }
// 
//                     fn visit_str<E>(self, value: &str) -> Result<Field, E>
//                         where
//                             E: Error,
//                     {
//                         eprintln!("Visiting {value}...");
//                         match value {
//                             "@id" => Ok(Field::Id),
//                             "TITLE" => Ok(Field::Title),
//                             "AUTHOR" => Ok(Field::Author),
//                             "CATEGORY" => Ok(Field::Category),
//                             "SUBCATEGORY" => Ok(Field::Subcategory),
//                             "CREATED" => Ok(Field::Created),
//                             "EDITED" => Ok(Field::Edited),
//                             "VIEWS" => Ok(Field::Views),
//                             "SCORE" => Ok(Field::Score),
//                             "$value" => Ok(Field::Value),
//                             _ => Err(Error::unknown_field(value, FIELDS)),
//                         }
//                     }
//                 }
//                 eprintln!("Deserializing the identifier");
//                 deserializer.deserialize_identifier(FieldVisitor)
//             }
//         }
// 
//         struct DispatchVisitor;
// 
//         impl<'de> Visitor<'de> for DispatchVisitor {
//             type Value = Dispatch;
// 
//             fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
//                 eprintln!("Calling Visitor `expecting`...");
//                 formatter.write_str("struct Dispatch")
//             }
// 
//             // fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
//             // where
//             //     A: SeqAccess<'de>,
//             // {
//             //     eprintln!("Visiting sequence...");
//             //     let id = seq.next_element()?.ok_or_else(|| Error::invalid_length(0, &self))?;
//             //     let title = seq
//             //         .next_element()?
//             //         .ok_or_else(|| Error::invalid_length(1, &self))?;
//             //     let author = seq
//             //         .next_element()?
//             //         .ok_or_else(|| Error::invalid_length(2, &self))?;
//             //     let main_category = seq
//             //         .next_element()?
//             //         .ok_or_else(|| Error::invalid_length(3, &self))?;
//             //     let sub_category = seq
//             //         .next_element()?
//             //         .ok_or_else(|| Error::invalid_length(4, &self))?;
//             //     let created = seq
//             //         .next_element()?
//             //         .ok_or_else(|| Error::invalid_length(5, &self))?;
//             //     let edited = seq
//             //         .next_element()?
//             //         .ok_or_else(|| Error::invalid_length(6, &self))?;
//             //     let views = seq
//             //         .next_element()?
//             //         .ok_or_else(|| Error::invalid_length(7, &self))?;
//             //     let score = seq
//             //         .next_element()?
//             //         .ok_or_else(|| Error::invalid_length(8, &self))?;
//             //
//             //     let category = main_and_sub_to_rust(main_category, sub_category);
//             //
//             //     Ok(Self::Value {
//             //         id,
//             //         title,
//             //         author,
//             //         category,
//             //         created,
//             //         edited,
//             //         views,
//             //         score,
//             //     })
//             // }
// 
//             fn visit_map<V>(self, mut map: V) -> Result<Dispatch, V::Error>
//                 where
//                     V: MapAccess<'de>,
//             {
//                 let mut id: Option<String> = None;
//                 let mut title: Option<String> = None;
//                 let mut author: Option<String> = None;
//                 let mut category: Option<String> = None;
//                 let mut subcategory: Option<String> = None;
//                 let mut created: Option<String> = None;
//                 let mut edited: Option<String> = None;
//                 let mut views: Option<String> = None;
//                 let mut score: Option<String> = None;
//                 let mut value: Option<String> = None;
//                 while let Some(key) = map.next_key()? {
//                     eprintln!("Handling {key:?}");
//                     match key {
//                         Field::Id => handle_field("@id", &mut id, &mut map)?,
//                         Field::Title => handle_field("TITLE", &mut title, &mut map)?,
//                         Field::Author => handle_field("AUTHOR", &mut author, &mut map)?,
//                         Field::Category => handle_field("CATEGORY", &mut category, &mut map)?,
//                         Field::Subcategory => {
//                             handle_field("SUBCATEGORY", &mut subcategory, &mut map)?
//                         }
//                         Field::Created => handle_field("CREATED", &mut created, &mut map)?,
//                         Field::Edited => handle_field("EDITED", &mut edited, &mut map)?,
//                         Field::Views => handle_field("VIEWS", &mut views, &mut map)?,
//                         Field::Score => handle_field("SCORE", &mut score, &mut map)?,
//                         Field::Value => handle_field("$value", &mut value, &mut map)?,
//                     }
//                 }
//                 eprintln!("Checking to make sure all fields are filled in");
//                 let id = id.ok_or_else(|| Error::missing_field("@id"))?;
//                 let title = title.ok_or_else(|| Error::missing_field("title"))?;
//                 let author = author.ok_or_else(|| Error::missing_field("author"))?;
//                 let category = category.ok_or_else(|| Error::missing_field("category"))?;
//                 let subcategory = subcategory.ok_or_else(|| Error::missing_field("subcategory"))?;
//                 let created = created.ok_or_else(|| Error::missing_field("created"))?;
//                 let edited = edited.ok_or_else(|| Error::missing_field("edited"))?;
//                 let views = views.ok_or_else(|| Error::missing_field("views"))?;
//                 let score = score.ok_or_else(|| Error::missing_field("score"))?;
// 
//                 eprintln!("Converting fields to proper types");
//                 let category = main_and_sub_to_rust(category, subcategory).unwrap();
// 
//                 let created = u64::from_str(&created).unwrap();
//                 let edited = u64::from_str(&edited).unwrap();
//                 let views = u32::from_str(&views).unwrap();
//                 let score = u32::from_str(&score).unwrap();
// 
//                 eprintln!("I'm gonna send it!!!");
//                 Ok(Self::Value {
//                     id,
//                     title,
//                     author,
//                     category,
//                     created,
//                     edited,
//                     views,
//                     score,
//                 })
//             }
//         }
// 
//         const FIELDS: &[&str] = &[
//             "@id",
//             "TITLE",
//             "AUTHOR",
//             "CATEGORY",
//             "SUBCATEGORY",
//             "CREATED",
//             "EDITED",
//             "VIEWS",
//             "SCORE",
//             "$value",
//         ];
//         eprintln!("About to deserialize!");
//         deserializer.deserialize_struct("Dispatch", FIELDS, DispatchVisitor)
//     }
// }
// 
// fn handle_field<'de, 'a, T, D>(
//     field: &'static str,
//     value: &mut Option<T>,
//     map: &mut D,
// ) -> Result<(), D::Error>
//     where
//         T: serde::de::DeserializeOwned + Debug,
//         D: MapAccess<'de>,
// {
//     eprintln!("Handling {field}!");
//     if value.is_some() {
//         eprintln!("Uh-oh, duplicate {field}");
//         let problem = Error::duplicate_field(field);
//         return Err(problem);
//     } else {
//         let v = map.next_value()?;
//         eprintln!("{field} = {v:?}");
//         *value = Some(v);
//     }
//     Ok(())
// }
// 
// fn main_and_sub_to_rust(main_category: String, sub_category: String) -> Option<DispatchCategory> {
//     match main_category.as_str() {
//         // TODO allow for errors in parsing category
//         "Factbook" => Some(DispatchCategory::Factbook(match sub_category.as_str() {
//             "Overview" => Some(FactbookCategory::Overview),
//             "History" => Some(FactbookCategory::History),
//             "Geography" => Some(FactbookCategory::Geography),
//             "Culture" => Some(FactbookCategory::Culture),
//             "Politics" => Some(FactbookCategory::Politics),
//             "Legislation" => Some(FactbookCategory::Legislation),
//             "Religion" => Some(FactbookCategory::Religion),
//             "Military" => Some(FactbookCategory::Military),
//             "Economy" => Some(FactbookCategory::Economy),
//             "International" => Some(FactbookCategory::International),
//             "Trivia" => Some(FactbookCategory::Trivia),
//             "Miscellaneous" => Some(FactbookCategory::Miscellaneous),
//             _ => None,
//         })),
//         "Bulletin" => Some(DispatchCategory::Bulletin(match sub_category.as_str() {
//             "Policy" => Some(BulletinCategory::Policy),
//             "News" => Some(BulletinCategory::News),
//             "Opinion" => Some(BulletinCategory::Opinion),
//             "Campaign" => Some(BulletinCategory::Campaign),
//             _ => None,
//         })),
//         "Account" => Some(DispatchCategory::Account(match sub_category.as_str() {
//             "Military" => Some(AccountCategory::Military),
//             "Trade" => Some(AccountCategory::Trade),
//             "Sport" => Some(AccountCategory::Sport),
//             "Drama" => Some(AccountCategory::Drama),
//             "Diplomacy" => Some(AccountCategory::Diplomacy),
//             "Science" => Some(AccountCategory::Science),
//             "Culture" => Some(AccountCategory::Culture),
//             "Other" => Some(AccountCategory::Other),
//             _ => None,
//         })),
//         "Meta" => Some(DispatchCategory::Meta(match sub_category.as_str() {
//             "Gameplay" => Some(MetaCategory::Gameplay),
//             "Reference" => Some(MetaCategory::Reference),
//             _ => None,
//         })),
//         _ => None,
//     }
// }
