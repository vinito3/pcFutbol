use std::collections::HashMap;

pub fn cocatenate_string_from_hashmap_key<'a>(sourceString: & 'a mut String, fields: &HashMap<String,String>) -> & 'a mut String {

    for (key, value) in fields.iter() {
        
        sourceString.push_str(&key);

        if fields.len() > 1 {

            sourceString.push_str(",");
        }
    }
    sourceString

}

pub fn cocatenate_string_from_hashmap_value<'a>(sourceString: & 'a mut String, fields: &HashMap<String,String>) -> & 'a mut String {

    for (key, value) in fields.iter() {
        
        sourceString.push_str(&value);

        if fields.len() > 1 {

            sourceString.push_str(",");
        }
    }
    sourceString

}