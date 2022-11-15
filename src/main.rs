//html
// css
mod structs {

    pub mod rscss;
}
use crate::structs::rscss;
 use crate::rscss::RsCSS;


// fs

pub mod fs;
use crate::fs::fs_to_string;
use serde_derive::Deserialize;

use serde::{Serialize};

pub mod css_compile;

use css_compile::*;


// add to here to create new items in 
#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct CsStruct {
    pub example_div: RsCSS,
}


fn css_import()  -> CsStruct{    

    let css_file = fs_to_string("css.toml")
        .expect("failed to read CSS file");


    let div1: CsStruct = toml::from_str(&css_file)
        .unwrap();


    return div1

}


fn css_compiler(struct_css: RsCSS) {

      
    let mut finvec: Vec<String> = vec![];
    let mut css = "div {\n".to_string();

    // for each section of CSS

    // background
    let background = if struct_css.background != None {
        css_background(struct_css.clone()) 
    }else {
        vec![] 
    };

    for x in 0..background.len() {
        css = format!("{css}\n{};", background[x]);};


    // border
    let border = if struct_css.border != None {
        css_border(struct_css.clone())
    } else {
        vec![]
    };
    for x in 0..border.len() {
        css = format!("{css}\n{};", border[x]);
    };
    
    // bottom
    if struct_css.bottom != None {
        css = format!("{css}\nbottom: {};", struct_css.bottom.as_ref().unwrap());
    }; 

    // box_
    let box_ = if struct_css.box_ != None {
        css_box(struct_css.clone())
    } else {
        vec![]
    };
    for x in 0..box_.len() {
        css = format!("{css}\n{};", box_[x]);
    };

    if struct_css.caption_side != None {
        css = format!("{css}\ncaption_side: {};", struct_css.caption_side.as_ref().unwrap());
    };
    if struct_css.clear != None {
        css = format!("{css}\nclear: {};", struct_css.clear.as_ref().unwrap());
    };
    if struct_css.clip != None {
        css = format!("{css}\nclip {};", struct_css.clip.as_ref().unwrap());
    };
    if struct_css.color != None {
        css = format!("{css}\ncolor: {};", struct_css.color.as_ref().unwrap());
    };
    // column
    let column = if struct_css.column != None {
        css_column(struct_css.clone())
    }else {
        vec![]
    };
    for x in 0..column.len() {
        css = format!("{css}\n{};", column[x]);
    };
       
    if struct_css.content != None {
        css = format!("{css}\ncontent: {};", struct_css.content.as_ref().unwrap());
    };

     let counter = if struct_css.counter != None {
        css_counter(struct_css.clone())
    }else {
        vec![]
    };
    for x in 0..counter.len() {
        css = format!("{css}\n{};", counter[x]);
    };











    // end 

    css += "\n}";

    println!("{}", css);

/*
   STRUCTURED

   let h = if struct_css.h != None {
        css_h(struct_css.clone())
    }else {
        vec![] 
    };
    for x in 0..h.len() {
        css = format!("{css}\n{};", h[x]);
    };

    SINGLE

        
    if struct_css.h != None {
        css = format!("{css}\nh: {};", struct_css.h.as_ref().unwrap());
    };

*/
}



fn main() {

    // toml to struct
    let div1 = css_import();

   
    // struct to CSS
    css_compiler(div1.example_div);

        
}

