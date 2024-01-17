use std::error::Error;

use lust::parser::{LuaParser, Rule};
use pest::Parser;

use pretty_assertions::assert_eq;

fn check(expected: &[&str], rule: Rule) -> Result<(), Box<dyn Error>> {
    for x in expected {
        let pairs = LuaParser::parse(rule, x);
        let Ok(pairs) = pairs else {
            panic!("{}", pairs.err().unwrap());
        };
        let pairs: Vec<_> = pairs.into_iter().collect();
        assert_eq!(pairs.len(), 1);
        assert_eq!(&pairs[0].as_str(), x);
    }
    Ok(())
}

#[test]
fn read_integers() -> Result<(), Box<dyn Error>> {
    let integers = ["0", "12345"];
    check(&integers, Rule::Integer)
}

#[test]
fn read_hex_integers() -> Result<(), Box<dyn Error>> {
    let integers = ["0x0", "0x123A5"];
    check(&integers, Rule::HexInteger)
}

#[test]
fn read_floats() -> Result<(), Box<dyn Error>> {
    let floats = [
        "12.",
        "34.",
        ".02",
        ".12",
        "34.e-3",
        "54.e45",
        "43.e23",
        "6.e-1",
        "7e3",
        "7e3",
        "324.1231",
        "432.423e34",
        "123e+43",
        "34e1",
    ];
    check(&floats, Rule::Float)
}

#[test]
fn read_hex_floats() -> Result<(), Box<dyn Error>> {
    let floats = [
        "0x12.",
        "0xf4.",
        "0x.a2",
        "0x.12E",
        "0x34.p-3",
        "0x54.pc5",
        "0x43f.p23",
        "0x6.p-1",
        "0x7p3",
        "0x7p3",
        "0xf324.1231",
        "0x432.423p3a4",
    ];
    check(&floats, Rule::HexFloat)
}

#[test]
fn read_identifiers() -> Result<(), Box<dyn Error>> {
    let ids = ["asads", "a", "_", "as8ads", "_232"];
    check(&ids, Rule::Name)
}

#[test]
fn read_sq_strings() -> Result<(), Box<dyn Error>> {
    let strings = [
        r#"'s'"#,
        r#"'Hello, World!'"#,
        r#"'   '"#,
        r#"'  Hello, \97 World!  '"#,
        r#"''"#,
        r#"'\''"#,
        r#"'\\'"#,
    ];
    check(&strings, Rule::SqString)
}

#[test]
fn read_dq_strings() -> Result<(), Box<dyn Error>> {
    let strings = [
        r#""s""#,
        r#""Hello, World!""#,
        r#""   ""#,
        r#""  Hello, \97 World!  ""#,
        r#""""#,
        r#""\"""#,
        r#""\\""#,
    ];
    check(&strings, Rule::DqString)
}

#[test]
fn read_raw_strings() -> Result<(), Box<dyn Error>> {
    let strings = [
        r#"[[]]"#,
        r#"[[Hello, World]]"#,
        r#"[===[]===]"#,
        r#"[===[ Hello [=[World]=] ]===]"#,
        r#"[=[ a [==[]==] a ]=]"#,
        r#"[===[

                Hello, World

            ]===]"#,
    ];
    check(&strings, Rule::RawString)
}
