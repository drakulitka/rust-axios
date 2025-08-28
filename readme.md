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

type TypeError = String;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct TypeRequest {
    field: String
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct TypeResponse {
    field: String
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    use ruxios::prelude::*;

    let api = Axios::from(AxiosConfig {
        base_url: String::from("https://api.mysite.com"),
        ..Default::default()
    });

    let res = api.get::<TypeResponse, TypeError>("/my-route").await;

    match res {
        Ok(res) => println!("{:?}", res.data),
        Err(err) => println!("{:?}", err),
    };

    //let res = api.post::<TypeRequest, TypeResponse, TypeError>("/my-route").await;
}
```


## Documentation

[docs.rs](https://docs.rs/rust-axios)


## License

[MIT](https://choosealicense.com/licenses/mit/)


| docs.rs                                              | downloads                                                | Version                                                  |
|------------------------------------------------------|----------------------------------------------------------|----------------------------------------------------------|
| ![docs.rs](https://img.shields.io/docsrs/rust-axios) | ![Crates.io](https://img.shields.io/crates/d/rust-axios) | ![Crates.io](https://img.shields.io/crates/v/rust-axios) |