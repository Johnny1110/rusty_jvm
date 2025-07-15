# rusty_jvm

JVM implemented by Rust (for study JVM)

<br>

---

<br>

## 專案結構建立

```
rusty-jvm/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── classfile/
│   │   ├── mod.rs
│   │   ├── parser.rs
│   │   ├── constant_pool.rs
│   │   └── attributes.rs
│   ├── runtime/
│   │   ├── mod.rs
│   │   ├── heap.rs
│   │   ├── method_area.rs
│   │   ├── stack.rs
│   │   └── frame.rs
│   ├── classloader/
│   │   ├── mod.rs
│   │   └── bootstrap.rs
│   ├── interpreter/
│   │   ├── mod.rs
│   │   ├── instructions/
│   │   └── bytecode.rs
│   └── gc/
│       ├── mod.rs
│       └── mark_sweep.rs
├── tests/
│   ├── test_classes/
│   └── integration_tests.rs
└── examples/
    └── simple_programs/

```

---

<br>

## 階段性任務

### 階段 1: 基礎設施

* 1.1 專案骨架建立

  * 創建 Cargo.toml 配置文件
  * 設置基本的模組結構
  * 建立錯誤處理體系 (使用 thiserror crate)
  * 設置日誌系統 (使用 log + env_logger)

<br>

* 1.2 Class 文件格式理解

  * 研讀 JVM 規範中的 Class 文件格式
  * 使用 javap -v 分析簡單的 .class 文件
  * 理解常量池、字段、方法、屬性等結構

<br>

* 1.3 Class 文件解析器 - 第一版

  * 實現 ClassFile 結構體
  * 實現常量池解析 (ConstantPool)
  * 實現基本的二進制讀取工具
  * 能夠解析最簡單的 HelloWorld.class
 
  <br>

里程碑: 能夠解析並印出 HelloWorld.class 的基本信息

<br>
<br>

### 階段 2: 運行時數據區


* 2.1 內存區域設計

  * 實現方法區 (Method Area)
  * 實現堆 (Heap) - 簡單版本
  * 實現虛擬機棧 (VM Stack)
  * 實現 PC 寄存器

* 2.2 對象模型

  * 設計 Java 對象在 Rust 中的表示
  * 實現 Klass (類的運行時表示)
  * 實現 Instance (對象實例)
  * 實現基本的內存分配機制

* 2.3 棧幀實現

  * 實現 Frame 結構
  * 實現局部變量表 (Local Variable Table)
  * 實現操作數棧 (Operand Stack)
  * 實現方法調用和返回機制

__里程碑: 可以創建對象和調用空方法__

<br>
<br>

###  階段 3: 類加載器

* 3.1 類加載機制

  * 實現 Bootstrap ClassLoader
  * 實現類的加載、鏈接、初始化過程
  * 實現類的符號引用解析
  * 處理類的繼承關係

* 3.2 基本類型和數組

  * 實現原始類型的處理
  * 實現數組類型的支持
  * 實現 String 類的基本支持

__里程碑: 能夠加載並實例化簡單的 Java 類__

<br>
<br>

### 階段 4: 字節碼解釋器

* 4.1 指令集實現 - 第一批

  * 常量指令 (aconst_null, iconst_0 等)
  * 加載指令 (aload, iload 等)
  * 存儲指令 (astore, istore 等)
  * 算術指令 (iadd, isub 等)

* 4.2 指令集實現 - 第二批

  * 對象操作指令 (new, getfield, putfield)
  * 數組操作指令 (newarray, arraylength)
  * 方法調用指令 (invokevirtual, invokespecial)
  * 控制流指令 (if_icmpeq, goto 等)

* 4.3 解釋器引擎

  * 實現指令分派機制
  * 實現執行循環
  * 添加調試和跟踪功能

__里程碑: 能夠運行包含基本算術和方法調用的 Java 程序__

<br>
<br>

### 階段 5: 核心功能完善

* 5.1 方法調用完善

  * 實現虛函數調用
  * 實現靜態方法調用
  * 實現構造函數調用
  * 處理方法重載和重寫

* 5.2 異常處理

  * 實現異常表解析
  * 實現 throw/catch 機制
  * 實現基本的異常類層次

* 5.3 標準庫支持

  * 實現 System.out.println 的 native 方法
  * 實現基本的 String 操作
  * 實現簡單的 I/O 操作

__里程碑: 能夠運行複雜的 Java 程序，包括異常處理__

<br>
<br>

### 階段 6: 垃圾回收器

* 6.1 GC 基礎設施

  * 實現對象的可達性分析
  * 實現根對象的識別
  * 設計 GC 的觸發條件

* 6.2 標記-清除收集器

  * 實現標記階段
  * 實現清除階段
  * 處理內存碎片問題

* 6.3 GC 優化

  * 實現分代收集的基礎
  * 添加 GC 性能監控
  * 實現內存壓縮

__里程碑: 具備基本的自動內存管理能力__

<br>
<br>
<br>
<br>

---

## 測試計劃


* 單元測試

  * 每個模組都要有對應的單元測試
  * 使用 cargo test 確保代碼質量

* 集成測試

  * HelloWorld.java
  * 算術運算測試
  * 對象創建和方法調用測試
  * 數組操作測試
  * 異常處理測試
  * 垃圾回收測試

* 性能測試

  * 簡單的 Fibonacci 計算
  * 對象創建和銷毀性能
  * GC 性能測試
 
  <br>
  <br>
  <br>
  <br>

  ---

  <br>

## 依賴套件

```
toml[dependencies]
thiserror = "1.0"
log = "0.4"
env_logger = "0.10"
byteorder = "1.4"
clap = "4.0"  # 命令行參數解析

[dev-dependencies]
tempfile = "3.0"  # 測試時創建臨時文件
```

  <br>
  <br>
  <br>
  <br>

  ---

  <br>

## 開發工具和資源

* 調試工具

  * 使用 javap -v 分析字節碼
  * 使用 hexdump 查看二進制文件
  * 實現自己的字節碼反彙編器
    

* 參考資料
  
  * 《深入理解Java虛擬機》
