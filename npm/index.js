#!/usr/bin/env node

const { platform, arch } = process;
const { join } = require("path");
const { spawn } = require("child_process");
const { existsSync, chmodSync } = require("fs");

// 根据平台和架构确定二进制文件的路径
let binaryPath;
const baseDir = join(__dirname, "bin"); // 二进制文件所在的目录

// 注意：你需要根据你实际的二进制文件名和结构来调整这里的逻辑
if (platform === "win32" && arch === "x64") {
  binaryPath = join(baseDir, "pupup_windows_x64.exe");
} else if (platform === "linux" && arch === "x64") {
  binaryPath = join(baseDir, "pupup_linux_x64");
} else if (platform === "darwin" && arch === "x64") {
  binaryPath = join(baseDir, "pupup_macos_x64");
} else {
  console.error(`错误：不支持当前平台 (${platform}) 或架构 (${arch})。`);
  process.exit(1);
}

// 检查二进制文件是否存在
if (!existsSync(binaryPath)) {
  console.error(`错误：找不到当前平台的二进制文件：${binaryPath}`);
  console.error("请确保你的系统与此 npm 包兼容。");
  process.exit(1);
}

// 在非 Windows 平台上确保二进制文件是可执行的
// 有时候 npm 会自动处理 bin 字段指向的文件的权限，
// 但手动设置更保险，尤其如果 bin 指向的是这个 wrapper 脚本，
// 而 wrapper 脚本再调用 bin 目录下的文件。
if (platform !== "win32") {
  try {
    // 赋予所有者、组和其他用户读取和执行权限 (rwxr-xr-x)
    // 这假设你希望它对所有用户可执行。如果只对所有者，可以使用 0o700。
    chmodSync(binaryPath, 0o755);
  } catch (err) {
    // 忽略权限错误，因为可能已经在别处设置了，或者用户没有权限修改
    // 重要的错误会在 spawn 失败时捕获
    // console.warn(`警告：未能设置二进制文件权限: ${err.message}`);
  }
}

// 获取用户输入的参数 (去掉 node 和脚本本身的路径)
const args = process.argv.slice(2);

// 执行找到的二进制文件，并连接其标准输入/输出/错误
const child = spawn(binaryPath, args, {
  stdio: "inherit", // 将子进程的标准流连接到父进程的标准流
});

// 处理子进程执行过程中的错误 (例如，文件不存在，权限问题等)
child.on("error", (err) => {
  console.error(`执行二进制文件时出错：${err.message}`);
  process.exit(1); // 子进程执行失败，退出并返回非零状态码
});

// 子进程退出时，使用子进程的退出码退出父进程
child.on("exit", (code) => {
  process.exit(code === null ? 1 : code); // 如果 code 为 null (信号退出), 则返回 1
});
