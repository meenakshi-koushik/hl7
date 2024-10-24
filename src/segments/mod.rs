mod abs;
mod acc;
mod add;
mod adj;
mod aff;
mod aig;
mod ail;
mod aip;
mod ais;
mod al1;
mod apr;
mod arq;
mod arv;
mod aut;
mod bhs;
mod blc;
mod blg;
mod bpo;
mod bpx;
mod bts;
mod btx;
mod bui;
mod dsc;
mod msh;
mod nk1;
mod nte;
mod obx;
mod orc;
mod pd1;
mod pid;
mod prt;
mod pv1;
mod pv2;
mod segment;
mod sft;
mod uac;

pub use abs::*;
pub use acc::*;
pub use add::*;
pub use adj::*;
pub use aff::*;
pub use aig::*;
pub use ail::*;
pub use aip::*;
pub use ais::*;
pub use al1::*;
pub use apr::*;
pub use arq::*;
pub use arv::*;
pub use aut::*;
pub use bhs::*;
pub use blc::*;
pub use blg::*;
pub use bpo::*;
pub use bpx::*;
pub use bts::*;
pub use btx::*;
pub use bui::*;
pub use dsc::*;
pub use msh::*;
pub use nk1::*;
pub use nte::*;
pub use obx::*;
pub use orc::*;
pub use pd1::*;
pub use pid::*;
pub use prt::*;
pub use pv1::*;
pub use pv2::*;
pub use segment::*;
pub use sft::*;
pub use uac::*;

pub const DEFAULT_FIELD_SEPARATOR: &str = "|";

#[derive(Debug, Fail)]
pub enum SegmentParsingError {
    #[fail(display = "invalid toolchain name: {}", name)]
    Generic { name: String },
    #[fail(display = "error parsing MSH segment: {}", version)]
    MSH { version: String },
}


pub fn some_if_not_empty(x: &str) -> Option<String> {
    if x.len() > 0 {
        Some(x.to_string())
    } else {
        None
    }
}

pub fn empty_if_none(x: &Option<String>) -> String {
    match x.as_ref() {
        None => String::from(""),
        Some(v) => v.clone(),
    }
}

pub fn optional_vec_to_string(x: &Option<Vec<String>>, delim: &str) -> String {
    match x.as_ref() {
        None => String::from(""),
        Some(v) => v.join(delim),
    }
}

pub fn split_repeated(repeat_delim: &str, x: &str) -> Option<Vec<String>> {
    let y: Vec<String> = x.split(repeat_delim).map(|y| y.to_string()).collect();
    if y.is_empty() {
        None
    } else {
        Some(y)
    }
}

