use structopt::StructOpt;

// Common options for the `cliper` tool.
#[derive(Debug, StructOpt)]
pub struct CommonOpts {
    /// Enable debug mode. Use `-d` or `--debug` to activate.
    #[structopt(short, long)]
    pub debug: bool,
    
    /// Specify the input file path. Example: `--input ./build/app.apk`.
    #[structopt(long)]
    pub input: String,
    
    /// Enable output in CSV format. Use `--output-csv` to activate.
    #[structopt(short, long)]
    pub output_csv: bool,
    
    /// Specify the build path. This option is skipped by default.
    #[structopt(skip)]
    pub build_path: String,
}

// Options for displaying detailed package volume information.
#[derive(Debug, StructOpt)]
pub struct DetailOpts {
    /// Filter by path within the package. Example: `--filter-path assets`.
    #[structopt(long, default_value = "", help = "Filter by path. Example: `--filter-path assets`.")]
    pub filter_path: String,
    
    /// Filter by minimum file size in bytes. Example: `--filter-size 10000`.
    #[structopt(long, default_value = "0", help = "Filter by minimum file size in bytes. Example: `--filter-size 10000`.")]
    pub filter_size: u64,
    
    /// Filter by file extension. Example: `--filter-ext .png`.
    #[structopt(long, default_value = "", help = "Filter by file extension. Example: `--filter-ext .png`.")]
    pub filter_ext: String,
    
    /// Filter by type such as Code, Res, Native, Assets, Other. Example: `--filter-type Res`.
    #[structopt(long, default_value = "", help = "Filter by type (Code, Res, Native, Assets, Other). Example: `--filter-type Res`.")]
    pub filter_type: String,
    
    /// Filter using a regular expression pattern. Example: `--filter-regex "^.*\\.png$"`.
    #[structopt(long, default_value = "", help = "Filter using a regular expression pattern. Example: `--filter-regex \"^.*\\.png$\"`.")]
    pub filter_regex: String,
    
    /// Limit the number of output lines. Example: `--limit 10`.
    #[structopt(long, default_value = "0", help = "Limit the number of output lines. Example: `--limit 10`.")]
    pub limit: usize,
}

/// Cliper: A package volume analysis tool.
///
/// This tool helps you analyze the contents of your app's package file (APK, AAB, etc.).
/// It provides a summary of the package volume, detailed volume information with
/// filter options, and identifies duplicate files within the package.
///
/// Basic Usage Examples:
///
/// To get a summary with CSV output:
///     `./cliper summary --input ./build/app.apk --output-csv`
///
/// To get detailed information filtered by extension and size:
///     `./cliper detail --input ./build/app.apk --filter-ext .png --filter-size 10000 --filter-path assets`
///
/// To find duplicate files:
///     `./cliper same --input ./build/app.apk`
/// 
/// To diff two package:
///     `./cliper diff --input ./build/app.apk --input-cmp ./build/app2.apk`
///     note: the input-cmp is the old package file path.
///
/// Getting Help:
///
/// For a list of all commands and options, use:
///     `cliper --help`
///
/// For help with subcommands, use:
///     `cliper <subcommand> --help`
///
#[derive(Debug, StructOpt)]
#[structopt(about = "A tool for analyzing package volume.")]
pub enum Args {
    /// Summarize the package volume.
    Summary {
        #[structopt(flatten)]
        common: CommonOpts,
    },
    /// Display detailed package volume information with filter options.
    Detail {
        #[structopt(flatten)]
        common: CommonOpts,
        #[structopt(flatten)]
        detail: DetailOpts,
    },
    /// Identify duplicate files within the package.
    Same {
        #[structopt(flatten)]
        common: CommonOpts,
    },
    /// Display package information such as package name, version code, and version name.
    Info {
        #[structopt(flatten)]
        common: CommonOpts,
    },
    /// Compare two package files and display the differences.
    Diff {
        #[structopt(flatten)]
        common: CommonOpts,
        /// Specify the second input file path. Example: `--input_cmp ./build/app2.apk`.
        #[structopt(long)]
        input_cmp: String,
    },
}