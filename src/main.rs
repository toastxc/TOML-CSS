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

