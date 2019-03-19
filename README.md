# Substrate Package

安定動作が確認されているSubstrateです。
チュートリアルで最初に見かける`curl https://getsubstrate.io -sSf | bash`は最新のバージョンを取得するため、予期せぬエラーが起こりやすです。初心者の方や安定バージョンを使いたい方はこのレポを使ってください。</br>

[Substrate Node Template](https://github.com/paritytech/substrate/tree/master/node-template) </br>
[Substrate UI](https://github.com/paritytech/substrate-ui) 

# 使い方:
 * `git clone https://github.com/masaki1223/substrate-package/`でクローン。
 
 * substrate-packageに移動後、 `./get-substrate-dependencies.sh`を実行。
    * 必要な外部ディペンデンシーをインストールします。
    * This installs some external dependencies, take a look at the script
    * Windowsの場合は [こちら](https://github.com/paritytech/substrate#61-hacking-on-substrate) を参照。

 * `./substrate-package-rename.sh <プロジェクト名> <Auth名>`を実行。
    * プロジェクトフォルダーとランタイムをコンパイルしたときに作られるバイナリファイルの名前変更をします。

 *  `<プロジェクト名>` フォルダに移動後、以下を実行:
    * `./build.sh`　Wasmのビルド
    * `cargo build --release`　バイナリのビルド
    * `./target/release/<プロジェクト名> purge-chain --dev`　チェーンのデータを全消去（しておくとエラーが起こりにくい）
    * `./target/release/<プロジェクト名> --dev`　チェーンの初期化とノード起動。ブロックが自動生成され始めます。

 * `<プロジェクト名>` フォルダに移動後、以下を実行:
    * `yarn install`
    * `yarn run dev`
    * `localhost:8000` でノードを触れるローカルサーバーが立ち上がります。
 
 * 代替UIアプリ [PolkadotAppUI](https://polkadot.js.org/apps/#/transfer)
    * 左タブのSettings> `remote node/endpoint to connect to`を `Local Node`に設定。
    *  `Save&Reload`ボタンを押して設定を適応＆更新。
   

# What is this?

* Compatible with the latest documentation available for Substrate Runtime Module development.
* The fastest way to get started building on substrate
* Using Substrate commit: [82744fbb6f4d677f2edfe9d88737c237622c97a4](https://github.com/paritytech/substrate/commit/82744fbb6f4d677f2edfe9d88737c237622c97a4)
* Using Substrate UI commit: [e32e874b177effe9b789699282f11dd63b783398](https://github.com/paritytech/substrate-ui/commit/e32e874b177effe9b789699282f11dd63b783398)

# How was it made?

`substrate-node-template` was created by running `substrate-node-new` and resetting some of the "naming" values to default.

`substrate-ui` was created by running `substrate-ui-new`.

# What it tested to work?

* `substrate-node-template` is fully compatible with `substrate-ui` included with this package:
    * Balance Transfer
    * Runtime Upgrades
    * Creating new UI elements for new modules
* Gav's Web3 Summit Demo
* Substratekitties Tutorial
* Substrate Documentation

# Why do I need `substrate-package`?

Substrate is a rapidly evolving platform, which means that breaking changes may occur on a day to day basis.
Most of the times, these breaking changes do not radically change how substrate works, but may affect how Substrate is organized, the name of functions, the name of modules, etc...

If you try to create a new `substrate-node-template` with the `substrate-node-new` command, you will pull the latest version of substrate which may not be compatible with different parts of the development ecosystem, such as:

* Documentation
* Tutorials
* Samples
* User Interfaces
* etc...

The `substrate-package` repository tries to help solve these problems by taking a snapshot of `substrate` when it is known to be working and compatible with these different resources.

Most of these issues should go away once Substrate v1.0 is released and we have a stable API.
