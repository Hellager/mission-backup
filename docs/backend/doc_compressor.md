# compressor.rs

## 功能

创建压缩文件/文件夹，目前支持 zip/tar/tar.gz/tar.bz2/tar.xz/bz2



## 函数



### create_archive

- params: 
  - src_path: &str
  - dst_path: &str
  - format: &str
- return
  - bool

根据所给路径和格式创建压缩文件/文件夹

### zip_dir

- params
  - it: &mut dyn Iterator<Item = DirEntry>
  - prefix: &str
  - writer: T
  - method: zip::CompressionMethod
- return
  - zip::result::ZipResult<()>

遍历压缩为 zip 格式

### create_zip_archive

- params
  - src_path: &str
  - dst_path: &str
- return
  - bool

压缩为 zip

### create_tar_archive

- params
  - src_path: &str
  - dst_path: &str
- return
  - bool

压缩为 tar

### create_bz2_archive

- params
  - src_path: &str
  - dst_path: &str
- return
  - bool

压缩为 bz2

### create_tar_gz_archive

- params
  - src_path: &str
  - dst_path: &str
- return
  - bool

压缩为 tar.gz



### create_tar_bz2_archive

- params
  - src_path: &str
  - dst_path: &str
- return
  - bool

压缩为 tar.bz2



### create_tar_xz_archive

- params
  - src_path: &str
  - dst_path: &str
- return
  - bool

压缩为 tar.xz