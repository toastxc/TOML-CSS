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

    let background = if struct_css.background != None {
        css_background(struct_css.clone()) 
    }else {
        vec![] 
    };
    for x in 0..background.len() {
        css = format!("{css}\n{};", background[x]);
    };
    let border = if struct_css.border != None {
        css_border(struct_css.clone())
    } else {
        vec![]
    };
    for x in 0..border.len() {
        css = format!("{css}\n{};", border[x]);
    };
    if struct_css.bottom != None {
        css = format!("{css}\nbottom: {};", struct_css.bottom.as_ref().unwrap());
    }; 
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
    if struct_css.cursor != None {
        css = format!("{css}\ncursor: {};", struct_css.cursor.as_ref().unwrap());
    };
    if struct_css.direction != None {
        css = format!("{css}\ndirection: {};", struct_css.direction.as_ref().unwrap());
    };
    if struct_css.display != None {
        css = format!("{css}\ndisplay: {};", struct_css.display.as_ref().unwrap());
    };
    if struct_css.empty_cells != None {
        css = format!("{css}\nempty-cells: {};", struct_css.empty_cells.as_ref().unwrap());
    }; 
    let flex = if struct_css.flex != None {
        css_flex(struct_css.clone())
    }else {
        vec![]
    };
    for x in 0..flex.len() {
        css = format!("{css}\n{};", flex[x]);
    };
    if struct_css.float != None {
        css = format!("{css}\nfloat: {};", struct_css.float.as_ref().unwrap());
    };
    let font = if struct_css.font != None {
        css_font(struct_css.clone())
    }else {
        vec![]
    };
    for x in 0..font.len() {
        css = format!("{css}\n{};", font[x]);
    };
    if struct_css.height != None {
        css = format!("{css}\nheight: {};", struct_css.height.as_ref().unwrap());
    };
    if struct_css.justify_content != None {
        css = format!("{css}\njustify-content: {};", struct_css.justify_content.as_ref().unwrap());
    };
    if struct_css.left != None {
        css = format!("{css}\nleft: {};", struct_css.left.as_ref().unwrap());
    };
    if struct_css.letter_spacing != None {
        css = format!("{css}\nletter-spacing: {};", struct_css.letter_spacing.as_ref().unwrap());
    };
    if struct_css.line_height != None {
        css = format!("{css}\nline-height: {};", struct_css.line_height.as_ref().unwrap());
    };
    let list_style= if struct_css.list_style != None {
        css_list_style(struct_css.clone())
    }else {
        vec![]
    };
    for x in 0..list_style.len() {
        css = format!("{css}\n{};", list_style[x]);
    };
    let margin = if struct_css.margin != None {
        css_margin(struct_css.clone())
    }else {
        vec![]
    };
    for x in 0..margin.len() {
        css = format!("{css}\n{};", margin[x]);
    };
    let max = if struct_css.max != None {
        css_max(struct_css.clone())
    }else {
        vec![]
    };
    for x in 0..max.len() {
        css = format!("{css}\n{};", max[x]);
    };
    let min = if struct_css.min != None {
        css_min(struct_css.clone())
    }else {
        vec![]
    };
    for x in 0..min.len() {
        css = format!("{css}\n{};", min[x]);
    };
      
    if struct_css.opacity != None {
        css = format!("{css}\nopacity: {};", struct_css.opacity.as_ref().unwrap());
    };
      
    if struct_css.order != None {
        css = format!("{css}\norder: {};", struct_css.order.as_ref().unwrap());
    };
      
    let outline = if struct_css.outline != None {
        css_outline(struct_css.clone())
    }else {
        vec![]
    };
    for x in 0..outline.len() {
        css = format!("{css}\n{};", outline[x]);
    };

     
    let overflow = if struct_css.overflow != None {
        css_overflow(struct_css.clone())
    }else {
        vec![]
    };
    for x in 0..overflow.len() {
        css = format!("{css}\n{};", overflow[x]);
    };   
    let padding = if struct_css.padding != None {
        css_padding(struct_css.clone())
    }else {
        vec![]
    };
    for x in 0..padding.len() {
        css = format!("{css}\n{};", padding[x]);
    };

     
    let page_break = if struct_css.page_break != None {
        css_page_break(struct_css.clone())
    }else {
        vec![]
    };
    for x in 0..page_break.len() {
        css = format!("{css}\n{};", page_break[x]);
    }; 

     let perspective = if struct_css.perspective != None {
        css_perspective(struct_css.clone())
    }else {
        vec![]
    };
    for x in 0..perspective.len() {
        css = format!("{css}\n{};", perspective[x]);
    };

    if struct_css.position != None {
        css = format!("{css}\nh: {};", struct_css.position.as_ref().unwrap());
    };
    if struct_css.quotes != None {
        css = format!("{css}\nquotes: {};", struct_css.quotes.as_ref().unwrap());
    };
    if struct_css.resize != None {
        css = format!("{css}\nresize: {};", struct_css.resize.as_ref().unwrap());
    };
    if struct_css.right != None {
        css = format!("{css}\nright: {};", struct_css.right.as_ref().unwrap());
    };
    if struct_css.tab_size != None {
        css = format!("{css}\ntab-size: {};", struct_css.tab_size.as_ref().unwrap());
    };
    if struct_css.table_layout != None {
        css = format!("{css}\ntable-layout: {};", struct_css.table_layout.as_ref().unwrap());
    };
    let text = if struct_css.text != None {
        css_text(struct_css.clone())
    }else {
        vec![]
    };
    for x in 0..text.len() {
        css = format!("{css}\n{};", text[x]);
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

