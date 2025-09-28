use super::super::Drivetrain;

#[test]
fn default_state_ok() {
    let dt = Drivetrain::new();
    assert_eq!(dt.gear(), 0);
    assert!(dt.rpm() >= 800.0 && dt.rpm() <= 1200.0);
    assert_eq!(dt.speed_kmh(), 0.0);
    assert!(dt.throttle() == 0.0 && dt.clutch() == 0.0);
    assert!(dt.engine_on());
}
