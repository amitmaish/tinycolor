# tinycolors

 this crate simplifies the process of working with different color spaces, it consists of two parts: color structs, and the color trait.

## structs

 the structs in this module represent a color in a particular color space. all structs can be cast into each other, as well as casting to and from a [f32; 3]

 ``` rust
 use tinycolors::srgb;

 let color0 = srgb{r: 1.0, g: 0.5, b: 0.25};
 let color1: srgb = [1.0, 0.5, 0.25].into();

 assert_eq!(color0, color1);
 ```

## the color trait

 every color struct implements the color trait. the color trait ensures that every struct that implements it can be cast to all the colors. when writing functions that require a color, using a generic color allows the caller to store their colors in whatever format they want.

 ``` rust
 use tinycolors::{Color, srgb, rgb};

 fn any_color_as_rgb<T: Color>(color: T) -> rgb {
     color.into()
 }

 let color_srgb = srgb::WHITE;
 let color_rgb = any_color_as_rgb(color_srgb);
 assert_eq!(color_rgb, rgb::from(color_srgb));
 ```

 > **_NOTE:_** conversions to and from okhsl and okhsv are thin wrappers for the `okhsl` crate. if that's all you're using, that crate might be a better choice for you.
