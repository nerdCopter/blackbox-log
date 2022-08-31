#![no_main]

use blackbox_fuzz::{encoding, fuzz_target, UnalignedBytes};

fuzz_target!(|data: UnalignedBytes| {
    let (mut reference, mut biterator) = data.to_streams().unwrap();

    assert_eq!(
        reference.read_negative_14_bit(),
        encoding::read_negative_14_bit(&mut biterator).unwrap_or(0)
    );
});
