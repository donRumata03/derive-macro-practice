use from_primitive::FromPrimitive;

#[derive(Debug, PartialEq)]
#[derive(FromPrimitive)]
enum Example {
	A,
	B,
	C,
	D
}

// Следующий код должен быть сгенерирован макросом:
/*
impl TryFrom<usize> for Example {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Example::A),
            1 => Ok(Example::B),
            2 => Ok(Example::C),
            3 => Ok(Example::D),
            _ => Err(())
        }
    }
}
*/

// Тест для проверки:
#[test]
fn test() {
	assert_eq!(Example::A as usize, 0);
	assert_eq!(Example::B as usize, 1);
	assert_eq!(Example::C as usize, 2);
	assert_eq!(Example::D as usize, 3);
	for (i, val) in (0..4).zip([Example::A, Example::B, Example::C, Example::D].into_iter()) {
		assert_eq!(Example::try_from(i), Ok(val));
	}
	assert!(Example::try_from(100).is_err());
}
