#!/bin/bash

# 版本设置脚本
# 用法: ./bin/set_version.sh <version>
# 例如: ./bin/set_version.sh 25.0.5

set -e

if [ -z "$1" ]; then
    echo "错误: 请提供版本号"
    echo "用法: $0 <version>"
    echo "例如: $0 25.0.5"
    exit 1
fi

NEW_VERSION=$1
CURRENT_BRANCH=$(git branch --show-current)

echo "=========================================="
echo "设置新版本: $NEW_VERSION"
echo "当前分支: $CURRENT_BRANCH"
echo "=========================================="

# 1. 更新 package.json
echo "更新 package.json..."
if [ -f "package.json" ]; then
    sed -i.bak "s/\"version\": \".*\"/\"version\": \"$NEW_VERSION\"/" package.json
    rm -f package.json.bak
    echo "✓ package.json 已更新"
else
    echo "⚠ package.json 不存在"
fi

# 2. 更新 src-tauri/Cargo.toml
echo "更新 src-tauri/Cargo.toml..."
if [ -f "src-tauri/Cargo.toml" ]; then
    sed -i.bak "s/^version = \".*\"/version = \"$NEW_VERSION\"/" src-tauri/Cargo.toml
    rm -f src-tauri/Cargo.toml.bak
    echo "✓ src-tauri/Cargo.toml 已更新"
else
    echo "⚠ src-tauri/Cargo.toml 不存在"
fi

# 3. 更新 src-tauri/tauri.conf.json
echo "更新 src-tauri/tauri.conf.json..."
if [ -f "src-tauri/tauri.conf.json" ]; then
    sed -i.bak "s/\"version\": \".*\"/\"version\": \"$NEW_VERSION\"/" src-tauri/tauri.conf.json
    rm -f src-tauri/tauri.conf.json.bak
    echo "✓ src-tauri/tauri.conf.json 已更新"
else
    echo "⚠ src-tauri/tauri.conf.json 不存在"
fi

# 4. 创建新的开发分支
NEW_BRANCH="dev-$NEW_VERSION"
echo ""
echo "创建新的开发分支: $NEW_BRANCH"
read -p "是否创建新分支并切换? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    git checkout -b "$NEW_BRANCH"
    echo "✓ 已创建并切换到分支: $NEW_BRANCH"
else
    echo "跳过创建新分支"
fi

echo ""
echo "=========================================="
echo "版本更新完成!"
echo "新版本: $NEW_VERSION"
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "新分支: $NEW_BRANCH"
fi
echo "=========================================="
echo ""
echo "接下来的步骤:"
echo "1. 检查更改: git diff"
echo "2. 提交更改: git add . && git commit -m 'chore: bump version to $NEW_VERSION'"
echo "3. 推送到远程: git push -u origin $NEW_BRANCH"
