分析Android包体积的工具


TODO: 生成数据：

1. 为了能够生成tree的图标数据，所以需要更多的原始文件大小的信息

路径，名称，压缩大小，原始大小，分类，文件类型，文件夹的路径

done

2. 可以分出去的类别

展示所有

展示某个类别：代码 ｜ Asserts | Res  | Code | Native | Others ｜

展示某个文件类型： png...

展示某个文件夹：文件夹的路径

done

3. 生成各种文件

csv等文件存储

done

4. 如何区分文件是否是新增的文件

对比两个apk的数据是否有差异

删除文件
新增文件
差异的文件

```rust
cargo run -- --input ./build/app.apk --filter-type Res --filter-ext .png --filter-size 10000 --filter-path assets
```