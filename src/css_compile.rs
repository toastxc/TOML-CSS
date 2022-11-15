use crate::rscss::*;

pub fn css_border(struct_css: RsCSS) -> Vec<String> {

    let mut css_vec: Vec<String> = vec![];
        // border

    let border_wp = struct_css.border;

    if border_wp !=  None {

        let border = border_wp.unwrap();

        // border left
        if let Some(ref border_wp) = border.left {

            let left = border.left.unwrap();

            if let Some(ref left) = left.set {
                css_vec.push(format!("border-left: {}", left));
            };
            if let Some(ref left) = left.color {
                css_vec.push(format!("border-left-color: {}", left));
            };
            if let Some(ref left) = left.style{
                css_vec.push(format!("border-left-style: {}", left));
            };
            if let Some(ref left) = left.width {
                css_vec.push(format!("border-left-width: {}", left));
            };
        };

        // border right
        if let Some(ref border_wp) = border.right{

            let right = border.right.unwrap();

            if let Some(ref right) = right.set {
                css_vec.push(format!("border-right: {}", right));
            };
            if let Some(ref right) = right.color {
                css_vec.push(format!("border-right-color: {}", right));
            };
            if let Some(ref right) = right.style{
                css_vec.push(format!("border-right-style: {}", right));
            };
            if let Some(ref right) = right.width {
                css_vec.push(format!("border-right-width: {}", right));
            };
        };

        if let Some(ref border_wp) = border.bottom {

            let bottom = border.bottom.unwrap();

            if let Some(ref bottom) = bottom.set {
                css_vec.push(format!("border-bottom: {}", bottom));
            };
            if let Some(ref bottom) =  bottom.color {
                css_vec.push(format!("border-bottom-color: {}", bottom));
            };
            if let Some(ref bottom) =  bottom.left_radius {
                css_vec.push(format!("border-bottom-left-radius: {}", bottom));
            };
            if let Some(ref bottom) =  bottom.right_radius {
                css_vec.push(format!("border-bottom-right-radius: {}", bottom));
            };
            if let Some(ref bottom) = bottom.style{
                css_vec.push(format!("border-bottom-style: {}", bottom));
            };
            if let Some(ref bottom) = bottom.width {
                css_vec.push(format!("border-bottom-width: {}", bottom));
            };
        };

         if let Some(ref border_wp) = border.top {

            let top = border.top.unwrap();

            if let Some(ref top) = top.set {
                css_vec.push(format!("border-top: {}", top));
            };
            if let Some(ref top) =  top.color {
                css_vec.push(format!("border-top-color: {}", top));
            };
            if let Some(ref top) =  top.left_radius {
                css_vec.push(format!("border-top-left-radius: {}", top));
            };
            if let Some(ref top) =  top.right_radius {
                css_vec.push(format!("border-top-right-radius: {}", top));
            };
            if let Some(ref top) = top.style{
                css_vec.push(format!("border-top-style: {}", top));
            };
            if let Some(ref top) = top.width {
                css_vec.push(format!("border-top-width: {}", top));
            };
        };

        if let Some(ref border) = border.set {
            css_vec.push(format!("border: {}", border))};
        
        if let Some(ref border) = border.color {
            css_vec.push(format!("border-color: {}", border))};

        if let Some(ref border) = border.radius {
            css_vec.push(format!("border-radius: {}", border))};

        if let Some(ref border) = border.width {
            css_vec.push(format!("border: {}", border))};

        if let Some(ref border) = border.style {
            css_vec.push(format!("border: {}", border))};

    };

    return css_vec
}


pub fn css_background(struct_css: RsCSS) -> Vec<String> {

    let mut css_vec: Vec<String> = vec![];

    let background_wp = struct_css.background;
    
    if background_wp !=  None {

        let background = background_wp.unwrap();

        if let Some(ref background) = background.set {
            css_vec.push(format!("background: {}", background))};
        
        if let Some(ref background) = background.attachment {
            css_vec.push(format!("background-attachment: {}", background))};
        
        if let Some(ref background) = background.clip {
            css_vec.push(format!("background-clip: {}", background))};

        if let Some(ref background) = background.color{
            css_vec.push(format!("background-color: {}", background))};

        if let Some(ref background) = background.image{
            css_vec.push(format!("background-image: {}", background))};

        if let Some(ref background) = background.origin{
            css_vec.push(format!("background-origin: {}", background))};

        if let Some(ref background) = background.position{
            css_vec.push(format!("background-position: {}", background))};

        if let Some(ref background) = background.repeat {
            css_vec.push(format!("background-repeat: {}", background))};

        if let Some(ref background) = background.size{
            css_vec.push(format!("background-size: {}", background))};
    };


    return css_vec
}


pub fn css_box(struct_css: RsCSS) -> Vec<String> {

    let mut css_vec: Vec<String> = vec![];

    let box_wp = struct_css.box_;

    if box_wp !=  None {

        let box_ = box_wp.unwrap();

        if let Some(ref box_) = box_.shadow {
            css_vec.push(format!("box-shadow: {}", box_))};

        if let Some(ref box_) = box_.sizing {
            css_vec.push(format!("box-sizing: {}", box_))};

    };

    return css_vec
}
pub fn css_column(struct_css: RsCSS) -> Vec<String> {

    let mut css_vec: Vec<String> = vec![];

    let column_wp = struct_css.column;

    if column_wp !=  None {

        let column = column_wp.unwrap();

        if let Some(ref column) = column.set {
            css_vec.push(format!("columns: {}", column))};

        if let Some(ref column) = column.count {
            css_vec.push(format!("column-count: {}", column))};

        if let Some(ref column) = column.fill {
            css_vec.push(format!("column-fill: {}", column))};


        if let Some(ref column) = column.gap {
            css_vec.push(format!("column-gap: {}", column))};

        if let Some(ref column) = column.rule {
            let rule = column;

            if let Some(ref rule) = rule.set{
                css_vec.push(format!("column-rule: {}", rule))};

            if let Some(ref rule) = rule.color {
                css_vec.push(format!("column-rule-color: {}", rule))};

            if let Some(ref rule) = rule.style{
                css_vec.push(format!("column-rule-style: {}", rule))};

            if let Some(ref rule) = rule.width{
                css_vec.push(format!("column-rule-width: {}", rule))};
        };
        if let Some(ref column) = column.span{
            css_vec.push(format!("column-span: {}", column))};

        if let Some(ref column) = column.width {
            css_vec.push(format!("column-width: {}", column))};
    };
    return css_vec
}

// counter
pub fn css_counter(struct_css: RsCSS) -> Vec<String> {

    let mut css_vec: Vec<String> = vec![];

    let counter_wp = struct_css.counter;

    if counter_wp !=  None {

        let counter = counter_wp.unwrap();

        if let Some(ref counter) = counter.increment {
            css_vec.push(format!("counter-increment: {}", counter))};
     
        if let Some(ref counter) = counter.reset {
            css_vec.push(format!("counter-reset: {}", counter))};
    };
    return css_vec
}


// template
/*
pub fn css_h(struct_css: RsCSS) -> Vec<String> {

    let mut css_vec: Vec<String> = vec![];

    let h_wp = struct_css.h;

    if h_wp !=  None {

        let h = h_wp.unwrap();

        if let Some(ref h) = h.h {
            css_vec.push(format!("h: {}", h))};
    };
    return css_vec

}


*/


