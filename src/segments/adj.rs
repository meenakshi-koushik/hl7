use crate::pub_struct;
pub_struct!(ADJ {
    adj1_provider_adjustment_number: String,
    adj2_payer_adjustment_number: String,
    adj3_adjustment_sequence_number: String,
    adj4_adjustment_category: String,
    adj5_adjustment_amount: Option<String>,
    adj6_adjustment_quantity: Option<String>,
    adj7_adjustment_reason_pa: Option<String>,
    adj8_adjustment_description: Option<String>,
    adj9_original_value: Option<String>,
    adj10_substitute_value: Option<String>,
    adj11_adjustment_action: Option<String>,
    adj12_provider_adjustment_number_cross_reference: Option<String>,
    adj13_provider_product_service_line_item_number_cross_reference: Option<String>,
    adj14_adjustment_date: String,
    adj15_responsible_organization: Option<String>,
});
