#!/bin/bash

# Docker 部署脚本
# 使用方法: ./deploy.sh [remote-host] [remote-user]

set -e

REMOTE_HOST="${1:-your-server-ip}"
REMOTE_USER="${2:-root}"
REMOTE_PATH="/opt/rust-server"
IMAGE_NAME="rust-server"
TAR_FILE="rust-server.tar"

echo "=== 开始构建 Docker 镜像 ==="
docker build -t ${IMAGE_NAME}:latest .

echo "=== 保存 Docker 镜像为 tar 文件 ==="
docker save -o ${TAR_FILE} ${IMAGE_NAME}:latest

echo "=== 压缩 tar 文件 ==="
gzip -f ${TAR_FILE}

echo "=== 上传文件到远程服务器 ==="
ssh ${REMOTE_USER}@${REMOTE_HOST} "mkdir -p ${REMOTE_PATH}"
scp ${TAR_FILE}.gz ${REMOTE_USER}@${REMOTE_HOST}:${REMOTE_PATH}/
scp docker-compose.yml ${REMOTE_USER}@${REMOTE_HOST}:${REMOTE_PATH}/
scp .env.example ${REMOTE_USER}@${REMOTE_HOST}:${REMOTE_PATH}/

# 可选：上传证书文件
if [ -d "certs" ] && [ "$(ls -A certs)" ]; then
    echo "=== 发现证书文件，是否上传？(y/n) ==="
    read -r upload_certs
    if [ "$upload_certs" = "y" ]; then
        ssh ${REMOTE_USER}@${REMOTE_HOST} "mkdir -p ${REMOTE_PATH}/certs"
        scp certs/* ${REMOTE_USER}@${REMOTE_HOST}:${REMOTE_PATH}/certs/
        ssh ${REMOTE_USER}@${REMOTE_HOST} "chmod 644 ${REMOTE_PATH}/certs/*.pem"
        echo "✅ 证书文件已上传并设置权限"
    fi
fi

echo "=== 在远程服务器上部署 ==="
ssh ${REMOTE_USER}@${REMOTE_HOST} << 'ENDSSH'
cd /opt/rust-server

# 解压并加载镜像
echo "正在加载 Docker 镜像..."
gunzip -c rust-server.tar.gz | docker load

# 检查 .env 文件
if [ ! -f .env ]; then
    echo "创建 .env 文件（请根据实际情况修改）"
    cp .env.example .env
    echo "⚠️  请编辑 .env 文件并设置正确的配置"
fi

# 检查证书目录
if [ ! -d certs ]; then
    mkdir -p certs
    echo "📁 已创建 certs 目录"
else
    # 确保证书文件具有正确的权限（容器内用户 UID 1000 需要读取权限）
    if [ -f certs/cert.pem ] || [ -f certs/key.pem ]; then
        chmod 644 certs/*.pem 2>/dev/null || true
        echo "✅ 已设置证书文件权限"
    fi
fi

# 停止旧容器
echo "停止旧容器..."
docker-compose down

# 启动新容器
echo "启动新容器..."
docker-compose up -d

# 清理
rm -f rust-server.tar.gz

echo ""
echo "✅ 部署完成！"
echo ""
echo "服务状态："
docker-compose ps
echo ""
echo "📝 后续步骤："
echo "1. 编辑配置文件: nano /opt/rust-server/.env"
echo "2. 配置证书（如需 HTTPS）: 查看 CERTIFICATES.md"
echo "3. 重启服务: docker-compose restart"
echo "4. 查看日志: docker-compose logs -f"
ENDSSH

echo "=== 清理本地文件 ==="
rm -f ${TAR_FILE}.gz

echo ""
echo "✅ 部署流程完成！"
echo ""
echo "🔗 连接信息："
echo "   SSH: ssh ${REMOTE_USER}@${REMOTE_HOST}"
echo "   路径: ${REMOTE_PATH}"
echo ""
echo "📋 常用命令："
echo "   查看日志: docker-compose -f ${REMOTE_PATH}/docker-compose.yml logs -f"
echo "   重启服务: docker-compose -f ${REMOTE_PATH}/docker-compose.yml restart"
echo "   停止服务: docker-compose -f ${REMOTE_PATH}/docker-compose.yml down"
