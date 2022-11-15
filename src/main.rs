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


// add to here to create new items in 
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CsStruct {
    pub example_div: RsCSS,
}


fn cs2()  -> CsStruct{    

    let css_file = fs_to_string("css.toml")
        .expect("failed to read CSS file");


    let div1: CsStruct = toml::from_str(&css_file)
        .unwrap();


    return div1

}


fn struct_to_css(struct_css: RsCSS) {




    
    let mut css_vec: Vec<String> = vec![];

    // border 
    
    let border_wp = struct_css.border;    
    
   
    
    if border_wp != None {
        
        let border = border_wp.unwrap();

        if let Some(ref border_wp) = border.left {

            let left = border.left.unwrap();

              
            if let Some(ref left) = left.set {
                println!("set!!");
                css_vec.push(format!("border-left: {}", left));
            };
            if let Some(ref left) = left.color {
                println!("color!!");
                css_vec.push(format!("border-left-color: {}", left));
            };


        };

        // prefered method
        if let Some(ref border) = border.set {
        css_vec.push(format!("border: {}", border))};



        // old method
        if let Some(ref border_wp) = border.color{
        css_vec.push(format!("border-color: {}", border.color.as_ref().unwrap()));};


        if let Some(ref border_wp) = border.radius{
        css_vec.push(format!("border-radius: {}", border.radius.as_ref().unwrap()));};

        if let Some(ref border_wp) = border.width{
        css_vec.push(format!("border-width: {}", border.width.as_ref().unwrap()));};

        if let Some(ref border_wp) = border.style{
        css_vec.push(format!("border-style: {}", border.style.as_ref().unwrap()));};

    };

   
    println!("{:?}", css_vec);

}




fn main() {


    // toml to struct
    let div1 = cs2();

   
    // struct to CSS
    struct_to_css(div1.example_div);


        
}

