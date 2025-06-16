use clap::Args;

#[derive(clap::Parser)]
#[command(about = "Curl with OAuth via Keycloak", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: MainCommand,
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(clap::Subcommand, Debug)]
pub enum MainCommand {
    // Http methods first
    #[command(name = "GET", alias = "get", about = "Do Http GET")]
    GetRequest(HttpRequestArgs),
    #[command(name = "POST", alias = "post", about = "Do Http POST")]
    PostRequest(HttpRequestArgs),
    #[command(name = "PUT", alias = "put", about = "Do Http PUT")]
    PutRequest(HttpRequestArgs),
    #[command(name = "DELETE", alias = "delete", about = "Do Http DELETE")]
    DeleteRequest(HttpRequestArgs),
    // Configuration last
    #[command(about = "Configure authentication")]
    Config {
        #[clap(subcommand)]
        command: KeycloakCommand,
    },
}

#[derive(Args, Debug)]
pub struct HttpRequestArgs {
    pub url: String,
    #[arg(
        name = "Header",
        short,
        long = "header",
        help = "For example: 'Content-Type: application/json'"
    )]
    pub headers: Vec<String>,
    #[arg(value_name = "json string", long, help = "JSON Body")]
    pub json_body: Option<String>,
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(clap::Subcommand, Debug)]
pub enum KeycloakCommand {
    Add {
        #[arg(value_name = "Keycloak address")]
        url: String,
        #[arg(short, long, help = "Alias in configuration")]
        alias: String,
        #[arg(short, long, help = "Keycloak Realm")]
        realm: String,
        #[arg(long, help = "Keycloak Client Id")]
        client_id: String,
        #[arg(long, help = "Keycloak Client Secret")]
        client_secret: String,
        #[arg(short, long, help = "Authenticating user name")]
        username: String,
        #[arg(short, long, help = "Authenticating user password")]
        password: String,
        #[arg(long, help = "Make this Keycloak default")]
        default: bool,
    },
    Remove {
        #[arg(help = "Keycloak Alias")]
        alias: String,
    },
    SetDefault {
        #[arg(help = "Keycloak Alias")]
        alias: String,
    },
    List,
}
