# hello-solana

## 环境要求

### 安装 Rust 1.89.0

### 安装 Solana CLI 2.2.12

```shell
sh -c "$(curl -sSfL https://release.anza.xyz/v2.2.12/install)"

export PATH="/root/.local/share/solana/install/active_release/bin:$PATH"

```

### 安装 Anchor CLI 0.32.1

通过 AVM 安装

```shell
cargo install --git https://github.com/solana-foundation/anchor avm --force

avm --version

avm install 0.32.1
```

### 安装 Node 项目依赖

```shell
fnm use 22

npm install --global yarn
````

## 初始化一个项目

```bash
anchor init hello-solana --no-git
```

## anchor 相关指令

### 重新生成程序ID

```bash
rm target/deploy/xxx-keypair.json 
anchor keys sync
```

### 编译和构建

```bash
anchor build
```

### 测试

运行测试默认会进行 build 和 deploy，可以指定 `--skip-deploy` 和 `--skip-build` 参数。

```bash
anchor test

anchor test --provider.cluster http://127.0.0.1:8899

anchor test --skip-deploy --skip-build --provider.cluster http://127.0.0.1:8899
```

### 清理

```bash
anchor clean
```

### 部署到不同网络

```bash
# 部署到本地网络
anchor deploy

# 部署到特定网络（如 devnet）
anchor deploy --provider.cluster devnet

# 部署到特定网络（如 devnet）
anchor deploy --provider.cluster http://127.0.0.1:8899
```

## 部署花费

部署花费总费用 = 程序空间租金 + Buffer 空间租金 + 交易费用

程序空间租金 ≈ (程序大小 * 每字节租金 * 2年)

Buffer 空间租金是临时的，部署后会返还，查看程序大小

```bash
ls -l target/deploy/batch_send.so
```

验证部署花费租金(字节大小)：

```bash
solana rent 199080
```

测试 devnet 总交易 GAS 评估约 0.0006 = 0.1U

## 关闭程序

```bash
# 1. 首先确保你有程序的升级权限（你是程序的升级权限持有者）
solana program show <program_id>

# 2. 关闭程序并收回 SOL
solana program close <program_id> --bypass-warning
```

> 注意：一旦关闭，程序ID将不再可用，无法恢复。


## 错误记录

### feature `edition2024` is required

feature `edition2024` is required

### 1. build 时提示找不到  platform

```bash
Failed to install platform-tools: HTTP status client error (404 Not Found) for url (https://github.com/anza-xyz/platform-tools/releases/download/v1.42/platform-tools-osx-x86_64.tar.bz2)
```

解决：`agave-install init 1.18.21` 再：`anchor build`


### 2. build 时提示 lock file version 4 requires `-Znext-lockfile-bump`

解决：修改 `Cargo.lock`，将版本号 version = 3


### 3. build 时一堆 warning: unexpected `cfg` condition value: `solana` 等警告

解决：不知道咋解决，先不管它

### 4. 部署时提示错误需要 Recover

```bash
Deploying cluster: https://devnet-rpc.shyft.to?api_key=m4T8LXSoiymOiiHM
Upgrade authority: /Users/ncq/.config/solana/id.json
Deploying program "batch_send"...
Program path: /Users/ncq/Repos/Solana/solana-programs/batch-send/target/deploy/batch_send.so...
==========================================================================
Recover the intermediate account's ephemeral keypair file with
`solana-keygen recover` and the following 12-word seed phrase:
To resume a deploy, pass the recovered keypair as the
[BUFFER_SIGNER] to `solana program deploy` or `solana program write-buffer'.
Or to recover the account's lamports, pass it as the
[BUFFER_ACCOUNT_ADDRESS] argument to `solana program close`.
==========================================================================
```

solana-keygen recover -o recover.json


solana program deploy --buffer recover.json \
--upgrade-authority ~/.config/solana/id.json \
--program-id ./target/deploy/batch_send-keypair.json ./target/deploy/batch_send.so
