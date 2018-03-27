#[derive(FromInt)]
enum TestEnum {
    VariantOne = 1,
    VariantTwo = -1,
    VariantThree = 528,
    VariantX = 999
}

#[test]
fn check_enum() {
    // Sanity checks
    assert_eq!(TestEnum::VariantOne as i32, 1);
    assert_eq!(TestEnum::VariantTwo as i32, -1);
    assert_eq!(TestEnum::VariantThree as i32, 528);
    assert_eq!(TestEnum::VariantX as i32, 999);
    
    // from_int
    assert_eq!(TestEnum::VariantOne, TestEnum::from_int(1));
    assert_eq!(TestEnum::VariantTwo, TestEnum::from_int(-1));
    assert_eq!(TestEnum::VariantThree, TestEnum::from_int(528));
    assert_eq!(TestEnum::VariantX, TestEnum::from_int(999));
}

#[test]
#[should_panic]
fn check_invalid_int() {
    TestEnum::from_int(123);
}
