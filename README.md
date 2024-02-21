简介:

包体积文件工具

1. 汇总包体积信息 
2. 显示包体积详情，支持过滤 
3. 查找相同文件

SAMPLE USAGE:

./cliper summary --input ./build/app.apk --output-csv

./cliper detail --input ./build/app.apk --filter-ext .png --filter-size 10000 --filter-path assets

./cliper detail --input=./build/app.apk --filter-type=Res --filter-ext=.png --filter-path=res/drawable

./cliper same --input ./build/app.apk

HELP:

'cliper --help' for all commands

'cliper subcommands --help' for subcommands and options

USAGE:
    cliper <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    detail     Show the detail of the package volume, support filter
    help       Prints this message or the help of the given subcommand(s)
    same       Show the same file in the package
    summary    Show the summary of the package volume


install 

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh