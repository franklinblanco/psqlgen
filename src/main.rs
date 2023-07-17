use args::Args;
use clap::Parser;
use generator::generate_sql_files_from_query_needs_list;
use parser::collect_sql_migrations;

mod args;
mod error;
mod generator;
mod parser;

fn main() {
    let args = Args::parse();
    match collect_sql_migrations(args.input_dir) {
        Ok(query_needs_list) => match generate_sql_files_from_query_needs_list(query_needs_list, args.output_dir) {
            Ok(_) => {},
            Err(error) => println!("{:?}", error),
        },
        Err(error) => println!("{:?}", error),
    };
}
