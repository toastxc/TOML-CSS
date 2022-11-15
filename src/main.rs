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
  
    // border
    let border = if struct_css.border != None {
        css_border(struct_css)
    } else {
        vec![]
    };
    for x in 0..border.len() {
        css = format!("{css}\n{};", border[x]);
    };


    // end 

    css += "\n}";

    println!("{}", css);




}




fn main() {


    // toml to struct
    let div1 = css_import();

   
    // struct to CSS
    css_compiler(div1.example_div);


        
}

