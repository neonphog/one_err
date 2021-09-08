use super::*;

#[test]
fn assert_bounds() {
    fn assert_bounds<T: 'static + Send + Sync + Unpin>(_t: T) {}
    let one_err: OneErr = std::io::ErrorKind::Other.into();
    assert_bounds(one_err);
}

#[test]
fn disp_ser_deser() {
    let test = |io_s: &'static str,
                errno_s: &'static str,
                s_s: &'static str,
                e: OneErr| {
        // make sure our expected kinds are equal
        assert_eq!(io_s, &format!("{:?}", e.io_kind()));
        assert_eq!(errno_s, &format!("{}", e.errno()));
        assert_eq!(s_s, e.str_kind());

        // make sure a clone is the same
        let c = e.clone();
        assert_eq!(io_s, &format!("{:?}", c.io_kind()));
        assert_eq!(errno_s, &format!("{}", c.errno()));
        assert_eq!(s_s, c.str_kind());
        assert_eq!(e, c);

        // serialize the error
        let ser = format!("{}", e);
        println!(
            "SER: {:?}/{}/{}: {}",
            e.io_kind(),
            e.errno(),
            e.str_kind(),
            ser
        );

        // deserialize the error
        let dec: OneErr = ser.parse().unwrap();
        println!(
            "DEC: {:?}/{}/{} {:?}",
            dec.io_kind(),
            dec.errno(),
            dec.str_kind(),
            dec.priv_as_inner().0
        );

        // make sure the deserialize error is the same
        assert_eq!(io_s, &format!("{:?}", dec.io_kind()));
        assert_eq!(errno_s, &format!("{}", dec.errno()));
        assert_eq!(s_s, dec.str_kind());
        assert_eq!(e, dec);
    };

    let mut e = OneErr::from(std::io::ErrorKind::ConnectionReset);
    e.set_field("test", "test");
    test("ConnectionReset", "ECONNRESET", "ConnectionReset", e);

    let mut e = OneErr::from(std::io::Error::new(
        std::io::ErrorKind::ConnectionReset,
        "a message from the future",
    ));
    e.set_field("test", "test");
    test("ConnectionReset", "ECONNRESET", "ConnectionReset", e);

    let mut e =
        OneErr::from(std::io::Error::from_raw_os_error(libc::ECONNRESET));
    e.set_field("test", "test");
    test("ConnectionReset", "ECONNRESET", "ConnectionReset", e);

    let mut e = OneErr::from(std::io::Error::new(
        std::io::ErrorKind::Other,
        "a message from the future",
    ));
    e.set_field("test", "test");
    test("Other", "EOTHER", "Other", e);

    let mut e = OneErr::from(ErrNo::Fault);
    e.set_field("test", "test");
    test("Other", "EFAULT", "EFAULT", e);

    let mut e = OneErr::from(std::io::Error::from_raw_os_error(libc::EFAULT));
    e.set_field("test", "test");
    test("Other", "EFAULT", "EFAULT", e);

    let mut e = OneErr::from(ErrNo::Other);
    e.set_field("test", "test");
    test("Other", "EOTHER", "Other", e);

    let e = OneErr::with_message("ConnectionReset", "my msg");
    test("ConnectionReset", "ECONNRESET", "ConnectionReset", e);

    let e = OneErr::with_message("EFAULT", "my msg");
    test("Other", "EFAULT", "EFAULT", e);

    let e = OneErr::with_message("CustomMsg", "my msg");
    test("Other", "EOTHER", "CustomMsg", e);

    let e = <Result<(), &'static str>>::Err("foo")
        .map_err(OneErr::new)
        .unwrap_err();
    test("Other", "EOTHER", "foo", e);

    let e = std::str::from_utf8(&[0, 159, 146, 150])
        .map_err(OneErr::new)
        .unwrap_err();
    test(
        "Other",
        "EOTHER",
        "invalid utf-8 sequence of 1 bytes from index 1",
        e,
    );
}
