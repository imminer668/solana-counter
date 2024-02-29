# Solana Counter Program

一个基于 Solana 区块链的简单计数器智能合约程序。该程序允许用户在链上创建一个计数器，并且可以递增计数器的值。

## 功能特性

- 创建计数器账户
- 递增计数器值
- 读取当前计数值
- 完整的账户权限验证
- 数据序列化和反序列化

## 技术栈

- Rust 编程语言
- Solana Program Library (SPL)
- Borsh 序列化
- Solana Web3.js (客户端交互)

## 项目结构 
solana-counter/
├── src/
│ └── lib.rs # 智能合约主程序代码
├── Cargo.toml # Rust 项目配置文件
├── Cargo.lock # 依赖版本锁定文件
└── README.md # 项目文档

## 智能合约功能模块

### 数据结构
- `Counter`: 存储计数器值的结构体
  - `value`: u32 类型，表示当前计数值

### 指令处理
- `process_instruction`: 处理程序指令
  - 验证账户权限
  - 读取当前计数值
  - 递增计数值
  - 保存更新后的数据

## 开发环境设置

1. 安装 Rust 和 Solana CLI
```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/v1.16.0/install)"

# 添加 Rust BPF 工具链
rustup component add rust-src
```

2. 安装 Solana 工具
```bash
# 更新 Solana 到指定版本
solana --version  # 检查当前版本
solana-install update v1.16.0  # 更新到指定版本

# 安装 Solana BPF 工具链
sh -c "$(curl -sSfL https://release.solana.com/v1.16.0/install)" --only-rust-bpf
```

3. 克隆项目
```bash
git clone <repository-url>
cd solana-counter
```

4. 构建项目
```bash
cargo build-sbf    # 注意：使用 build-sbf 而不是 build-bpf 
```
由于网络问题，本地构建可能很慢慢，考虑使用 Solana Playground 进行快速开发和测试，等代码确定后再在本地构建发布。

## 部署说明

1. 启动本地测试验证节点（可选）
```bash
solana-test-validator
```

2. 部署程序
```bash
solana program deploy target/deploy/solana_counter.so
```

## 测试

1. 运行单元测试
```bash
cargo test
```

2. 在测试网进行测试
```bash
solana config set --url devnet
solana airdrop 2 # 获取测试代币
solana program deploy target/deploy/solana_counter.so
```

## 安全考虑

- 程序实现了完整的账户所有权验证
- 确保账户数据可写性检查
- 防止未经授权的数据修改

## 性能优化

- 使用 Borsh 进行高效的数据序列化
- 最小化存储空间使用
- 优化指令处理逻辑

## 贡献指南

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 开启一个 Pull Request

## 许可证

[MIT](LICENSE)

## 作者

- 0xF1

## 更新日志

### [0.1.0] - 2024-02-16
- 初始版本发布
- 实现基本计数器功能
- 添加账户验证逻辑