//! Implementing `Arbitrary` trait from crate `proptest` for the u* and s* types.
//! This allows them to be easily used as inputs in `proptest` propery based tests.
//!
//! The arbitrary selection strategy is adapted using `proptest`'s range strategy,
//! with range from minimum value to the maximum value.

use crate::*;
use ::proptest::arbitrary::Arbitrary;
use ::proptest::strategy::{Strategy, BoxedStrategy};

macro_rules! implement_arbitrary {
    ([$($name:ident),+], $type:ident) => {$(implement_arbitrary!($name, $type);)+};
    ($name:ident, $type:ident) => {
        impl Arbitrary for $name {
            type Parameters = ();
            type Strategy = BoxedStrategy<Self>;

            fn arbitrary_with(_args: ()) -> BoxedStrategy<Self> {
                let range = $type::from(Self::MIN)..=$type::from(Self::MAX);
                range.prop_map(
                    |x| Self::try_from(x).expect(
                        "Value should be in range due to the interval we're choosing from"
                    )
                ).boxed()
            }
        }
    }
}


implement_arbitrary!([u1, u2, u3, u4, u5, u6, u7], u8);
implement_arbitrary!([u9, u10, u11, u12, u13, u14, u15], u16);
implement_arbitrary!(
    [
        u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31
    ],
    u32
);
implement_arbitrary!(
    [
        u33, u34, u35, u36, u37, u38, u39, u40, u41, u42, u43, u44, u45, u46, u47,
        u48, u49, u50, u51, u52, u53, u54, u55, u56, u57, u58, u59, u60, u61, u62,
        u63
    ],
    u64
);

implement_arbitrary!([i2, i3, i4, i5, i6, i7], i8);
implement_arbitrary!([i9, i10, i11, i12, i13, i14, i15], i16);
implement_arbitrary!(
    [
        i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31
    ],
    i32
);
implement_arbitrary!(
    [
        i33, i34, i35, i36, i37, i38, i39, i40, i41, i42, i43, i44, i45, i46, i47,
        i48, i49, i50, i51, i52, i53, i54, i55, i56, i57, i58, i59, i60, i61, i62,
        i63
    ],
    i64
);
// Arbitrary implementations for i1, u65-u127 and i65-i127 are intentionally missing,
// because these are not supported for conversions.

#[cfg(test)]
mod tests {
    use super::*;
    use ::proptest::prelude::*;

    proptest!{
        #[test]
        fn u7_doesnt_panic(_ in any::<u7>()) {}
    }
}
