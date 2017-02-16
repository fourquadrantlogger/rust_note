# idea 插件

今天我们将使用 intellij-rust 这款插件来构建我们的Rust开发环境。

### 环境初始化

1. 安装[Intellij IDEA](https://www.jetbrains.com/idea/)
2. 安装[intellij-rust](https://github.com/intellij-rust/intellij-rust) 
    1. $  git clone https://github.com/intellij-rust/intellij-rust 
    2. $  cd intellij-rust 
    3. $ ./gradlew buildPlugin
    4. 启动[Intellij IDEA](https://www.jetbrains.com/idea/)
    5. 进入以下对话框：
        1.  File->Setting->Plugins->Install plugin from disk
        2.  选择 intellij-rust/build/distributions/xxxx.zip
        3. 安装成功后将根据提示重新启动IDE
    6. [intellij-rust](https://github.com/intellij-rust/intellij-rust)  插件安装完毕
环境配置

我们在安装完毕 intellij-rust 之后，已经可以创建rs源文件了，并且可以享受其智能提示等功能，但是，当前的版本中尚未直接集成编译、调试等命令。接下来，我们将构建Rust的自动构建系统。

1. 进入以下对话框:
    1. File->Setting->Tools->External Tools
    2. 点击对话框的左下方+号添加一个自定义框架命令。
    3. 我提供了一组构建工具的配置方案

Name      |	Description     |		Program	Parameters  |		Working Directory
--------- | --------------- | ------------------------- | -------------------------
Rustc     |run with rustc	|	/usr/local/bin/rustc    |	\$FileName\$	\$FileDir\$
CargoRun  |	cargo run	    |	/usr/local/bin/cargo	|	run	\$ProjectFileDir\$
CargoBuild|		cargo build |		/usr/local/bin/cargo|		build	\$ProjectFileDir\$
CargoTest |		cargo test  |  /usr/local/bin/cargo	test|		\$ProjectFileDir\$
CargoFmt  |		cargo fmt	|	/usr/local/bin/cargo	|	fmt	\$ProjectFileDir\$
 

以下命令将会在Tools->External Tools 下面以列表的形式进行显示。 配置的更多参考可以阅读官方文档

快捷键配置

我们可以进入以下对话框，完成上述我们定义的自定义命令的快捷键配置：

1. File->Setting->keymap 
总结

我们已经完成了基于 Intellij IDEA 的Rust开发环境构建，现在你就可以享受其愉悦的体验了。

意外之喜

由于众说周知的原因，我朝的互联网在访问某些非我国的网络节点的时候，不总是那么顺畅，很遗憾的是，cargo包管理器的源地址就在其列表之列。事实上，carg的源码包托管在amazon s3 的美国节点上，部分github包也是托管在其之上。细节不再表述。

当你在使用cargo的时候，因为网络的原因导致体验不那么顺畅的时候，我们可以寻求解决方案。

1. VPN
2. Sh***Socks                                
如果你使用的是Sh***Socks，可以使用 proxychains ，以使得 Sh***Socks可以支持终端使用。为了使用 proxychains ,我们可以将我们的自定义命令做如下变更。

Name	   |	Description   |		Program	Parameters   |		Working Directory
--------- | --------------- | ------------------------- | -------------------------
Rustc   |	run with rustc   |		/usr/local/bin/rustc   |	\$FileName\$   |	\$FileDir\$
CargoRun   |	cargo run   |	/usr/local/bin/proxychains4   |	/usr/local/bin/cargo run   |	\$ProjectFileDir\$
CargoBuild   |	cargo build	/usr/local/bin/proxychains4	/usr/local/bin/cargo build	\$ProjectFileDir\$
CargoTest   |	cargo test	/usr/local/bin/cargo   |		test	   |	\$ProjectFileDir\$
CargoFmt   |	cargo fmt	/usr/local/bin/cargo   |		fmt	   |	\$ProjectFileDir\$

