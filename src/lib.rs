use derive_builder::Builder;
use serde::{Serialize};
use serde_json;
#[allow(unused_imports)]
use std::io::Write;
pub mod error;
pub mod pango_markup;
#[derive(Builder,Default,Debug,Serialize)]
#[allow(non_camel_case_types)]
pub struct i3BarJsonObject {
    pub full_text :String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_text :Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color :Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background :Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border :Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_top :Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_right :Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_bottom :Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_left :Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_width :Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align :Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgent :Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator :Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seperator_block_width :Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markup :Option<String>,
}
#[derive(Serialize,Debug)]
#[allow(non_camel_case_types)]
pub struct i3Array {
    i3_blocks : Vec<i3BarJsonObject>,
}

impl i3Array {
    pub fn new()->i3Array{
        i3Array{
            i3_blocks :Vec::<i3BarJsonObject>::new(),
        }
    }
    pub fn push(&mut self, object :i3BarJsonObject)->&i3Array{
        self.i3_blocks.push(object);
        self
    }
    pub fn send(&self)->Result<() , error::Error>{
        serde_json::to_writer(std::io::stdout(),&self.i3_blocks)?;
        print!(",\n");
        Ok(())
    }
}

pub fn init(){
    println!("{{\"version\":1}} \n[[],");
}