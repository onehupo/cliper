// if run with src, use this
//      cargo run -- --input /Users/liangrui/Work/liangrui/cliper/build/app.apk --filter-type Res --filter-ext .png --filter-size 10000 --filter-path assets
// else use this
//      ./cliper detail --input ./build/app.apk --filter-type Res --filter-ext .png --filter-size 10000 --filter-path assets
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct CommonOpts {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    pub debug: bool,
    /// 输入文件 : --input ./build/app.apk
    #[structopt(long)]
    pub input: String,
    /// 输出csv文件: --output-csv
    #[structopt(short, long)]
    pub output_csv: bool,
    /// 输出文件 --build-path=./build 
    #[structopt(skip)]
    pub build_path: String,
}

#[derive(Debug, StructOpt)]
pub struct DetailOpts {
    #[structopt(long, default_value = "", help = "过滤路径 : --filter-path assets")]
    pub filter_path: String,
    #[structopt(long, default_value = "0", help = "过滤大小 : --filter-size 10000")]
    pub filter_size: u64,
    #[structopt(long, default_value = "", help = "过滤后缀 : --filter-ext .png")]
    pub filter_ext: String,
    #[structopt(long, default_value = "", help = "过滤类型 : --filter-type Code, Res, Native, Assets, Other")]
    pub filter_type: String,
    #[structopt(long, default_value = "", help = "过滤正则匹配 : --filter-regex \"^.*\\.png$\"")]
    pub filter_regex: String,
    #[structopt(long, default_value = "0", help = "限制输出行数 : --limit 10")]
    pub limit: usize,
}

/// 简介:
///
///     包体积文件工具
///     
///     1. 汇总包体积信息
///     2. 显示包体积详情，支持过滤
///     3. 查找相同文件
///
/// SAMPLE USAGE:
///     
///    ./cliper summary --input ./build/app.apk --output-csv
///
///    ./cliper detail --input ./build/app.apk --filter-ext .png --filter-size 10000 --filter-path assets
///
///    ./cliper detail --input=./build/app.apk --filter-type=Res --filter-ext=.png --filter-path=res/drawable
///
///    ./cliper same --input ./build/app.apk
///
/// HELP:
///
///     'cliper --help' for all commands
///
///     'cliper subcommands --help' for subcommands and options
///
#[derive(Debug, StructOpt)]
#[structopt(about = "the stupid content tracker")]
pub enum Args {
    /// Show the summary of the package volume
    Summary {
        #[structopt(flatten)]
        common: CommonOpts,
    },
    /// Show the detail of the package volume, support filter
    Detail {
        #[structopt(flatten)]
        common: CommonOpts,
        #[structopt(flatten)]
        detail: DetailOpts,
    },
    /// Show the same file in the package
    Same {
        #[structopt(flatten)]
        common: CommonOpts,
    }
}
