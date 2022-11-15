use serde::{Deserialize, Serialize};
// this struct is supposed to cover the entire CSS v3 standard
// CSS properties are both optional and there are multiple ways to do the same thing
// set represents itself
// LR = left or right
// TB = top or bottom




#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Clone)]
pub struct RsCSS {
 //   pub css: File,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<Background>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<Border>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "box")]
    pub box_: Option<Box_>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_side: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<Column>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counter: Option<Counter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_cells: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex: Option<Flex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub float: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<Font>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub justify_content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub letter_spacing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_height: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_style: Option<ListStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<Margin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<MaxMin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<MaxMin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline: Option<Outline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overflow: Option<Overflow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<Padding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_break: Option<PageBreak>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perspective: Option<Perspective>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quotes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_layout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<Transform>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition: Option<Transition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visablity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub white_space: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word: Option<Word>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_index: Option<String>,



}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Background {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,   
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Border {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom: Option<Border_TB>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<Border_LR>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<Border_LR>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<Border_TB>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,

}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Border_TB {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_radius: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_radius: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Border_LR {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,

}


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Box_ {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizing: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Column {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<ColumnRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,


}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ColumnRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Counter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub increment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Flex {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basis: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grow: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shrink: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrap: Option<String>,
} 

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Font {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<FontSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strech: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<String>,

}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct FontSize {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjust: Option<String>,

}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ListStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Margin {
    pub bottom: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MaxMin {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
}
#[derive(Serialize, Deserialize,Debug, PartialEq, Clone)]
pub struct Outline {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Overflow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Padding {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PageBreak {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inside: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Perspective {    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Text {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allign: Option<TextAllign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoration: Option<TextDecoration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub justify: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overflow: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<String>,
}    
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TextAllign {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TextDecoration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Transform {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Transition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing_function: Option<String>,

}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Word {
    #[serde(skip_serializing_if = "Option::is_none", rename = "break")]
    pub break_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spacing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrap: Option<String>,
}
