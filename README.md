# vpx-rs

Rust wrapper for the libvpx (VP8/VP9) codec library.

```rust
extern crate vpx;

use vpx::Decoder;

fn main() {
	let frames = vec![
		vec![ /* VP8/VP9 binary frame */ ],
		...
	];

    let mut decoder = Decoder::new().unwrap();

    for frame in frames {
        for image in decoder.decode(frame).expect("Failed to decode frame") {
            println!("Decoded image: {}x{}", image.width(), image.height());
        }
    }
}
```

## Requirements

The libvpx library mut be present on the system in order for this wrapper to work.

## License

This project is licensed under the WTFPL license (public domain).
The libvpx library has its own BSD-style license.
