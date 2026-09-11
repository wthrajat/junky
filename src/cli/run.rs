use clap::Parser;

use super::args::Cli;
use super::args::Command;
use super::view::print_clean;
use super::view::print_list;
use super::view::print_scan;
use crate::cleaners::catalog::all_cleaners;
use crate::cleaners::catalog::describe_all;
use crate::core::minimum_age::MinimumAge;
use crate::engine::remover::remove_items;
use crate::engine::scanner::scan_all;
use crate::platform::privileges::is_elevated;

pub fn run() -> i32 {
    let cli = Cli::parse();
    let minimum_age = to_minimum_age(cli.older_than_hours);

    match cli.command {
        Command::Scan(args) => {
            let cleaners = all_cleaners(cli.include_aggressive);
            let summary = scan_all(&cleaners, minimum_age);
            print_scan(&summary, cli.json, args.limit);
            0
        }
        Command::Clean(args) => run_clean(cli.json, cli.include_aggressive, minimum_age, args.yes),
        Command::List => {
            let infos = describe_all();
            print_list(&infos, cli.json);
            0
        }
    }
}

fn run_clean(
    as_json: bool,
    include_aggressive: bool,
    minimum_age: MinimumAge,
    confirmed: bool,
) -> i32 {
    if !confirmed {
        if !as_json {
            println!("Refusing to delete without --yes. Run `scan` first to preview.");
        } else {
            println!("{}", serde_json::json!({"error": "missing --yes"}));
        }
        return 2;
    }

    if !is_elevated() && !as_json {
        println!("Running without elevation. Locked and admin-only files will be skipped.");
    }

    let cleaners = all_cleaners(include_aggressive);
    let scanned = scan_all(&cleaners, minimum_age);
    let cleaned = remove_items(&scanned.items);
    print_clean(&scanned, &cleaned, as_json);

    if cleaned.failed_files > 0 { 1 } else { 0 }
}

fn to_minimum_age(hours: Option<u64>) -> MinimumAge {
    match hours {
        Some(hours) => MinimumAge::from_hours(hours),
        None => MinimumAge::none(),
    }
}
