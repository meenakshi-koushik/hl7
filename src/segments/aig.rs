use crate::pub_struct;
pub_struct!(AIG {
    aig1_set_id: String,
    aig2_segment_action_code: Option<String>,
    aig3_resource_id: Option<String>,
    aig4_resource_type: Option<String>,
    aig5_resource_group: Option<Vec<String>>,
    aig6_resource_quantity: Option<String>,
    aig7_resource_quantity_units: Option<String>,
    aig8_start_date_time: Option<String>,
    aig9_start_date_time_offset: Option<String>,
    aig10_start_date_time_offset_units: Option<String>,
    aig11_duration: Option<String>,
    aig12_duration_units: Option<String>,
    aig13_allow_substitution_code: Option<String>,
    aig14_filler_status_code: Option<String>,
});
