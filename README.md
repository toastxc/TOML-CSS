<h1 align="center">
  TOML-CSS
</h1>
<h3 align="center">
  An alternate for normal CSS
</h3>


TOML-CSS converts structured .toml files into CSSv3 compliant text files




# Example
Every CSS object is deserialized using a struct (yes there is a struct for every CSS property).
```rust
// add to here to create new items in 
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CsStruct {
    pub example_div: RsCSS,
}
```

Once your object is named with the type RsCSS (short for Rust CSS) you can create an TOML config with the same name
```toml
[example_div]

# 'top level' properties can be simply defined by their names
color = "white"

# objects with several properties can be defined in a group
border={width="2px", radius="20px", style="solid", color="blue", left={set="hi", color="s"}}


# or as a single statement with 'self' (commented out to avoid errors)
#border={self="2px solid #85101F"} 

# just like normal CSS not all fields have to be met
background={color="#85101F"}
```
If the TOML is valid, it will be compiled to normal CSS
```css
example_div {
  border-left: hi;
  border-left-color: s;
  border-color: blue;
  border-radius: 20px;
  border-width: 2px;
  border-style: solid;
  
  /* the rest is yet to be completed */
}
```

# The purpose of TOML-CSS
I wanted a way to quickly create CSS for Rust based frameworks without using the default structure for CSS while still remaining 100% compatible with vanilla CSS.
I liked the idea of CSS with more structure but couldn't find anything for rust so I made my own


![LGPLv3 Badge](/README_RESOURCES/LGPLv3%20Logo.svg)
