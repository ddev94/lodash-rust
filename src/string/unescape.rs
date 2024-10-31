use std::collections::HashMap;

pub fn create_html_entities_map() -> HashMap<String, String> {
    let mut entities = HashMap::new();
    entities.insert("&amp;".to_string(), "&".to_string());
    entities.insert("&lt;".to_string(), "<".to_string());
    entities.insert("&gt;".to_string(), ">".to_string());
    entities.insert("&quot;".to_string(), "\"".to_string());
    entities.insert("&#39;".to_string(), "'".to_string());
    entities.insert("&apos;".to_string(), "'".to_string());
    entities
}

pub fn unescape(input: &str) -> String {
    let entities = create_html_entities_map();
    let mut result = input.to_string();
    
    for (entity, character) in entities.iter() {
        result = result.replace(entity, character);
    }
    
    result
}