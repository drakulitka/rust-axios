# rust-axios

Async-centered Rust library similar to the JS library "axios"

!DISCLAIMER!
While `rust-axios` is in an alpha state, consider any minor version changes to be breaking changes.
Raxios will not be out of Alpha state and stable until the 1.0.0 release.


## Features

- JSON, XML, and URL-Encoded Serialization
- JSON Deserialization (XML and others to come)
- Interceptors in dev...
- An "axios"-like api


## Usage/Examples

```rust
use rust_axios::Axios;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ToReceive {
    field1: String
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ToSend {
    field1: String
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // An error _might_ occur here if you try to set non-valid headers in the Options
    let client = Axios::new("", None)?;

    let data_to_send = ToSend { field1 : String::from("Hello World") };

    let result = client
        .post::<ToReceive, ToSend>("/endpoint", Some(data_to_send), None)
        .await?;

    println!("{0}", result.body.unwrap());
}
```


## Documentation

[docs.rs](https://docs.rs/rust-axios)


## License

[MIT](https://choosealicense.com/licenses/mit/)


| docs.rs                                              | downloads                                                | Version                                                  |
|------------------------------------------------------|----------------------------------------------------------|----------------------------------------------------------|
| ![docs.rs](https://img.shields.io/docsrs/rust-axios) | ![Crates.io](https://img.shields.io/crates/d/rust-axios) | ![Crates.io](https://img.shields.io/crates/v/rust-axios) |