use mason::generate_builder;

#[generate_builder(CommandBuilder)]
pub struct Command {
    #[builder(entrypoint, into, required)]
    program: String,

    args: Vec<String>,

    cwd: Option<String>,
}

/// Produces roughly this output:
///
/// ```
/// pub struct Command {
///     program: String,
///     args: Vec<String>,
///     cwd: Option<String>,
/// }
///
/// impl Command {
///     pub fn program(program: impl Into<String>) -> CommandBuilderWithProgram {
///         CommandBuilderWithProgram {
///             program: program.into(),
///             optionals: CommandBuilderOptional::default(),
///         }
///     }
/// }
///
/// #[derive(Default)]
/// pub struct CommandBuilderOptional {
///     args: Option<Vec<String>>,
///     cwd: Option<String>,
/// }
///
/// pub struct CommandBuilderWithProgram {
///     program: String,
///     optionals: CommandBuilderOptional,
/// }
///
/// impl CommandBuilderWithProgram {
///     pub fn args(mut self, args: Vec<String>) -> CommandBuilderWithProgram {
///         self.optionals.args = Some(args);
///         self
///     }
///
///     pub fn cwd(mut self, cwd: String) -> CommandBuilderWithProgram {
///         self.optionals.cwd = Some(cwd);
///         self
///     }
///
///     pub fn build(self) -> Command {
///         let CommandBuilderWithProgram { program, optionals } = self;
///         let CommandBuilderOptional { args, cwd } = optionals;
///         Command {
///             program: program,
///             args: args.unwrap_or_else(Default::default),
///             cwd: cwd,
///         }
///     }
/// }
/// ```
fn main() {
    let cmd = Command::program("echo")
        .args(vec!["Hello", "World!"])
        .build();
}
