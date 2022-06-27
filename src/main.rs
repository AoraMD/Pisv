mod api;
mod command;
mod context;
mod util;
use api::initialize_token;
use clap::{clap_derive::ArgEnum, AppSettings, Parser, Subcommand};
use command::{artist, default_save_path, export, import, like, login, logout};
use context::Context;

const APP_NAME: &str = "pisv";
const APP_NAME_TITLEIZE: &str = "Pisv";

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(setting(AppSettings::ArgRequiredElseHelp))]
struct Args {
    /// Enable debug output
    #[clap(short, long, action)]
    debug: bool,

    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Login
    Login,
    /// Logout
    Logout,
    /// Import token from other computer
    Import {
        /// Token from other computer
        #[clap(value_parser, value_name = "token")]
        token: String,
    },
    /// Export token for other computer
    Export,
    /// Fetch like illustrations
    Like {
        /// Fetch like illustrations incrementally
        #[clap(short, long, action)]
        increment: bool,

        /// Set fetch scope
        #[clap(short, long, arg_enum, value_parser, default_value = "public")]
        scope: Scope,

        /// Set images download path
        #[clap(
            short,
            long,
            value_parser,
            value_name = "path",
            default_value_t = default_save_path("like")
        )]
        path: String,
    },
    Artist {
        /// Fetch like illustrations incrementally
        #[clap(short, long, action)]
        increment: bool,

        /// Set images download path
        #[clap(
            short,
            long,
            value_parser,
            value_name = "path",
            default_value_t = default_save_path("artist")
        )]
        path: String,

        /// Artist ID
        #[clap(value_parser, value_name = "id")]
        id: u64,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Scope {
    Public,
    Private,
}

impl Command {
    fn execute(self, context: &mut Context) {
        match self {
            Command::Login => {
                context.report_debug("run command: login");
                login::main(context);
            }
            Command::Logout => {
                context.report_debug("run command: logout");
                logout::main(context);
            }
            Command::Import { token } => {
                context.report_debug("run command: import");
                import::main(token, context);
            }
            Command::Export => {
                context.report_debug("run command: export");
                export::main(context);
            }
            Command::Like {
                increment,
                scope,
                path,
            } => {
                context.report_debug("run command: like");
                like::main(increment, scope, path, context);
            }
            Command::Artist {
                increment,
                path,
                id,
            } => {
                context.report_debug("run command: artist");
                artist::main(increment, path, id, context);
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    let mut context = Context::new(args.debug);
    initialize_token(&mut context);
    args.command.unwrap().execute(&mut context);
}
