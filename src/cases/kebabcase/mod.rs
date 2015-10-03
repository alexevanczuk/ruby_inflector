use std::ascii::*;
use regex::Regex;

use cases::classcase::is_class_case;
use cases::snakecase::is_snake_case;
use cases::camelcase::is_camel_case;
use cases::sentencecase::is_sentence_case;
use cases::lowercase::to_lower_case;

pub fn is_kebab_case<'a>(test_string: String) -> bool{
    let kebab_matcher = Regex::new(r"(?:[^_]?=^|[-])([a-z]+)").unwrap();
    let upcase_matcher = Regex::new(r"[A-Z]").unwrap();
    let mut is_kebab_case = false;
    if kebab_matcher.is_match(test_string.as_ref())
        && !upcase_matcher.is_match(test_string.as_ref()) {
            is_kebab_case = true;
        }
    return is_kebab_case;
}

pub fn to_kebab_case<'a>(non_kebab_case_string: String) -> String {
    let mut kebab_string: String = non_kebab_case_string.clone();
    if is_camel_case(non_kebab_case_string.clone()) {
        kebab_string = to_kebab_from_camel(non_kebab_case_string.clone());
    }else if is_snake_case(non_kebab_case_string.clone()) {
        kebab_string = to_kebab_from_snake(non_kebab_case_string.clone());
    }else if is_sentence_case(non_kebab_case_string.clone()) {
        kebab_string = to_kebab_from_sentence(non_kebab_case_string.clone());
    }else if is_class_case(non_kebab_case_string.clone()) {
        kebab_string = to_kebab_from_camel(non_kebab_case_string.clone());
    }
    return kebab_string;
}

    fn to_kebab_from_camel<'a>(non_kebab_case_string: String) -> String {
        let mut result:String = "".to_string();
        let mut first_character: bool = true;
        for character in non_kebab_case_string.chars() {
            if character == character.to_ascii_uppercase() && !first_character {
                result = format!("{}-{}", result, character.to_ascii_lowercase());
            } else {
                result = format!("{}{}", result, character.to_ascii_lowercase());
                first_character = false;
            }
        }
        return result
    }

    fn to_kebab_from_snake<'a>(non_kebab_case_string: String) -> String {
        return to_lower_case(non_kebab_case_string.replace("_", "-"));
    }

    fn to_kebab_from_sentence<'a>(non_kebab_case_string: String) -> String {
        return to_lower_case(non_kebab_case_string.replace(" ", "-"));
    }

#[cfg(test)]
mod kebabcase_test;
