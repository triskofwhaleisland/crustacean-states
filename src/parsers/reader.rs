use std::collections::HashMap;
use quick_xml::events::Event;
use quick_xml::Reader;

#[derive(Debug, Default)]
struct Thing {
    thing1: Option<String>,
}
#[derive(Debug)]
struct UnexpectedFields(HashMap<String, String>);

fn read_nation(xml: &str) -> (Thing, UnexpectedFields) {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut thing = Thing::default();
    let mut unexpected_fields = UnexpectedFields(HashMap::new());

    while let Ok(event) = reader.read_event_into(&mut buf) {
        match event {
            Event::Start(ref e) => {
                let field = String::from_utf8_lossy(e.name().as_ref()).to_string();
                match field.as_str() {
                    "a" => {
                        thing.thing1 = read_text(&mut reader, &mut buf);
                    }
                    _ => {
                        let value = read_text(&mut reader, &mut buf).unwrap_or_default();
                        unexpected_fields.0.insert(field, value);
                    }
                }
            }
            Event::Eof => break,
            _ => {},
        }
    }

    (thing, unexpected_fields)
}


fn read_text(reader: &mut Reader<&[u8]>, buf: &mut Vec<u8>) -> Option<String> {
    if let Ok(Event::Text(e)) = reader.read_event_into(buf) {
        Some(e.unescape().unwrap_or_default().to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::reader::read_nation;

    #[test]
    fn test1() {
        let xml_data = r#"
        <a>Hello!</a>
        <b>I'm also here!</b>
    "#;

        let (known_field, unexpected_fields) = read_nation(xml_data);

        println!("Known Field: {:?}", known_field);
        println!("Unexpected Fields: {:?}", unexpected_fields);
    }
}