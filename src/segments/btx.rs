use crate::pub_struct;
pub_struct!(BTX {
    btx1_set_id: String,
    btx2_bc_donation_id: Option<String>,
    btx3_bc_component: Option<String>,
    btx4_bc_blood_group: Option<String>,
    btx5_cp_commercial_product: Option<String>,
    btx6_cp_manufacturer: Option<String>,
    btx7_cp_lot_number: Option<String>,
    btx8_bp_quantity: String,
    btx9_bp_amount: Option<String>,
    btx10_bp_units: Option<String>,
    btx11_bp_transfusion_disposition_status: String,
    btx12_bp_message_status: String,
    btx13_bp_date_time_of_status: String,
    btx14_bp_transfusion_administrator: Option<String>,
    btx15_bp_transfusion_verifier: Option<String>,
    btx16_bp_transfusion_start_date_time_of_status: Option<String>,
    btx17_bp_transfusion_end_date_time_of_status: Option<String>,
    btx18_bp_adverse_reaction_type: Option<Vec<String>>,
    btx19_bp_transfusion_interrupted_reason: Option<String>,
    btx20_bp_unique_id: Option<String>,
});
