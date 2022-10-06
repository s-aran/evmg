pub mod settings {
    pub struct Settings<'a> {
        pub no_args: bool,
        pub help: bool,
        pub dry_run: bool,
        pub verbose: u32,
        pub version: bool,
        pub export: Option<&'a str>,
        pub import: Option<&'a str>,
        pub set_variable: Option<(&'a str, &'a str)>,
        pub system: bool,

        #[cfg(target_os = "linux")]
        pub shell: Option<&'a str>,
    }
}
