use clap::{App, Arg};
pub const NAME_ARG: &str = "name";
pub const OUT_ARG: &str = "out";
pub const VERSION_ARG: &str = "version";
pub const EPOCH_ARG: &str = "epoch";
pub const LICENSE_ARG: &str = "license";
pub const ARCH_ARG: &str = "arch";
pub const RELEASE_ARG: &str = "release";
pub const DESC_ARG: &str = "desc";
pub const FILE_ARG: &str = "file";
pub const EXEC_FILE_ARG: &str = "exec-file";
pub const DOC_FILE_ARG: &str = "doc-file";
pub const CONFIG_FILE_ARG: &str = "config-file";
pub const DIR_ARG: &str = "dir";
pub const COMPRESSION_ARG: &str = "compression";
pub const CHANGELOG_ARG: &str = "changelog";
pub const REQUIRES_ARG: &str = "requires";
pub const OBSOLETES_ARG: &str = "obsoletes";
pub const PROVIDES_ARG: &str = "provides";
pub const CONFLICTS_ARG: &str = "conflicts";
pub const PRE_INSTALL_SCRIPTLET_ARG: &str = "pre-install-script";
pub const POST_INSTALL_SCRIPTLET_ARG: &str = "post-install-script";
pub const PRE_UNINSTALL_SCRIPTLET_ARG: &str = "pre-uninstall-script";
pub const POST_UNINSTALL_SCRIPTLET_ARG: &str = "post-uninstall-script";
pub const SIGN_WITH_PGP_ASC_ARG: &str = "sign-with-pgp-asc";
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn build_cli() -> App<'static, 'static> {
    let supported_compression_options = ["gzip","zstd", "none"];
    App::new("rpm-builder")
          .version(VERSION)
          .author("René R. <richterrettich@gmail.com>")
          .about("Build rpms with ease")
          .arg(Arg::with_name(OUT_ARG)
               .long(OUT_ARG)
               .short("o")
               .value_name("OUT")
               .help("Specify an out file")
               .takes_value(true))
          .arg(Arg::with_name(VERSION_ARG)
               .long(VERSION_ARG)
               .value_name("VERSION")
               .help("Specify a version")
               .default_value("1.0.0")
               .takes_value(true))
          .arg(Arg::with_name(EPOCH_ARG)
               .long(EPOCH_ARG)
               .value_name("EPOCH")
               .help("Specify an epoch")
               .default_value("0")
               .takes_value(true))
          .arg(Arg::with_name(LICENSE_ARG)
               .long(LICENSE_ARG)
               .value_name("LICENSE")
               .help("Specify a license")
               .default_value("MIT")
               .takes_value(true))
          .arg(Arg::with_name(ARCH_ARG)
               .long(ARCH_ARG)
               .value_name("ARCH")
               .help("Specify the target architecture")
               .default_value("x86_64")
               .takes_value(true))
          .arg(Arg::with_name(RELEASE_ARG)
               .long(RELEASE_ARG)
               .value_name("RELEASE")
               .help("Specify release number of the package")
               .default_value("1")
               .takes_value(true))
          .arg(Arg::with_name(DESC_ARG)
               .long(DESC_ARG)
               .value_name("DESC")
               .help("Give a description of the package")
               .default_value("")
               .takes_value(true))
          .arg(Arg::with_name(FILE_ARG)
               .long("file")
               .value_name("FILE")
               .help("add a regular file to the rpm")
               .takes_value(true)
               .multiple(true)
               .number_of_values(1))
          .arg(Arg::with_name(EXEC_FILE_ARG)
               .long("exec-file")
               .value_name("EXEC_FILE")
               .help("add a executable-file to the rpm")
               .takes_value(true)
               .multiple(true)
               .number_of_values(1))
          .arg(Arg::with_name(DOC_FILE_ARG)
               .long("doc-file")
               .value_name("DOC_FILE")
               .help("add a documentation-file to the rpm")
               .takes_value(true)
               .multiple(true)
               .number_of_values(1))
          .arg(Arg::with_name(CONFIG_FILE_ARG)
               .long("config-file")
               .value_name("CONFIG_FILE")
               .help("add a config-file to the rpm")
               .takes_value(true)
               .multiple(true)
               .number_of_values(1))
          .arg(Arg::with_name(DIR_ARG)
               .long("dir")
               .value_name("DIR")
               .help("add a directory and all its files to the rpm")
               .takes_value(true)
               .multiple(true)
               .number_of_values(1))
          .arg(Arg::with_name(COMPRESSION_ARG)
               .long("compression")
               .value_name("COMPRESSION")
               .help("specify the compression algorithm. Currently only gzip and zstd are supported")
               .takes_value(true)
               .default_value("none")
               .multiple(true)
               .possible_values(&supported_compression_options)
               .number_of_values(1))
          .arg(Arg::with_name(CHANGELOG_ARG)
               .long("changelog")
               .value_name("CHANGELOG_ENTRY")
               .help("add a changelog entry to the rpm. The entry has the form <author>:<content>:<yyyy-mm-dd> (time is in utc)")
               .takes_value(true)
               .multiple(true)
               .number_of_values(1))
          .arg(Arg::with_name(REQUIRES_ARG)
               .long("requires")
               .value_name("REQUIRES")
               .help("indicates that the rpm requires another package. Use the format '<name> [>|>=|=|<=|< version]'")
               .takes_value(true)
               .multiple(true)
               .number_of_values(1))
          .arg(Arg::with_name(PROVIDES_ARG)
               .long("provides")
               .value_name("PROVIDES")
               .help("indicates that the rpm provides another package. Use the format '<name> [>|>=|=|<=|< version]'")
               .takes_value(true)
               .multiple(true)
               .number_of_values(1))
          .arg(Arg::with_name(OBSOLETES_ARG)
               .long("obsoletes")
               .value_name("OBSOLETES")
               .help("indicates that the rpm obsoletes another package. Use the format '<name> [>|>=|=|<=|< version]'")
               .takes_value(true)
               .multiple(true)
               .number_of_values(1))
          .arg(Arg::with_name(CONFLICTS_ARG)
               .long("conflicts")
               .value_name("CONFLICTS")
               .help("indicates that the rpm conflicts with another package. Use the format '<name> [>|>=|=|<=|< version]'")
               .takes_value(true)
               .multiple(true)
               .number_of_values(1))
          .arg(Arg::with_name(PRE_INSTALL_SCRIPTLET_ARG)
               .long("pre-install-script")
               .value_name("PREINSTALLSCRIPT")
               .help("path to a file that contains the pre installation script")
               .takes_value(true)
               .multiple(false)
               .number_of_values(1))
          .arg(Arg::with_name(POST_INSTALL_SCRIPTLET_ARG)
               .long("post-install-script")
               .value_name("POSTINSTALLSCRIPT")
               .help("path to a file that contains the post installation script")
               .takes_value(true)
               .multiple(false)
               .number_of_values(1))
          .arg(Arg::with_name(PRE_UNINSTALL_SCRIPTLET_ARG)
               .long("pre-uninstall-script")
               .value_name("PRE_UNINSTALL_SCRIPT")
               .help("path to a file that contains a pre uninstall script")
               .takes_value(true)
               .multiple(false)
               .number_of_values(1))
          .arg(Arg::with_name(POST_UNINSTALL_SCRIPTLET_ARG)
               .long("post-uninstall-script")
               .value_name("POST_UNINSTALL_SCRIPT")
               .help("path to a file that contains a post uninstall script")
               .takes_value(true)
               .multiple(false)
               .number_of_values(1))
          .arg(Arg::with_name(NAME_ARG)
               .help("Specify the name of your package")
               .required(true))
          .arg(Arg::with_name(SIGN_WITH_PGP_ASC_ARG)
               .long("sign-with-pgp-asc")
               .value_name("SIGN_WITH_PGP_ASC")
               .takes_value(true)
               .number_of_values(1)
               .help("sign this package with the specified pgp secret key"))
}
