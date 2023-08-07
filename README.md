# filewalker

迭代访问文件夹和子文件夹中的文件。

## 用法

```rust
let path = "/etc";
let walker = FileWalker::open(Path::new(path))?;

for file in walker.take(50) {
    if let Ok(file_path) = file {
        println!("{}", file_path.display());
    }
}
```
