#![allow(dead_code, unused_variables)]
#![feature(unboxed_closures, phase)]
#![experimental]

//! # Handlebars
//! Handlebars is a modern and extensible templating solution originally created in the JavaScript world. It's used by many popular frameworks like [Ember.js](http://emberjs.com) and Chaplin. It's also ported to some other platforms such as [Java](https://github.com/jknack/handlebars.java).
//!
//! And this is handlebars Rust implementation, designed for server-side page generation. It's a general-purpose library so you use it for any kind of text generation.
//!
//! ## Handlebars spec
//!
//! ### Base
//!
//! You can go to [Handlebars.js](http://handlebarsjs.com/) website for its syntax. This implementation should be compatible with most parts of the spec, except:
//!
//! * raw helper syntax `{{{raw-helper}}}...{{{/raw-helper}}}` is implemented as block helper raw.
//! * configurable logging (hard-coded to rust native logging, with fixed level `INFO`)
//!
//! ### Extensions
//!
//! We have template reuse facilities supported via built-in helpers `>`, `partial` and `block`.
//!
//! There are two ways to reuse a template:
//!
//! * include (using `>`)
//! * inheritance (using `>` together with `block` and `partial`)
//!
//! Consult [Handlebar.java document about template inheritance](http://jknack.github.io/handlebars.java/reuse.html).
//!
//! ## Usage
//!
//! ### Template Creation
//!
//! Templates are created from String.
//!
//! ```
//! extern crate handlebars;
//!
//! use handlebars::Template;
//!
//! fn main() {
//!   let source = "hello {{world}}";
//!   //compile returns an Option, we use unwrap() to deref it directly here
//!   let tpl = Template::compile(source.to_string).unwrap();
//! }
//! ```
//!
//! ### Registration
//!
//! All the templates and helpers have to be registered into Registry, so they can look up each other by name.
//!
//! ```
//! extern crate handlebars;
//!
//! use handlebars::{Template, Registry};
//!
//! fn main() {
//!   let source = "hello {{world}}";
//!   //compile returns an Option, we use unwrap() to deref it directly here
//!   let tpl = Template::compile(source.to_string).unwrap();
//!
//!   let mut handlebars = Registry::new();
//!   handlebars.register_template("hello_world", &tpl);
//! }
//! ```
//!
//! ### Rendering Something
//!
//! I should say that rendering is a little tricky. Since handlebars is originally a JavaScript templating framework. It supports dynamic features like duck-typing, truthy/falsy values. But for a static language like Rust, this is a little difficult. As a solution, I'm using the `serialize::json::Json` internally for data rendering, which seems good by far.
//!
//! That means, if you want to render something, you have to ensure that it implements the `serialize::json::ToJson` trait. Luckily, most built-in types already have trait. However, if you want to render your custom struct, you need to implement this trait manually. (Rust has a deriving facility, but it's just for selected types. Maybe I will add some syntax extensions or macros to simplify this process.)
//!
//! ```
//! extern crate handlebars;
//!
//! use serialize::json::ToJson;
//! use std.collections::BTreeMap;
//!
//! use handlebars::{Template, Registry};
//!
//! fn main() {
//!   let source = "hello {{world}}";
//!   //compile returns an Option, we use unwrap() to deref it directly here
//!   let tpl = Template::compile(source.to_string).unwrap();
//!
//!   let mut handlebars = Registry::new();
//!   handlebars.register_template("hello_world", &tpl);
//!
//!   let mut data = BTreeMap::new();
//!   data.insert("world", "world".to_json());
//!   let result = handlebars.render("hello_world", &data);
//! }
//! ```
//!
//! ### Custom Helper
//!
//! Handlebars is nothing without helpers. You can also create your own helpers with rust. Helpers in handlebars-rust are custom struct implements the `HelperDef` trait, concretely, the `resolve` function.
//!
//! ```
//! extern crate handlebars;
//!
//! use handlebars::{Registry, HelperDef, RenderError, RenderContext, Helper, Context};
//!
//! #[deriving(Copy)]
//! struct SimpleHelper;
//!
//! impl HelperDef for SimpleHelper {
//!   fn resolve(&self, c: &Context, h: &Helper, _: &Registry, rc: &mut RenderContext) -> Result<String, RenderError> {
//!     let param = h.params().get(0).unwrap();
//!
//!     // get value from context data
//!     // rc.get_path() is current json parent path, you should always use it like this
//!     // param is the key of value you want to display
//!     let value = c.navigate(rc.get_path(), param);
//!     Ok(format!("My helper dumps: {} ", value))
//!   }
//! }
//!
//! // create an instance
//! static MY_HELPER: SimpleHelper = SimpleHelper;
//!
//! fn main() {
//!   let mut handlebars = Registry::new();
//!   handlebars.register_helper("simple-helper", box MY_HELPER);
//!
//!   //...
//! }
//! ```
//!
//! You can get data from the `Helper` argument about the template information:
//!
//! * `name()` for the helper name. This is known to you for most situation but if you are defining `helperMissing` or `blockHelperMissing`, this is important.
//! * `params()` is a vector of String as params in helper, like `{{#somehelper param1 param2 param3}}`.
//! * `hash()` is a map of String key and Json value, defined in helper as `{{@somehelper a=1 b="2" c=true}}`.
//! * `template()` gives you the nested template of block helper.
//! * `inverse()` gives you the inversed template of it, inversed template is the template behind `{{else}}`.
//!
//! You can learn more about helpers by looking into source code of built-in helpers.
//!
//! ## TODO
//!
//! * More friendly ToJson
//! * Better error report
//!

extern crate serialize;
extern crate regex;
#[phase(plugin)]
extern crate regex_macros;

#[phase(plugin, link)]
extern crate log;

pub use self::template::{Template, Helper};
pub use self::registry::{Registry};
pub use self::render::{Renderable, RenderError, RenderContext};
pub use self::helpers::{HelperDef};
pub use self::context::{Context, JsonRender, JsonTruthy};

mod template;
mod registry;
mod render;
mod helpers;
mod context;
