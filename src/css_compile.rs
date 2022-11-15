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

        // prefered method
        if let Some(ref border) = border.set {
        css_vec.push(format!("border: {}", border))};

    };

    return css_vec

}
