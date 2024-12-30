use super::*;

use std::io::Result;

#[test]
fn ops_i32() -> Result<()> {
    let m1 = Mat2::new([[1, 2], [3, 4]]);
    let m2 = Mat2::from([4, 3, 2, 1]);

    assert_eq!(m1 + m2, Mat2::from(5));
    assert_eq!(m1 - m2, Mat2::from(5));
    assert_eq!(m1 * m2, Mat2::from(5));
    assert_eq!(m1 / m2, Mat2::from(5));

    Ok(())
}
