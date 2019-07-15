use ff::*;

#[derive(PrimeField)]
#[PrimeFieldModulus = "17"]
#[PrimeFieldGenerator = "3"]
struct ShortFr(ShortFrRepr);

#[test]
fn test_short_square() {
    let mut a = ShortFr::from_repr(ShortFrRepr::from(5)).unwrap();
    a.square();
    assert_eq!("0000000000000008", a.to_hex());
    println!("`5*2 mod 17` into hex = {}", a.to_hex());
}

#[test]
fn test_short_to_hex() {
    let a = ShortFr::from_repr(ShortFrRepr::from(2)).unwrap();
    assert_eq!("0000000000000002", a.to_hex());
    println!("`2` into hex = {}", a.to_hex());
}