//! Contains the CLI API Defintion.
//!
use clap::{App, AppSettings, Arg, ArgMatches};
use std::iter::IntoIterator;
use std::path::PathBuf;

use crate::labview::installs::Bitness;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct Configuration {
    pub to_launch: PathBuf,
    pub verbose: bool,
    pub lv_version_string: Option<String>,
    pub bitness: Bitness,
    pub timeout_secs: Option<f32>,
}

impl Configuration {
    /// Load configuration from an arguement array. Intended for testing.
    #[allow(dead_code)]
    pub fn from_arg_array(args: Vec<String>) -> Self {
        let matches = clap_app().get_matches_from_safe(args).unwrap();
        Self::args_to_configuration(matches)
    }

    /// Load the configuration from the program arguments.
    /// Will exit the program if the arguments are invalid.
    pub fn from_env() -> Self {
        let matches = clap_app().get_matches();
        Self::args_to_configuration(matches)
    }

    /// Private function to extract the common functionality of moving args to config.
    fn args_to_configuration(args: ArgMatches) -> Self {
        println!("{:?}", args);
        Self {
            to_launch: PathBuf::from(args.value_of("app to run").unwrap().to_owned()),
            verbose: args.is_present("verbose mode"),
            lv_version_string: args.value_of("labview version").map(|str| str.to_owned()),
            bitness: if args.is_present("64bit") {
                Bitness::X64
            } else {
                Bitness::X86
            },
            // todo: use clap validation to remove risk of panic in this unwrap.
            timeout_secs: args
                .value_of("timeout (ms)")
                .map(|str| str.parse::<f32>().unwrap() / 1000.0), //to ms
        }
    }
}

/// Returns a fully configured clap app with all the parameters configured.
fn clap_app() -> clap::App<'static, 'static> {
    App::new("G CLI")
        .version(VERSION)
        .about("Connects a LabVIEW app to the command line.")
        .arg(
            Arg::with_name("verbose mode")
                .short("v")
                .long("verbose")
                .help("Prints additional details for debugging"),
        )
        .arg(
            Arg::with_name("labview version")
                .takes_value(true)
                .long("lv-ver")
                .help("The version of LabVIEW to launch e.g. 2020"),
        )
        .arg(
            Arg::with_name("64bit")
                .long("x64")
                .help("Set this to launch the 64 bit version of LabVIEW."),
        )
        .arg(
            Arg::with_name("timeout (ms)")
                .takes_value(true)
                .long("timeout")
                .help("The time in ms to wait for the connection from LabVIEW"),
        )
        .setting(AppSettings::TrailingVarArg)
        .arg(Arg::with_name("app to run").multiple(true).required(true))
}

/// Extract the arguments that are going to be passed to the VI/exe we will run.
pub fn program_arguments<T: IntoIterator<Item = String>>(main_args: T) -> Vec<String> {
    let args_iter = main_args.into_iter();

    args_iter.skip_while(|s| s != "--").skip(1).collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_item_to_run() {
        let args = vec![
            String::from("g-cli"),
            String::from("test.vi"),
            String::from("--"),
            String::from("test1"),
            String::from("-t"),
            String::from("test2"),
        ];

        let config = Configuration::from_arg_array(args);

        assert_eq!(config.to_launch.to_str().unwrap(), "test.vi");
    }

    #[test]
    fn no_verbose_mode() {
        let args = vec![
            String::from("g-cli"),
            String::from("test.vi"),
            String::from("--"),
            String::from("test1"),
        ];

        let config = Configuration::from_arg_array(args);

        assert_eq!(config.verbose, false);
    }

    #[test]
    fn verbose_mode() {
        let args = vec![
            String::from("g-cli"),
            String::from("-v"),
            String::from("test.vi"),
            String::from("--"),
            String::from("test1"),
        ];

        let config = Configuration::from_arg_array(args);

        assert_eq!(config.verbose, true);
    }

    #[test]
    fn lv_details_32bit() {
        let args = vec![
            String::from("g-cli"),
            String::from("--lv-ver"),
            String::from("2015"),
            String::from("test.vi"),
            String::from("--"),
            String::from("test1"),
        ];

        let config = Configuration::from_arg_array(args);

        assert_eq!(config.lv_version_string.unwrap(), String::from("2015"));
        assert_eq!(config.bitness, Bitness::X86);
    }

    #[test]
    fn lv_details_64bit() {
        let args = vec![
            String::from("g-cli"),
            String::from("--lv-ver"),
            String::from("2015"),
            String::from("--x64"),
            String::from("test.vi"),
            String::from("--"),
            String::from("test1"),
        ];

        let config = Configuration::from_arg_array(args);

        assert_eq!(config.lv_version_string.unwrap(), String::from("2015"));
        assert_eq!(config.bitness, Bitness::X64);
    }

    #[test]
    fn timeout_not_set() {
        let args = vec![
            String::from("g-cli"),
            String::from("--lv-ver"),
            String::from("2015"),
            String::from("test.vi"),
            String::from("--"),
            String::from("test1"),
        ];

        let config = Configuration::from_arg_array(args);
        assert_eq!(None, config.timeout_secs);
    }

    #[test]
    fn timeout_set() {
        let args = vec![
            String::from("g-cli"),
            String::from("--timeout"),
            String::from("10000"),
            String::from("test.vi"),
            String::from("--"),
            String::from("test1"),
        ];

        let config = Configuration::from_arg_array(args);
        assert_eq!(Some(10.0), config.timeout_secs);
    }

    #[test]
    fn get_program_arguments() {
        let args = vec![
            String::from("g-cli"),
            String::from("test.vi"),
            String::from("--"),
            String::from("test1"),
            String::from("-t"),
            String::from("test2"),
        ];

        let processed = program_arguments(args);

        assert_eq!(
            processed,
            vec![
                String::from("test1"),
                String::from("-t"),
                String::from("test2")
            ]
        );
    }

    #[test]
    fn get_program_arguments_empty() {
        let args = vec![String::from("g-cli"), String::from("test.vi")];

        let processed = program_arguments(args);

        assert_eq!(processed, Vec::<String>::new());
    }
}