// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>,
// Kevin Knapp (@kbknapp) <kbknapp@gmail.com>, and
// Ana Hobden (@hoverbear) <operator@hoverbear.org>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test for custom crate path attribute
// This test verifies that the #[clap(crate = "...")] attribute works correctly
// when clap is re-exported from another crate.

// First, let's create a module that re-exports clap
mod common {
    pub(super) use clap;
}

#[test]
fn custom_crate_path_parser() {
    use common::clap::Parser;

    #[derive(clap::Parser, Debug, PartialEq)]
    #[clap(crate = "common::clap")]
    struct Args {
        #[arg(short, long)]
        verbose: bool,

        #[arg(short, long)]
        name: Option<String>,
    }

    let args = Args::parse_from(["test", "--verbose", "--name", "foo"]);
    assert_eq!(args.verbose, true);
    assert_eq!(args.name, Some("foo".to_string()));
}

#[test]
fn custom_crate_path_subcommand() {
    use common::clap::Parser;

    #[derive(clap::Parser, Debug, PartialEq)]
    #[clap(crate = "common::clap")]
    struct Cli {
        #[command(subcommand)]
        command: Commands,
    }

    #[derive(clap::Subcommand, Debug, PartialEq)]
    #[clap(crate = "common::clap")]
    enum Commands {
        Add { name: String },
        Remove { name: String },
    }

    let cli = Cli::parse_from(["test", "add", "foo"]);
    assert_eq!(cli.command, Commands::Add { name: "foo".to_string() });
}

#[test]
fn custom_crate_path_args() {
    use common::clap::Parser;

    #[derive(clap::Args, Debug, PartialEq)]
    #[clap(crate = "common::clap")]
    struct Config {
        #[arg(short = 'H', long)]
        host: String,

        #[arg(short, long, default_value = "8080")]
        port: u16,
    }

    #[derive(clap::Parser, Debug)]
    #[clap(crate = "common::clap")]
    struct Cli {
        #[command(flatten)]
        config: Config,
    }

    let cli = Cli::parse_from(["test", "--host", "localhost"]);
    assert_eq!(cli.config.host, "localhost");
    assert_eq!(cli.config.port, 8080);
}

#[test]
fn custom_crate_path_value_enum() {
    use common::clap::Parser;

    #[derive(clap::ValueEnum, Clone, Debug, PartialEq)]
    #[clap(crate = "common::clap")]
    enum Color {
        Red,
        Green,
        Blue,
    }

    #[derive(clap::Parser, Debug)]
    #[clap(crate = "common::clap")]
    struct Args {
        #[arg(short, long)]
        color: Color,
    }

    let args = Args::parse_from(["test", "--color", "red"]);
    assert_eq!(args.color, Color::Red);
}

#[test]
fn custom_crate_path_nested() {
    use common::clap::Parser;

    #[derive(clap::Args, Debug, PartialEq)]
    #[clap(crate = "common::clap")]
    struct Inner {
        #[arg(short, long)]
        value: String,
    }

    #[derive(clap::Parser, Debug)]
    #[clap(crate = "common::clap")]
    struct Outer {
        #[command(flatten)]
        inner: Inner,
    }

    let outer = Outer::parse_from(["test", "--value", "test"]);
    assert_eq!(outer.inner.value, "test");
}

