use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "g_api")]
pub(crate) struct CliAndEnvArgs {
    /// Port to listen to
    #[structopt(short, long, env = "PORT", default_value = "3000")]
    pub port: u16,

    /// Database URL
    #[structopt(long, env = "DATABASE_URL")]
    pub database_url: String,

    /// Database username
    #[structopt(long, env = "DATABASE_USERNAME")]
    pub database_username: String,

    /// Database password
    #[structopt(long, env = "DATABASE_PASSWORD")]
    pub database_password: String,

}
