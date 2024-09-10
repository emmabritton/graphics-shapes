# Changelog

### Version 0.4.3
- Update docs
- Update deps

### Version 0.4.2
- Update deps

### Version 0.4.1
- Update deps

### Version 0.4.0
- Add `AnglePosition::Top, Bottom, Left, Right` for `Triangle::right_angle()`
- Change `Triangle::right_angle()` to only take one size 

### Version 0.3.0
- Fix serde feature
- Make serde feature enabled by default

### Version 0.2.7
- Actually Fix prelude/imports

### Version 0.2.6
- Fix prelude/imports

### Version 0.2.5
- Add `coord!` to prelude
- Make `scale_points()` and `rotate_points()` public

### Version 0.2.4
- Add inv_lerp and inv_flerp

### Version 0.2.3
- Adds `Hash` to all shapes except Ellipse

### Version 0.2.2
- Add `Ellipse` and all relevant methods
- Fix bugs in `move_center_to`

### Version 0.2.1
- Remove debug logging

### Version 0.2.0
- Serious breaking changes
  - The `Shape` trait no longer has generics so now `&dyn Shape` can be used
  - Removed `Ellipse` as it's completely broken (hopefully temporarily)
- Adds
  - `ShapeBox` an enum that can hold any shape and implements the shape traits 
  - `coord!` macro, it accepts `number, number` or `(number, number)`  
  - All shapes:
    - `to_shape_box()` 
    - `top_left()`, `top_right()`, `bottom_left()`, `bottom_right()`
    - `intersects_rect()`, `intersects_line()`, `intersects_circle()`, `intersects_triangle()`, `intersects_polygon()`
    - `contains_rect()`, `contains_line()`, `contains_circle()`, `contains_triangle()`, `contains_polygon()`
  - `Rect`:
    - `as_lines()`
  - `Circle`
    - `as_outer_rect()` replacing `as_rect()`, which will be removed in 0.3.0
    - `as_inner_rect()`
  - `Line`
    - `nearest_point()`
- Fixes:
  - `Line`:
    - `Vertical`/`Horizontal` type mix up
    - `left()`, `right()`, `top()`, `bottom()` assuming point order
  - Make `Line` and `Triangle` serializable

### Version 0.1.14
- Add `coord_vec` macro that creates a `Vec<Coord>` from any number of expressions that can converted to Coords using `Coord::from`
- Rename `outline_points` and `filled_points` to `outline_pixels` and `filled_pixels`
- Remove duplicate pixels from `*_pixels` methods

### Version 0.1.13
- Fix typo in rect points

### Version 0.1.12
- Add `outline_points` and `filled_points` for shapes

### Version 0.1.11
- Fix bug with multiplying coords and floats
- Add `Shape::move_center_to`

### Version 0.1.10
- Change angle math to use 0 as top

### Version 0.1.9
- Add coord to prelude

### Version 0.1.8
- Change constructors to take slices instead of vecs
- Fix bug in `move_to`

### Version 0.1.7
- Add prelude module
- Add must_use to all methods

### Version 0.1.6
- Fix bug with Triangle::right_angle() 

### Version 0.1.5
- Add Triangle::right_angle and Triangle::equilateral constructor methods

### Version 0.1.4
- Make several methods const

### Version 0.1.3
- Add Ellipse

### Version 0.1.2
- Add Rect::as_polygon()

### Version 0.1.1
- Change Polygon::as_triangles() to return None for concave polygons
- Add Rect::as_triangles()

### Version 0.1.0
- Initial release