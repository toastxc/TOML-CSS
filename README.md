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
# for more complex statements a dot can be used 
border.style="solid"
border.left={color="blue", width="2px"}
border.right={color="red", width="4px"}

# OR

[example_div]
border={style="solid", left={color="blue", width="2px"}, right={color="red", width="4px"}}

```
If the TOML is valid, it will be compiled to normal CSS
```css
example_div {
border-left-color: blue;
border-left-width: 2px;
border-right-color: red;
border-right-width: 4px;
border-style: solid;
}
```

# The purpose of TOML-CSS
I wanted a way to quickly create CSS for Rust based frameworks without using the default structure for CSS while still remaining 100% compatible with vanilla CSS.
I liked the idea of CSS with more structure but couldn't find anything for rust so I made my own


![LGPLv3 Badge](/README_RESOURCES/LGPLv3%20Logo.svg)
