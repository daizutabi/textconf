// mod common;

// #[test]
// fn a() {
//     let e = common::read_example("tests/examples/a.txt").unwrap();
//     assert_eq!(e[0], "{A}{a=1}{b:.2f=2.0}{B}")
// }
// use anyhow::Result;
// use assert_cmd::Command;
// use predicates::prelude::*;
// use pretty_assertions::assert_eq;
// use rand::{distributions::Alphanumeric, Rng};
// use std::fs;

// const PRG: &str = "catr";
// const EMPTY: &str = "tests/inputs/empty.txt";
// const FOX: &str = "tests/inputs/fox.txt";
// const SPIDERS: &str = "tests/inputs/spiders.txt";
// const BUSTLE: &str = "tests/inputs/the-bustle.txt";

// // --------------------------------------------------
// #[test]
// fn usage() -> Result<()> {
//     for flag in &["-h", "--help"] {
//         Command::cargo_bin(PRG)?
//             .arg(flag)
//             .assert()
//             .stdout(predicate::str::contains("Usage"));
//     }
//     Ok(())
// }

// // --------------------------------------------------
// fn gen_bad_file() -> String {
//     loop {
//         let filename: String = rand::thread_rng()
//             .sample_iter(&Alphanumeric)
//             .take(7)
//             .map(char::from)
//             .collect();

//         if fs::metadata(&filename).is_err() {
//             return filename;
//         }
//     }
// }

// // --------------------------------------------------
// #[test]
// fn skips_bad_file() -> Result<()> {
//     let bad = gen_bad_file();
//     let expected = format!("{bad}: .* [(]os error 2[)]");
//     Command::cargo_bin(PRG)?
//         .arg(&bad)
//         .assert()
//         .success()
//         .stderr(predicate::str::is_match(expected)?);
//     Ok(())
// }

// // --------------------------------------------------
// fn run(args: &[&str], expected_file: &str) -> Result<()> {
//     let expected = fs::read_to_string(expected_file)?;
//     let output = Command::cargo_bin(PRG)?.args(args).output().unwrap();
//     assert!(output.status.success());

//     let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
//     assert_eq!(stdout, expected);

//     Ok(())
// }
