# collector.rs

## 功能

统计任务保存目录中的备份情况，包括备份数和所占空间大小



## 函数

### get_one_day_analyse

- params
  - path: &str
  - date: &str
  - individual: bool
- return
  - HashMap<String, Vec\<u64>>

获取一天的统计情况，date 格式应为 yyyy-mm-dd，individual 决定是否各时段分开计数或累计

### get_one_week_analyse

- params
  - path: &str
  - start_date: &str
  - individual: bool
- return
  - HashMap<String, Vec\<u64>>

获取一周的统计情况，start_date 格式应为 yyyy-mm-dd，individual 决定是否各时段分开计数或累计

### get_days_from_month

- params
  - year: i32
  - month: u32
- return
  - i64

获取月份对应的天数

### get_one_month_analyse

- params
  - path: &str
  - month: &str
  - individual: bool
- return
  - HashMap<String, Vec\<u64>>

获取一月的统计情况，month 格式应为 yyyy-mm，individual 决定是否各时段分开计数或累计

### get_one_year_analyse

- params
  - path: &str
  - year: &str
  - individual: bool
- return
  - HashMap<String, Vec\<u64>>

获取一年的统计情况，month 格式应为 yyyy，individual 决定是否各时段分开计数或累计

### convert_count_size

- params
  - origin_cout_arr: Vec\<u64>
- return
  - Vec\<f64>

转换备份数单位

### convert_size_unit

- params
  - origin_size_arr: Vec\<u64>
  - size_unit: &str
- return
  - Vec\<f64>

转换备份空间大小单位

### count_size_with_path

- params
  - path: &str
  - query_type: &str
  - start: &str
  - size_unit: &str
- return
  - HashMap<String, Vec\<f64>>

获取路径对应统计结果