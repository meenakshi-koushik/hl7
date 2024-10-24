use derivative::Derivative;
use std::fmt;

use crate::segments::{
    empty_if_none, optional_vec_to_string, some_if_not_empty, split_repeated, SegmentParsingError,
    DEFAULT_FIELD_SEPARATOR,
};
const HEADER: &str = "PID";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Derivative)]
#[derivative(Default)]
pub struct PID {
    pub pid1_set_id: Option<String>,
    pub pid2_patient_id: Option<String>,
    pub pid3_patient_identifier_list: Vec<String>,
    pub pid4_alternate_patient_id: Option<String>,
    pub pid5_patient_name: Vec<String>,
    pub pid6_mothers_maiden_name: Option<Vec<String>>,
    pub pid7_date_time_of_birth: Option<String>,
    pub pid8_administrative_sex: Option<String>,
    pub pid9_patient_alias: Option<String>,
    pub pid10_race: Option<Vec<String>>,
    pub pid11_patient_address: Option<Vec<String>>,
    pub pid12_county_code: Option<String>,
    pub pid13_phone_number_home: Option<Vec<String>>,
    pub pid14_phone_number_business: Option<Vec<String>>,
    pub pid15_primary_language: Option<String>,
    pub pid16_marital_status: Option<String>,
    pub pid17_religion: Option<String>,
    pub pid18_patient_account_number: Option<String>,
    pub pid19_ssn_number_patient: Option<String>,
    pub pid20_drivers_license_number_patient: Option<String>,
    pub pid21_mothers_identifier: Option<Vec<String>>,
    pub pid22_ethnic_group: Option<Vec<String>>,
    pub pid23_birth_place: Option<String>,
    pub pid24_multiple_birth_indicator: Option<String>,
    pub pid25_birth_order: Option<String>,
    pub pid26_citizenship: Option<Vec<String>>,
    pub pid27_veterans_military_status: Option<String>,
    pub pid28_nationality: Option<String>,
    pub pid29_patient_death_date_and_time: Option<String>,
    pub pid30_patient_death_indicator: Option<String>,
    pub pid31_identity_unknown_indicator: Option<String>,
    pub pid32_identity_reliability_code: Option<Vec<String>>,
    pub pid33_last_update_date_time: Option<String>,
    pub pid34_last_update_facility: Option<String>,
    pub pid35_taxonomic_classification_code: Option<String>,
    pub pid36_breed_code: Option<String>,
    pub pid37_strain: Option<String>,
    pub pid38_production_class_code: Option<String>,
    pub pid39_tribal_citizenship: Option<Vec<String>>,
    pub pid40_patient_telecommunication_information: Option<Vec<String>>,
    // the following are not fields of this struct but are required for parsing/ building
    pub source: String,
    #[derivative(Default(value = "String::from(\"^~\\\\&\")"))]
    pub encoding_characters: String,
}

impl fmt::Display for PID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut fields: Vec<String> = Vec::new();
        let component_delim = &self.encoding_characters[0..1];
        let repeat_delim = &self.encoding_characters[1..2];
        let _escape_char = &self.encoding_characters[2..3];
        let _sub_component_delim = &self.encoding_characters[3..4];
        fields.push(HEADER.into());
        fields.push(empty_if_none(&self.pid1_set_id));
        fields.push(empty_if_none(&self.pid2_patient_id));
        fields.push(self.pid3_patient_identifier_list.join(repeat_delim));
        fields.push(empty_if_none(&self.pid4_alternate_patient_id));
        fields.push(self.pid5_patient_name.join(component_delim));
        fields.push(optional_vec_to_string(
            &self.pid6_mothers_maiden_name,
            component_delim,
        ));
        fields.push(empty_if_none(&self.pid7_date_time_of_birth));
        fields.push(empty_if_none(&self.pid8_administrative_sex));
        fields.push(empty_if_none(&self.pid9_patient_alias));
        fields.push(optional_vec_to_string(&self.pid10_race, repeat_delim));
        fields.push(optional_vec_to_string(
            &self.pid11_patient_address,
            component_delim,
        ));
        fields.push(empty_if_none(&self.pid12_county_code));
        fields.push(optional_vec_to_string(
            &self.pid13_phone_number_home,
            component_delim,
        ));
        fields.push(optional_vec_to_string(
            &self.pid14_phone_number_business,
            component_delim,
        ));
        fields.push(empty_if_none(&self.pid15_primary_language));
        fields.push(empty_if_none(&self.pid16_marital_status));
        fields.push(empty_if_none(&self.pid17_religion));
        fields.push(empty_if_none(&self.pid18_patient_account_number));
        fields.push(empty_if_none(&self.pid19_ssn_number_patient));
        fields.push(empty_if_none(&self.pid20_drivers_license_number_patient));
        fields.push(optional_vec_to_string(
            &self.pid21_mothers_identifier,
            component_delim,
        ));
        fields.push(optional_vec_to_string(
            &self.pid22_ethnic_group,
            repeat_delim,
        ));
        fields.push(empty_if_none(&self.pid23_birth_place));
        fields.push(empty_if_none(&self.pid24_multiple_birth_indicator));
        fields.push(empty_if_none(&self.pid25_birth_order));
        fields.push(optional_vec_to_string(
            &self.pid26_citizenship,
            repeat_delim,
        ));
        fields.push(empty_if_none(&self.pid27_veterans_military_status));
        fields.push(empty_if_none(&self.pid28_nationality));
        fields.push(empty_if_none(&self.pid29_patient_death_date_and_time));
        fields.push(empty_if_none(&self.pid30_patient_death_indicator));
        fields.push(empty_if_none(&self.pid31_identity_unknown_indicator));
        fields.push(optional_vec_to_string(
            &self.pid32_identity_reliability_code,
            component_delim,
        ));
        fields.push(empty_if_none(&self.pid33_last_update_date_time));
        fields.push(empty_if_none(&self.pid34_last_update_facility));
        fields.push(empty_if_none(&self.pid35_taxonomic_classification_code));
        fields.push(empty_if_none(&self.pid36_breed_code));
        fields.push(empty_if_none(&self.pid37_strain));
        fields.push(empty_if_none(&self.pid38_production_class_code));
        fields.push(optional_vec_to_string(
            &self.pid39_tribal_citizenship,
            repeat_delim,
        ));
        fields.push(optional_vec_to_string(
            &self.pid40_patient_telecommunication_information,
            component_delim,
        ));

        write!(f, "{}", fields.join(DEFAULT_FIELD_SEPARATOR))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_def() {
        let mut s = PID::default();
        s.pid5_patient_name = vec!["John".into(), "Doe".into()];
        println!("{}", s);
        assert_eq!(
            "PID|||||John^Doe|||||||||||||||||||||||||||||||||||",
            s.to_string()
        );
    }
}
