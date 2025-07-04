use crate::commands;
use crate::commands::command::Command;
use crate::config::FnmConfig;
use clap::Parser;

#[derive(clap::Subcommand, Debug)]
pub enum SubCommand {
    /// List all remote Node.js versions
    #[command(name = "list-remote", visible_aliases = &["ls-remote"])]
    LsRemote(commands::ls_remote::LsRemote),

    /// List all locally installed Node.js versions
    #[command(name = "list", visible_aliases = &["ls"])]
    LsLocal(commands::ls_local::LsLocal),

    /// Install a new Node.js version
    #[command(name = "install", visible_aliases = &["i"])]
    Install(commands::install::Install),

    /// Change Node.js version
    #[command(name = "use")]
    Use(commands::r#use::Use),

    /// Print and set up required environment variables for fnm
    ///
    /// This command generates a series of shell commands that
    /// should be evaluated by your shell to create a fnm-ready environment.
    ///
    /// Each shell has its own syntax of evaluating a dynamic expression.
    /// For example, evaluating fnm on Bash and Zsh would look like `eval "$(fnm env)"`.
    /// In Fish, evaluating would look like `fnm env | source`
    #[command(name = "env")]
    Env(commands::env::Env),

    /// Print shell completions to stdout
    #[command(name = "completions")]
    Completions(commands::completions::Completions),

    /// Alias a version to a common name
    #[command(name = "alias")]
    Alias(commands::alias::Alias),

    /// Remove an alias definition
    #[command(name = "unalias")]
    Unalias(commands::unalias::Unalias),

    /// Set a version as the default version
    ///
    /// This is a shorthand for `fnm alias VERSION default`
    #[command(name = "default")]
    Default(commands::default::Default),

    /// Print the current Node.js version
    #[command(name = "current")]
    Current(commands::current::Current),

    /// Run a command within fnm context
    ///
    /// Example:
    /// --------
    /// fnm exec --using=v12.0.0 node --version
    /// => v12.0.0
    #[command(name = "exec", verbatim_doc_comment)]
    Exec(commands::exec::Exec),

    /// Uninstall a Node.js version
    ///
    /// > Warning: when providing an alias, it will remove the Node version the alias
    /// > is pointing to, along with the other aliases that point to the same version.
    #[command(name = "uninstall", visible_aliases = &["uni"])]
    Uninstall(commands::uninstall::Uninstall),
}

impl SubCommand {
    pub fn call(self, config: FnmConfig) {
        match self {
            Self::LsLocal(cmd) => cmd.call(config),
            Self::LsRemote(cmd) => cmd.call(config),
            Self::Install(cmd) => cmd.call(config),
            Self::Env(cmd) => cmd.call(config),
            Self::Use(cmd) => cmd.call(config),
            Self::Completions(cmd) => cmd.call(config),
            Self::Alias(cmd) => cmd.call(config),
            Self::Default(cmd) => cmd.call(config),
            Self::Current(cmd) => cmd.call(config),
            Self::Exec(cmd) => cmd.call(config),
            Self::Uninstall(cmd) => cmd.call(config),
            Self::Unalias(cmd) => cmd.call(config),
        }
    }
}

/// A fast and simple Node.js manager.
#[derive(clap::Parser, Debug)]
#[clap(name = "fnm", version = env!("CARGO_PKG_VERSION"), bin_name = "fnm")]
pub struct Cli {
    #[clap(flatten)]
    pub config: FnmConfig,
    #[command(subcommand)]
    pub subcmd: SubCommand,
}

pub fn parse() -> Cli {
    Cli::parse()
}
