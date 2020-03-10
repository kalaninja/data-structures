use crate::*;

#[test]
fn test_case_1() {
    let packets = vec![
        Packet::new(0, 0),
    ];

    let expected = vec![
        Some(0),
    ];

    assert_eq!(simulate(packets, 1), expected);
}

#[test]
fn test_case_2() {
    let packets = vec![
        Packet::new(0, 1),
        Packet::new(0, 1)
    ];

    let expected = vec![
        Some(0),
        None
    ];

    assert_eq!(simulate(packets, 1), expected);
}

#[test]
fn test_case_3() {
    let packets = vec![
        Packet::new(0, 1),
        Packet::new(1, 1)
    ];

    let expected = vec![
        Some(0),
        Some(1)
    ];

    assert_eq!(simulate(packets, 1), expected);
}