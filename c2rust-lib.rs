#![feature(const_transmute)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(c_variadic)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(main)]
#![feature(ptr_wrapping_offset_from)]
#![feature(register_tool)]
#![register_tool(c2rust)]
//#![deny(warnings)]


extern crate libc;



pub mod base;
pub mod binding;
pub mod dump;
pub mod iconv;
pub mod parser;
pub mod public;
pub mod style;
pub mod widgets {
pub mod wt_box;
pub mod wt_checkbox;
pub mod wt_input;
pub mod wt_label;
pub mod wt_list;
pub mod wt_table;
pub mod wt_textedit;
pub mod wt_textview;
} // mod widgets

