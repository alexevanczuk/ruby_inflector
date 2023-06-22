# Rust Inflector

This project is forked from https://github.com/chrislearn/cruet.

The intent of this library is to emulate https://api.rubyonrails.org/classes/ActiveSupport/Inflector.html as closely as possible to allow for rust implementations of ruby tools.

![CI](https://github.com/alexevanczuk/ruby_inflector/actions/workflows/ci.yml/badge.svg)
![Audit](https://github.com/alexevanczuk/ruby_inflector/actions/workflows/audit.yml/badge.svg)

Adds String based inflections for Rust. Snake, kebab, train, camel,
sentence, class, and title cases as well as ordinalize,
deordinalize, demodulize, deconstantize, foreign key, table case, and pluralize/singularize are supported as both traits and pure functions
acting on &str and String types.

-----
## Documentation:

Documentation can be found here at the README or via rust docs below.

[Rust docs with examples](https://docs.rs/ruby_inflector)

-----

## Installation:

### As a [crate](http://crates.io)

```toml
[dependencies]
ruby_inflector = "*"
```

### Compile yourself:

1. Install [Rust and cargo](http://doc.crates.io/)
2. git clone https://github.com/alexevanczuk/ruby_inflector
3. Library: cd ruby_inflector && cargo build --release --lib
4. You can find the library in target/release

## Usage / Example:

```rust
// to use methods like String.to_lower_case();
use ruby_inflector::Inflector;
fn main() {
  let camel_case_string: String = "some_string".to_camel_case();
}

```

Or

```rust
// to use methods like to_snake_case(&str);
use ruby_inflector;

// use ruby_inflector::to_class_case;
// use ruby_inflector::is_class_case;

// use ruby_inflector::to_camel_case;
// use ruby_inflector::is_camel_case;

// use ruby_inflector::to_pascal_case;
// use ruby_inflector::is_pascal_case;

// use ruby_inflector::to_screamingsnake_case;
// use ruby_inflector::is_screamingsnake_case;

// use ruby_inflector::to_snake_case;
// use ruby_inflector::is_snake_case;

// use ruby_inflector::to_kebab_case;
// use ruby_inflector::is_kebab_case;

// use ruby_inflector::to_train_case;
// use ruby_inflector::is_train_case;

// use ruby_inflector::to_sentence_case;
// use ruby_inflector::is_sentence_case;

// use ruby_inflector::to_title_case;
// use ruby_inflector::is_title_case;

// use ruby_inflector::to_table_case;
// use ruby_inflector::is_table_case;

// use ruby_inflector::ordinalize;
// use ruby_inflector::deordinalize;

// use ruby_inflector::to_foreign_key;
// use ruby_inflector::is_foreign_key;

// use ruby_inflector::demodulize;
// use ruby_inflector::deconstantize;

// use ruby_inflector::to_plural;
// use ruby_inflector::to_singular;
fn main() {
  let camel_case_string: String = to_camel_case("some_string");
}
```

## [Contributing](CONTRIBUTING.md)

This project is intended to be a safe, welcoming space for collaboration, and contributors are expected to adhere to the [Contributor Covenant](http://contributor-covenant.org) code of conduct.
