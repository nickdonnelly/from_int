#[macro_use] extern crate from_int_derive;

/// The trait that you derive to get `YourEnum::from_int(i32)`.
pub trait FromInt {
    fn from_int(i: i32) -> Self;
}

#[cfg(test)]
mod tests {
    use ::*;

    #[derive(FromInt, Debug, PartialEq)]
    enum TestEnum {
        VariantOne = 1,
        VariantTwo = 2,
        VariantThree = 528,
        VariantX = 999
    }

    #[test]
    fn check_enum() {
        // Sanity checks
        assert_eq!(TestEnum::VariantOne as i32, 1);
        assert_eq!(TestEnum::VariantTwo as i32, 2);
        assert_eq!(TestEnum::VariantThree as i32, 528);
        assert_eq!(TestEnum::VariantX as i32, 999);
        
        // from_int
        assert_eq!(TestEnum::VariantOne, TestEnum::from_int(1));
        assert_eq!(TestEnum::VariantTwo, TestEnum::from_int(2));
        assert_eq!(TestEnum::VariantThree, TestEnum::from_int(528));
        assert_eq!(TestEnum::VariantX, TestEnum::from_int(999));
    }

    #[test]
    #[should_panic]
    fn check_invalid_int() {
        TestEnum::from_int(123);
    }
}
