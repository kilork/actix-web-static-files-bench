/*!
# example-02-set-01 description

## Features

## Usage

Add dependency to Cargo.toml:

```toml
[dependencies]
example-02-set-01 = "0.1"
```

*/

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
