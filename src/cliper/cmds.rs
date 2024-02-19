use structopt::StructOpt;

    // cargo run -- --input /Users/liangrui/Work/liangrui/cliper/build/app.apk --filter-type Res --filter-ext .png --filter-size 10000 --filter-path assets
    // cargo run -- --input ./build/app.apk --filter-type Res --filter-ext .png --filter-size 10000 --filter-path assets
    
    /// Common options for the command line interface
    #[derive(Debug, StructOpt)]
    pub struct CommonOpts {
        /// Activate debug mode
        // short and long flags (-d, --debug) will be deduced from the field's name
        #[structopt(short, long)]
        pub debug: bool,
        /// 输入文件
        #[structopt(long)]
        pub input: String,
        /// 输出csv文件
        #[structopt(short, long, help = "输出csv文件")]
        pub output_csv: bool,
        #[structopt(skip)]
        pub build_path: String,
    }
    
    #[derive(Debug, StructOpt)]
    pub struct DetailOpts {
        #[structopt(long, default_value = "", help = "过滤路径")]
        pub filter_path: String,
        #[structopt(long, default_value = "0", help = "过滤大小")]
        pub filter_size: u64,
        #[structopt(long, default_value = "", help = "过滤后缀")]
        pub filter_ext: String,
        #[structopt(long, default_value = "", help = "过滤类型")]
        pub filter_type: String,
        #[structopt(long, default_value = "0", help = "限制输出行数")]
        pub limit: usize,
    }
    
    /// 简介:
    /// 
    ///     一个简单的包体积分析工具，可以分析apk包的大小，包含的文件，文件大小，文件类型等信息
    /// 
    /// 用法:
    ///     
    ///    cliper --input ./build/app.apk --filter-type Res --filter-ext .png --filter-size 10000 --filter-path assets
    /// 
    /// 帮助:
    /// 
    ///     'cliper --help' for all commands
    /// 
    ///     'cliper summary --help' for subcommands and options
    /// 
    ///     'cliper detail --help' for subcommands and options
    /// 
    #[derive(Debug, StructOpt)]
    #[structopt(about = "the stupid content tracker")]
    pub enum Args {
        Summary {
            #[structopt(flatten)]
            common: CommonOpts
        },
        Detail {
            #[structopt(flatten)]
            common: CommonOpts,
            #[structopt(flatten)]
            detail: DetailOpts,
        },
    }