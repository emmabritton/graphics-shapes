# Shapes for Graphics

Contains code to make and alter lines, rectangles, circles, triangles and polygons.

Primarily designed to be use with [Buffer Graphics](https://github.com/raybritton/buffer-graphics-lib) and [Graphics lib](https://github.com/raybritton/rust-graphics-lib)

### Usage

Add this line to Cargo.toml
```toml
graphics-shapes = "0.1.6"
# or with both features
graphics-shapes = {version = "0.1.6", features = ["mint", "serde"] }
```

### Code 

```rust
let rect = Rect::new((10,10),(20,20));
assert!(rect.contains((15,15)));
let triangle = Triangle::new((34,5),(12,30),(9,10));
let rotated = triangle.rotate(45);

let start = Coord::new(20,130);
let dist = start.distance((30,130));
```

#### Shapes

All shapes have contains, rotate, scale and translate methods.

Each shape also has custom methods, for example, `len` on `Line`, `union` on `Rect`, `angle_type` on `Triangle`, etc

You should use `Rect` or `Triangle` over `Polygon` where possible as their math methods are optimised.

#### Assertions

This library uses debug assertions for some methods.

### Features

Both are off by default

#### Serde

`serde` adds `serde::{Serialize, Deserialize}` to `Coord`, `Line`, `Rect`, `Circle`, `Triangle`, `Polygon`

#### Mint

`mint` adds a `From` impl for `Point2<isize>` to `Coord`
