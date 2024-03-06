use time_units::AsDuration;

#[test]
fn equality() {
    assert!(
        1.secs() == 1_000.millis()
            && 1.secs() == 1_000_000.micros()
            && 1.secs() == 1_000_000_000.nanos()
    );
    assert!(
        1.0.secs() == 1_000.0.millis()
            && 1.0.secs() == 1_000_000.0.micros()
            && 1.0.secs() == 1_000_000_000.0.nanos()
    );
    assert!(0.5.secs() == 500.millis());
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn negative() {
    let _d = (-1).secs();
}
