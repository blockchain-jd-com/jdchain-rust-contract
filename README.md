JD Chain Rust 合约规范及示例代码

1. 安装 Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. 安装 wasm-pack
```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

3. 创建合约项目
```bash
wasm-pack new jdchain-contract
```

或者直接基于本项目修改

4. 修改`sample_contract.rs`

按具体业务需求组合`sample_contract.rs`中的模板方法

5. 编译
```bash
wasm-pack build .
```

6. 合约部署
```bash
./jdchain-cli.sh tx contract-deploy --code /jdchain-rust-contract/pkg/jdchain_rust_contract_bg.wasm --lang Rust --pubkey 7VeRG8jpBNg15W7HCrFyLG7TdpUea5jnHAUDbmxAkK6ZYqu4
```

7. 合约调用
```bash
./jdchain-cli.sh tx contract --address LdeNgGn7tPYXNi4vAhXN57qAYtb57NvAUDvvg --method get_ledger_hash
```