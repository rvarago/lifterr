//! Applying a sequence of fail-fast validations to data.

#![allow(dead_code)]

fn main() {}

mod option {
    use lifterr::OptionExt;

    fn validate() -> Option<()> {
        let msg = &[0x05, 0x10][..];

        validate_size(msg)
            .then(|| validate_code(msg))
            .then(|| validate_payload(msg))
    }

    fn validate_size(msg: &[u8]) -> Option<()> {
        (msg.len() >= 2).then(|| ())
    }

    fn validate_code(msg: &[u8]) -> Option<()> {
        msg.get(0).map(|x| *x == 0x05).void()
    }

    fn validate_payload(msg: &[u8]) -> Option<()> {
        msg.get(1).map(|x| *x == 0x10).void()
    }
}

mod result {
    use lifterr::{IntoErr, IntoOk, ResultExt};

    type Result<T> = std::result::Result<T, &'static str>;

    fn validate() -> Result<()> {
        let msg = &[0x05, 0x10][..];

        validate_size(msg)
            .then(|| validate_code(msg))
            .then(|| validate_payload(msg))
    }

    fn validate_size(msg: &[u8]) -> Result<()> {
        if msg.len() >= 2 {
            ().into_ok()
        } else {
            "size".into_err()
        }
    }

    fn validate_code(msg: &[u8]) -> Result<()> {
        msg.get(0).map(|x| *x == 0x05).ok_or("code").void()
    }

    fn validate_payload(msg: &[u8]) -> Result<()> {
        msg.get(1).map(|x| *x == 0x10).ok_or("payload").void()
    }
}
