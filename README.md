[![Crates.io](https://img.shields.io/crates/v/graphics-shapes)](https://crates.io/crates/graphics-shapes "Crates.io version")
[![Documentation](https://img.shields.io/docsrs/graphics-shapes)](https://docs.rs/graphics-shapes "Documentation")

# Shapes for Graphics

Contains code to make and alter various shapes.

Primarily designed to be used with [Buffer Graphics](https://github.com/emmabritton/buffer-graphics-lib) and [Pixels graphics lib](https://github.com/emmabritton/pixel-graphics-lib)

### Usage

Add this line to Cargo.toml
```toml
graphics-shapes = "0.4.0"
# or with both features (serde is enabled by default)
graphics-shapes = {version = "0.4.0", features = ["mint"] }
```

### Examples 

```rust
let rect = Rect::new((10,10),(20,20));
assert!(rect.contains(coord!(15,15)));
let triangle = Triangle::new((34,5),(12,30),(9,10));
let rotated = triangle.rotate(45);

let start = coord!(20,130);
let dist = start.distance((30,130));
```

### Shapes

* `Line` 
* `Rect` 
* `Triangle`
* `Circle`
* `Ellipse`
* `Polygon`

#### Shared methods

* `contains` (`Coord` | `Shape`) - Returns true if param is entirely inside 
* `intersects` (`Shape`) - Returns true if param is partially inside/touching
* `outline_pixels` - Returns a list of points that can be used to draw a stroke version
* `filled_pixels` - Returns a list of points that can be used to draw a filled version
* `rotate`, `scale`, `transform` - Copy and change the shape

#### Per shape methods

All the shapes have methods to create similar sized shapes of different types, e.g. `Circle::to_outer_rect()`, `Rect::to_triangles()`

#### Working with multiple shapes

Each `Shape` is a separate struct so to store them without putting them in a `Box` you can use `ShapeBox` which implements `Shape` and so is fully compatible with other `Shape`s and their methods.

#### Assertions

This library uses debug assertions for some methods.

### Features

> Default features: "serde"

#### Serde

`serde` adds `serde::{Serialize, Deserialize}` to `Coord`, `Line`, `Rect`, `Circle`, `Triangle`, `Ellipse`, `Polygon`

#### Mint

`mint` adds a `From` impl for `Point2<isize>` to `Coord`

### Known issues

- `Ellipse`s don't render correctly when rotated