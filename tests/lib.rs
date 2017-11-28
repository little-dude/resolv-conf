extern crate resolv_conf;

use std::net::IpAddr;
use std::str::FromStr;

#[test]
fn test_comment() {
    resolv_conf::Config::parse("#").unwrap();
    resolv_conf::Config::parse(";").unwrap();
    resolv_conf::Config::parse("#junk").unwrap();
    resolv_conf::Config::parse("# junk").unwrap();
    resolv_conf::Config::parse(";junk").unwrap();
    resolv_conf::Config::parse("; junk").unwrap();
}

fn ip(s: &str) -> IpAddr {
    IpAddr::from_str(s).unwrap()
}

fn parse_str(s: &str) -> resolv_conf::Config {
    resolv_conf::Config::parse(s).unwrap()
}

#[test]
fn test_basic_options() {
    assert_eq!(
        parse_str("nameserver 127.0.0.1").nameservers,
        vec![ip("127.0.0.1")]
    );
    assert_eq!(
        parse_str("search localnet.*").search,
        vec!["localnet.*".to_string()]
    );
    assert_eq!(
        parse_str("domain example.com.").search,
        vec!["example.com.".to_string()]
    );
}

#[test]
fn test_ip_addr() {
    assert_eq!(
        parse_str("nameserver 127.0.0.1").nameservers[0],
        ip("127.0.0.1")
    );
    assert_eq!(parse_str("nameserver ::1").nameservers[0], ip("::1"));
    assert_eq!(
        parse_str("nameserver 2001:db8:85a3:8d3:1319:8a2e:370:7348").nameservers[0],
        ip("2001:db8:85a3:8d3:1319:8a2e:370:7348")
    );
    assert_eq!(
        parse_str("nameserver ::ffff:192.0.2.128").nameservers[0],
        ip("::ffff:192.0.2.128")
    );
}

// #[test]
// fn test_name() {
//     let mut errors = Vec::new();
//     assert_eq!(
//         resolv_conf::parse_name(&mut errors, ".").unwrap(),
//         Name::from_labels::<String>(vec![])
//     );
//
//     assert_eq!(
//         resolv_conf::parse_name(&mut errors, "com.").unwrap(),
//         Name::from_labels(vec!["com"])
//     );
//
//     assert_eq!(
//         resolv_conf::parse_name(&mut errors, "example.com.").unwrap(),
//         Name::from_labels(vec!["example", "com"])
//     );
// }
//
// #[test]
// fn test_config_line() {
//     let mut errors = Vec::new();
//     // no comment
//     assert_eq!(
//         resolv_conf::parse_config_line(&mut errors, "nameserver 127.0.0.1").expect("failed"),
//         Some(ConfigOption::Basic(BasicOption::Nameserver(
//             IpAddr::from_str("127.0.0.1").unwrap(),
//         )))
//     );
//
//     assert_eq!(
//         resolv_conf::parse_config_line(&mut errors, "nameserver 127.0.0.1; a comment")
//             .expect("failed"),
//         Some(ConfigOption::Basic(BasicOption::Nameserver(
//             IpAddr::from_str("127.0.0.1").unwrap(),
//         )))
//     );
//
//     assert_eq!(
//         resolv_conf::parse_config_line(&mut errors, "nameserver 127.0.0.1# a comment")
//             .expect("failed"),
//         Some(ConfigOption::Basic(BasicOption::Nameserver(
//             IpAddr::from_str("127.0.0.1").unwrap(),
//         )))
//     );
//
//     assert_eq!(
//         resolv_conf::parse_config_line(&mut errors, "nameserver 127.0.0.1 #a comment")
//             .expect("failed"),
//         Some(ConfigOption::Basic(BasicOption::Nameserver(
//             IpAddr::from_str("127.0.0.1").unwrap(),
//         )))
//     );
//
//     assert_eq!(
//         resolv_conf::parse_config_line(&mut errors, "nameserver 127.0.0.1 # a comment")
//             .expect("failed"),
//         Some(ConfigOption::Basic(BasicOption::Nameserver(
//             IpAddr::from_str("127.0.0.1").unwrap(),
//         )))
//     );
//
//     assert_eq!(
//         resolv_conf::parse_config_line(&mut errors, "options ndots:8 # a comment").expect("failed"),
//         Some(ConfigOption::Advanced(
//             vec![AdvancedOption::NumberOfDots(8)],
//         ))
//     );
//
//     // only comment
//     assert_eq!(
//         resolv_conf::parse_config_line(&mut errors, "# a comment").expect("failed"),
//         None
//     );
// }
//
// #[test]
// fn test_advanced_option() {
//     let mut errors = Vec::new();
//     assert_eq!(
//         resolv_conf::parse_advanced_option(&mut errors, "ndots:8").expect("failed"),
//         AdvancedOption::NumberOfDots(8)
//     );
//
//     assert_eq!(
//         resolv_conf::parse_advanced_option(&mut errors, "timeout:8").expect("failed"),
//         AdvancedOption::Timeout(Duration::from_secs(8))
//     );
//
//     assert_eq!(
//         resolv_conf::parse_advanced_option(&mut errors, "attempts:8").expect("failed"),
//         AdvancedOption::Attempts(8)
//     );
// }
//
// #[test]
// fn test_advanced_options() {
//     let mut errors = Vec::new();
//     assert_eq!(
//         resolv_conf::parse_advanced_options(&mut errors, "options ndots:8 timeout:8 attempts:8")
//             .expect("failed"),
//         vec![
//             AdvancedOption::NumberOfDots(8),
//             AdvancedOption::Timeout(Duration::from_secs(8)),
//             AdvancedOption::Attempts(8),
//         ]
//     );
// }
//
// #[test]
// fn test_resolv_conf_macos() {
//     let mut data = String::new();
//     let mut file = File::open("./tests/resolv.conf-macos").unwrap();
//     file.read_to_string(&mut data).unwrap();
//
//     let configuration = vec![
//         ConfigOption::Advanced(vec![
//             AdvancedOption::NumberOfDots(8),
//             AdvancedOption::Timeout(Duration::from_secs(8)),
//             AdvancedOption::Attempts(8),
//         ]),
//         ConfigOption::Basic(BasicOption::Domain(
//             Name::from_labels(vec!["example", "com"]),
//         )),
//         ConfigOption::Basic(BasicOption::Search(vec![
//             Name::from_labels(vec!["example", "com"]),
//             Name::from_labels(vec!["sub", "example", "com"]),
//         ])),
//         ConfigOption::Basic(BasicOption::Nameserver(
//             IpAddr::from_str("2001:4860:4860::8888").unwrap(),
//         )),
//         ConfigOption::Basic(BasicOption::Nameserver(
//             IpAddr::from_str("2001:4860:4860::8844").unwrap(),
//         )),
//         ConfigOption::Basic(BasicOption::Nameserver(
//             IpAddr::from_str("8.8.8.8").unwrap(),
//         )),
//         ConfigOption::Basic(BasicOption::Nameserver(
//             IpAddr::from_str("8.8.4.4").unwrap(),
//         )),
//     ];
//
//     let mut errors = Vec::new();
//     assert_eq!(
//         resolv_conf::parse_config(&mut errors, &data).expect("failed"),
//         configuration
//     );
// }
//
// #[test]
// fn test_resolv_conf_linux() {
//     let mut data = String::new();
//     let mut file = File::open("./tests/resolv.conf-linux").unwrap();
//     file.read_to_string(&mut data).unwrap();
//
//     let configuration = vec![
//         ConfigOption::Advanced(vec![
//             AdvancedOption::NumberOfDots(8),
//             AdvancedOption::Timeout(Duration::from_secs(8)),
//             AdvancedOption::Attempts(8),
//         ]),
//         ConfigOption::Basic(BasicOption::Domain(
//             Name::from_labels(vec!["example", "com"]),
//         )),
//         ConfigOption::Basic(BasicOption::Search(vec![
//             Name::from_labels(vec!["example", "com"]),
//             Name::from_labels(vec!["sub", "example", "com"]),
//         ])),
//         ConfigOption::Basic(BasicOption::Nameserver(
//             IpAddr::from_str("2001:4860:4860::8888").unwrap(),
//         )),
//         ConfigOption::Basic(BasicOption::Nameserver(
//             IpAddr::from_str("2001:4860:4860::8844").unwrap(),
//         )),
//         ConfigOption::Basic(BasicOption::Nameserver(
//             IpAddr::from_str("8.8.8.8").unwrap(),
//         )),
//         ConfigOption::Basic(BasicOption::Nameserver(
//             IpAddr::from_str("8.8.4.4").unwrap(),
//         )),
//         ConfigOption::Advanced(vec![AdvancedOption::Unknown("rotate", None)]),
//         ConfigOption::Advanced(vec![
//             AdvancedOption::Unknown("inet6", None),
//             AdvancedOption::Unknown("no-tld-query", None),
//         ]),
//         ConfigOption::Basic(BasicOption::SortList(
//             vec!["130.155.160.0/255.255.240.0", "130.155.0.0"],
//         )),
//     ];
//
//     let mut errors = Vec::new();
//     assert_eq!(
//         resolv_conf::parse_config(&mut errors, &data).expect("failed"),
//         configuration
//     );
// }
//
// #[test]
// fn test_read_resolv_conf() {
//     read_resolv_conf("./tests/resolv.conf-simple").expect("simple failed");
//     read_resolv_conf("./tests/resolv.conf-macos").expect("macos failed");
//     read_resolv_conf("./tests/resolv.conf-linux").expect("linux failed");
// }
