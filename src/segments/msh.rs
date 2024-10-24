use crate::segments::SegmentParsingError;
use derivative::Derivative;
use std::fmt;
use std::str::FromStr;

const HEADER: &str = "MSH";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Derivative)]
#[derivative(Default)]
pub struct MSH {
    #[derivative(Default(value = "String::from(\"|\")"))]
    pub msh_1_field_separator: String,
    #[derivative(Default(value = "String::from(\"^~\\\\&\")"))]
    pub msh_2_encoding_characters: String,
    pub msh_3_sending_application: Option<String>,
    pub msh_4_sending_facility: Option<String>,
    pub msh_5_receiving_application: Option<String>,
    pub msh_6_receiving_facility: Option<String>,
    pub msh_7_date_time_of_message: String,
    pub msh_8_security: Option<String>,
    pub msh_9_message_type: String,
    pub msh_10_message_control_id: String,
    pub msh_11_processing_id: String,
    pub msh_12_version_id: String,
    pub msh_13_sequence_number: Option<String>,
    pub msh_14_continuation_pointer: Option<String>,
    pub msh_15_accept_acknowledgment_type: Option<String>,
    pub msh_16_application_acknowledgment_type: Option<String>,
    pub msh_17_country_code: Option<String>,
    pub msh_18_character_set: Option<Vec<String>>,
    pub msh_19_principal_language_of_message: Option<String>,
    pub msh_20_alternate_character_set_handling_scheme: Option<String>,
    pub msh_21_message_profile_identifier: Option<Vec<String>>,
    pub msh_22_sending_responsible_organization: Option<String>,
    pub msh_23_receiving_responsible_organization: Option<String>,
    pub msh_24_sending_network_address: Option<String>,
    pub msh_25_receiving_network_address: Option<String>,
}

fn some_if_not_empty(x: &str) -> Option<String> {
    if x.len() > 0 {
        Some(x.to_string())
    } else {
        None
    }
}

fn empty_if_none(x: &Option<String>) -> String {
    match x.as_ref() {
        None => String::from(""),
        Some(v) => v.clone(),
    }
}

fn optional_vec_to_string(x: &Option<Vec<String>>, delim: &str) -> String {
    match x.as_ref() {
        None => String::from(""),
        Some(v) => v.join(delim),
    }
}

fn split_repeated(repeat_delim: &str, x: &str) -> Option<Vec<String>> {
    let y: Vec<String> = x.split(repeat_delim).map(|y| y.to_string()).collect();
    if y.is_empty() {
        None
    } else {
        Some(y)
    }
}

impl fmt::Display for MSH {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut fields: Vec<String> = Vec::new();
        let component_delim = &self.msh_2_encoding_characters[0..1];
        let repeat_delim = &self.msh_2_encoding_characters[1..2];
        let _escape_char = &self.msh_2_encoding_characters[2..3];
        let _sub_component_delim = &self.msh_2_encoding_characters[3..4];
        fields.push(HEADER.into());
        fields.push(self.msh_2_encoding_characters.clone());
        fields.push(empty_if_none(&self.msh_3_sending_application));
        fields.push(empty_if_none(&self.msh_4_sending_facility));
        fields.push(empty_if_none(&self.msh_5_receiving_application));
        fields.push(empty_if_none(&self.msh_6_receiving_facility));
        fields.push(self.msh_7_date_time_of_message.clone());
        fields.push(empty_if_none(&self.msh_8_security));
        fields.push(self.msh_9_message_type.clone());
        fields.push(self.msh_10_message_control_id.clone());
        fields.push(self.msh_11_processing_id.clone());
        fields.push(self.msh_12_version_id.clone());
        fields.push(empty_if_none(&self.msh_13_sequence_number));
        fields.push(empty_if_none(&self.msh_14_continuation_pointer));
        fields.push(empty_if_none(&self.msh_15_accept_acknowledgment_type));
        fields.push(empty_if_none(&self.msh_16_application_acknowledgment_type));
        fields.push(empty_if_none(&self.msh_17_country_code));
        fields.push(optional_vec_to_string(
            &self.msh_18_character_set,
            repeat_delim,
        ));
        fields.push(empty_if_none(&self.msh_19_principal_language_of_message));
        fields.push(empty_if_none(
            &self.msh_20_alternate_character_set_handling_scheme,
        ));
        fields.push(optional_vec_to_string(
            &self.msh_21_message_profile_identifier,
            component_delim,
        ));
        fields.push(empty_if_none(&self.msh_22_sending_responsible_organization));
        fields.push(empty_if_none(
            &self.msh_23_receiving_responsible_organization,
        ));
        fields.push(empty_if_none(&self.msh_24_sending_network_address));
        fields.push(empty_if_none(&self.msh_25_receiving_network_address));
        write!(f, "{}", fields.join(&self.msh_1_field_separator))
    }
}

impl FromStr for MSH {
    type Err = SegmentParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut msh = MSH::default();
        let delimiter = &s[3..4];
        let mut split_input = s.split(delimiter);

        assert!(split_input.next().unwrap() == HEADER);

        let encoding_chars = split_input.next().unwrap();
        let component_delim = &encoding_chars[0..1];
        let repeat_delim = &encoding_chars[1..2];
        let escape_char = &encoding_chars[2..3];
        let sub_component_delim = &encoding_chars[3..4];

        msh.msh_1_field_separator = delimiter.to_string();
        msh.msh_2_encoding_characters = encoding_chars.to_string();
        split_input
            .next()
            .and_then(|x| {
                msh.msh_3_sending_application = some_if_not_empty(x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_4_sending_facility = some_if_not_empty(x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_5_receiving_application = some_if_not_empty(x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_6_receiving_facility = some_if_not_empty(x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_7_date_time_of_message = x.to_string();
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_8_security = some_if_not_empty(x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_9_message_type = x.to_string();
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_10_message_control_id = x.to_string();
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_11_processing_id = x.to_string();
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_12_version_id = x.to_string();
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_13_sequence_number = some_if_not_empty(x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_14_continuation_pointer = some_if_not_empty(x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_15_accept_acknowledgment_type = some_if_not_empty(x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_16_application_acknowledgment_type = some_if_not_empty(x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_17_country_code = some_if_not_empty(x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_18_character_set = split_repeated(repeat_delim, x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_19_principal_language_of_message = some_if_not_empty(x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_20_alternate_character_set_handling_scheme = some_if_not_empty(x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_21_message_profile_identifier = split_repeated(repeat_delim, x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_22_sending_responsible_organization = some_if_not_empty(x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_23_receiving_responsible_organization = some_if_not_empty(x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_24_sending_network_address = some_if_not_empty(x);
                split_input.next()
            })
            .and_then(|x| {
                msh.msh_25_receiving_network_address = some_if_not_empty(x);
                split_input.next()
            });
        Ok(msh)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_def() {
        let s = MSH::default();
        println!("{}", s);
        assert_eq!("MSH|^~\\&|||||||||||||||||||||||", s.to_string());
    }
}
