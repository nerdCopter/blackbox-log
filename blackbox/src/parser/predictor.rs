use core::ops::{Add, Sub};

use num_enum::TryFromPrimitive;

use super::{as_signed, as_unsigned, Headers, ParseResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive)]
#[repr(u8)]
pub enum Predictor {
    Zero = 0,
    Previous,
    StraightLine,
    Average2,
    MinThrottle,
    Motor0,
    Increment,
    HomeLat, // TODO: check that lat = 0, lon = 1
    FifteenHundred,
    VBatReference,
    LastMainFrameTime,
    MinMotor,
    // HomeLon = 256,
}

impl Predictor {
    #[allow(clippy::too_many_arguments)]
    pub fn apply(
        self,
        headers: &Headers,
        value: u32,
        signed: bool,
        current: &[u32],
        last: Option<u32>,
        last_last: Option<u32>,
        skipped_frames: u32,
    ) -> ParseResult<u32> {
        let _span = if signed {
            tracing::trace_span!(
                "Predictor::apply",
                ?self,
                value = as_signed(value),
                last = last.map(as_signed),
                last_last = last_last.map(as_signed),
                skipped_frames,
            )
        } else {
            tracing::trace_span!(
                "Predictor::apply",
                ?self,
                value,
                last,
                last_last,
                skipped_frames
            )
        };
        let _span = _span.enter();

        let diff = match self {
            Self::Zero => 0,
            Self::Previous => last.unwrap_or(0),
            Self::StraightLine => {
                if signed {
                    as_unsigned(straight_line::<i32, i64>(
                        last.map(as_signed),
                        last_last.map(as_signed),
                    ))
                } else {
                    straight_line::<u32, u64>(last, last_last)
                }
            }
            Self::Average2 => {
                if signed {
                    as_unsigned(average_2_signed(
                        last.map(as_signed),
                        last_last.map(as_signed),
                    ))
                } else {
                    average_2_unsigned(last, last_last)
                }
            }
            Self::MinThrottle => headers.min_throttle.into(),
            Self::Motor0 => headers.main_frames.get_motor_0_from(current)?,
            Self::Increment => {
                if signed {
                    1 + skipped_frames + last.unwrap_or(0)
                } else {
                    let skipped_frames = i32::try_from(skipped_frames)
                        .expect("never skip more than i32::MAX frames");
                    as_unsigned(1 + skipped_frames + as_signed(last.unwrap_or(0)))
                }
            }
            // Self::HomeLat => todo!(), // TODO: check that lat = 0, lon = 1
            Self::FifteenHundred => 1500,
            Self::VBatReference => headers.vbat_reference.into(),
            // Self::LastMainFrameTime => todo!(),
            Self::MinMotor => headers.motor_output_range.min().into(),
            // Self::HomeLon => todo!(),
            Self::HomeLat | Self::LastMainFrameTime => {
                tracing::warn!("found unimplemented predictor: {self:?}");
                0
            }
        };

        Ok(if signed {
            let signed = as_signed(value) + as_signed(diff);
            tracing::trace!(return = signed);
            as_unsigned(signed)
        } else {
            let x = value + diff;
            tracing::trace!(return = x);
            x
        })
    }
}

#[inline]
pub(crate) fn straight_line<T, U>(last: Option<T>, last_last: Option<T>) -> T
where
    T: Copy + Default + TryFrom<U>,
    U: Copy + Sub<Output = U> + Add<Output = U> + From<T>,
{
    match (last, last_last) {
        (Some(last), Some(last_last)) => {
            let result = {
                let last = U::from(last);
                (last - U::from(last_last)) + last
            };
            T::try_from(result).unwrap_or(last)
        }
        (Some(last), None) => last,
        _ => T::default(),
    }
}

macro_rules! average_2 {
    ($name:ident, $input:ty, $overflow:ty) => {
        // REASON: Will not truncate since the average of two `$input`s is guaranteed to
        // fit in $input
        #[allow(clippy::cast_possible_truncation)]
        #[inline]
        fn $name(last: Option<$input>, last_last: Option<$input>) -> $input {
            let last = last.unwrap_or_default();
            last_last.map_or(last, |last_last| {
                ((<$overflow>::from(last) + <$overflow>::from(last_last)) / 2) as $input
            })
        }
    };
}

average_2!(average_2_signed, i32, i64);
average_2!(average_2_unsigned, u32, u64);

#[cfg(test)]
mod tests {
    use test_case::case;

    #[case(None, None => 0)]
    #[case(Some(10), None => 10)]
    #[case(Some(-2), None => -2)]
    #[case(Some(12), Some(10) => 14)]
    #[case(Some(10), Some(12) => 8)]
    #[case(Some(0), Some(i8::MIN) => 0 ; "underflow")]
    #[case(Some(126), Some(0) => 126 ; "overflow")]
    fn straight_line(last: Option<i8>, last_last: Option<i8>) -> i8 {
        super::straight_line::<i8, i16>(last, last_last)
    }

    #[case(None, None => 0)]
    #[case(Some(-1), None => -1)]
    #[case(Some(2), Some(-1) => 0)]
    #[case(Some(i32::MAX), Some(1) => 0x4000_0000 ; "overflow")]
    fn average_2_signed(last: Option<i32>, last_last: Option<i32>) -> i32 {
        super::average_2_signed(last, last_last)
    }

    #[case(None, None => 0)]
    #[case(Some(1), None => 1)]
    #[case(Some(2), Some(10) => 6)]
    #[case(Some(u32::MAX), Some(1) => 0x8000_0000 ; "overflow")]
    fn average_2_unsigned(last: Option<u32>, last_last: Option<u32>) -> u32 {
        super::average_2_unsigned(last, last_last)
    }
}
