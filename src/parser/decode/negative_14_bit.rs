use super::variable;
use crate::parser::InternalResult;
use crate::Reader;

pub(crate) fn negative_14_bit(data: &mut Reader) -> InternalResult<i32> {
    let result = variable(data)? as u16;
    let result: i32 = if (result & 0x2000) > 0 {
        (result | 0xC000).cast_signed().into()
    } else {
        result.into()
    };

    Ok(-result)
}

#[cfg(test)]
mod tests {
    use test_case::case;

    use super::*;

    #[case(0, &[0]; "zero")]
    #[case(-0x1FFF, &[0xFF, 0x3F]; "min")]
    #[case(0x2000, &[0x80, 0x40]; "max")]
    #[case(1, &[0xFF, 0x7F]; "all bits set")]
    #[case(1, &[0xFF, 0xFF, 0xFF, 0xFF, 0x7F]; "extra bits ignored")]
    fn decode(expected: i32, bytes: &[u8]) {
        let mut b = Reader::new(bytes);
        assert_eq!(expected, negative_14_bit(&mut b).unwrap());
    }
}
