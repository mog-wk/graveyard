use super::*;

use std::io::Result;

#[test]
fn ops_i32() -> Result<()> {
    let v1 = Vec2::new(1, 6);
    let v2 = Vec2::new(3, 4);
    assert_eq!(v1 + v2, Vec2::new(4, 10));
    assert_eq!(v1 - v2, Vec2::new(-2, 2));
    assert_eq!(v1 * v2, Vec2::new(3, 24));
    assert_eq!(v1 / v2, Vec2::new(0, 1));
    assert_eq!(v1 % v2, Vec2::new(1, 2));

    assert_eq!(v1 + 3, Vec2::new(4, 9));
    assert_eq!(v1 - 3, Vec2::new(-2, 3));
    assert_eq!(v1 * 3, Vec2::new(3, 18));

    assert_eq!(v1.dot(v2), 27);
    Ok(())
}

#[test]
fn ops_f64() {
    let v1 = Vec2::new(3.0, 4.0);
    let v2 = Vec2::new(5.0, 0.0);
    assert_eq!(v1.mag(), 5.0);
    assert_eq!(v1.angle(v2), 0.6);
}

#[test]
fn axions() -> Result<()> {
    // v + (u + t) = (v + u) + t
    let v1 = Vec2::new(3.0, 4.0);
    let v2 = Vec2::new(5.0, 0.0);
    let v3 = Vec2::new(1.0, 6.0);
    let origin = Vec2::<f64>::zero();
    // assciativity
    assert_eq!(v1 + (v2 + v3), (v1 + v2) + v3);
    // communtativity
    assert_eq!(v1 + v2, v2 + v1);
    // inverse
    assert_eq!(v1.neg() + v1, origin);
    // distributivity
    assert_eq!((v1 + v2) * 2., v1 * 2. + v2 * 2.);
    Ok(())
}
