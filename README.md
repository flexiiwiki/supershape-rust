# supershape rust

Inspired by #26 of Coding Challenge from [The Coding Train](https://www.youtube.com/watch?v=f0lkz2gSsIk) using the  [nannou framework](https://github.com/nannou-org/nannou) ([nannou.cc](https://nannou.cc/)) in rust. 

Due to nannou not having extensive 3d capabilities compared to p5.js, rather than using a triangle strip and the like, I opted for a method of creating indices based on the vertices manually, and used those in the 3d mesh in nannou.
## build & run

```console
cargo build --release
OR
cargo run --release
```


## demo
![quat_rYsBgQ4JPV](https://github.com/flexiiwiki/supershape-rust/assets/100071255/83079c9b-fca0-4904-b88b-f9fce6cce486)
