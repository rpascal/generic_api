pub mod cli_args;

use structopt::StructOpt;


pub(crate) fn cli_and_env_arguments() -> cli_args::CliAndEnvArgs {
    dotenv::dotenv().ok();
    cli_args::CliAndEnvArgs::from_args()
}