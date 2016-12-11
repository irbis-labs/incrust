
![](https://img.shields.io/crates/l/incrust.svg) [![](https://img.shields.io/crates/v/incrust.svg)](https://crates.io/crates/incrust)

## {% Incrust %}

> Incrust is a template engine inspired by Jinja2 and written in Rust.

In fact it is a [Jinja2](http://jinja.pocoo.org/), [Django](https://docs.djangoproject.com/en/1.10/topics/templates/),
[Twig](http://twig.sensiolabs.org/), [Swig](http://paularmstrong.github.io/swig/), [Liquid](https://shopify.github.io/liquid/)
(and probably others) template engines constellation, which uses similar methodologies.

### Unstable

The implementation is at a very early stage and the API is a subject of changes.

__Note that Incrust currently requires the nightly version of the Rust compiler.__


## Installation

Incrust is [available on crates.io](https://crates.io/crates/incrust) and can be included in your Cargo enabled project like this:

```
[dependencies]
incrust = "0.2"
```

For ease of use hashmaps you may use the [maplit](https://crates.io/crates/maplit)

Then you need to setup your environment:

```rust
#[macro_use]
extern crate maplit;
extern crate incrust;

use incrust::ex;

fn create_env() -> Incrust {
    use incrust::{Incrust, FilesystemLoader};

    let mut instance = Incrust::default();
    instance.loaders.push(FilesystemLoader::new("./assets/tpl"));
    instance
}

fn main() {
    let incrust = create_env();
    let args = hashmap!{ "name".into() => ex("World") };
    incrust.render("hello", args).unwrap();
}
```

Though Incrust has smart loaders, it may be used just as advanced formatter to render directly from string template

```rust
incrust.render_text("Hello, {{ name | e }}!", hashmap!{ "name".into() => ex("World") }).unwrap();
// or with prepared template
let hello = incrust.parse("Hello, {{ name | e }}!");
let args = hashmap!{ "name".into() => ex("World") };
incrust.render_parsed(&hello, args).unwrap();
```


## Syntax examples

### Filters

```rust
let args = hashmap!{ "title".into() => ex("<Cats & Dogs>") };
```
```twig
<h1>{{ title | escape }}</h1>
```
```html
<h1>&lt;Cats &amp; Dogs&gt;</h1>
```

### Literals

```twig
Braces: {{ "{{" }}
Pi: {{ 3.1415926 }}
```
```html
Braces: {{
Pi: 3.1415926
```

### Expressions

```rust
let args = hashmap!{
    "what".into() => ex("Hello"),
    "who".into() => ex("World")
};
```
```twig
Say: "{{ what + ", " + who }}!"
```
```html
Say: "Hello, World!"
```

```rust
let args = hashmap!{
    "alpha".into() => ex(6isize),
    "omega".into() => ex(7f64)
};
```
```twig
The answer is {{ alpha * omega }}
```
```html
The answer is 42
```

### Lazy boolean evaluation

```twig
Amount: {{ amount and ("" + amount + " pcs") or "-" }}
```
```rust
assert_eq!("Amount: 6 pcs", incrust.render("tpl", hashmap!{ "amount".into() => ex(6isize) }).unwrap());
assert_eq!("Amount: -", incrust.render("tpl", hashmap!{ "amount".into() => ex(0isize) }).unwrap());
```

### Conditional statements

```twig
String {% if "" %}has chars{% else %}is empty{% endif %}
It's {% if False %}false{% elif True %}true{% endif %}
```
```html
String is empty
It's true
```

### For-Loop statements

```rust
let args = hashmap!{ "fruits".into() => ex(vec![ex("Orange"), ex("Apple"), ex("Banana")]) };
```
```twig
<ul>{% for fruit in fruits %}<li>{{ index }}. {{ fruit | e }}</li>{% endfor %}</ul>
```
```html
<ul><li>1. Orange</li><li>2. Apple</li><li>3. Banana</li></ul>
```

### Comments

```twig
<p>Visible {# partially #} paragraph</p>
```
```html
<p>Visible  paragraph</p>
```

### Escaping

```twig
Example: {% raw %}{{ mustaches }}{% endraw %}
```
```html
Example: {{ mustaches }}
```


## Alternatives

If you are looking for a template engine for your project, you may also look at these projects.

### With a similar to Incrust syntax

 * [cobalt-org/liquid-rust](https://github.com/cobalt-org/liquid-rust)   Liquid templating for Rust
 * [colin-kiegel/twig-rust](https://github.com/colin-kiegel/twig-rust)   Rust port of the twig-php template library
 * [Nercury/twig-rs](https://github.com/Nercury/twig-rs)   The Twig templating engine for Rust (work in progress)
 * [Keats/tera](https://github.com/Keats/tera)   A template engine for Rust

### Others

 * [sunng87/handlebars-rust](https://github.com/sunng87/handlebars-rust)   Rust templating with Handlebars
 * [nickel-org/rust-mustache](https://github.com/nickel-org/rust-mustache)   Mustache template library for rust


## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.


### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
