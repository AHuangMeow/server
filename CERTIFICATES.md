# 证书处理指南

## 三种方案选择

### 方案1: 不使用 HTTPS（最简单，适合内网或 Nginx 反向代理后）

**步骤：**
1. 在 `.env` 中注释掉 SSL 配置：
```bash
# SSL_CERT_PATH=/app/certs/cert.pem
# SSL_KEY_PATH=/app/certs/key.pem
```

2. 在 `docker-compose.yml` 中注释掉 SSL 环境变量：
```yaml
# - SSL_CERT_PATH=/app/certs/cert.pem
# - SSL_KEY_PATH=/app/certs/key.pem
```

3. 部署时使用 Nginx 处理 HTTPS（推荐）

---

### 方案2: 在远程服务器生成自签名证书（适合测试）

**本地部署脚本：**
```bash
./deploy.sh your-server-ip username
```

**在远程服务器上执行：**
```bash
ssh username@your-server-ip
cd /opt/rust-server

# 生成自签名证书
docker-compose exec server ./generate_certs.sh

# 或者在宿主机生成
mkdir -p certs
openssl genrsa -out certs/key.pem 2048
openssl req -new -x509 -key certs/key.pem -out certs/cert.pem -days 365 \
  -subj "/C=CN/ST=State/L=City/O=Organization/CN=your-domain.com"

# 重启服务
docker-compose restart server
```

---

### 方案3: 使用真实证书（适合生产环境）

#### 3.1 手动上传证书

```bash
# 本地有证书文件
scp cert.pem key.pem username@your-server-ip:/opt/rust-server/certs/

# 设置权限
ssh username@your-server-ip
cd /opt/rust-server
chmod 600 certs/key.pem
chmod 644 certs/cert.pem
```

#### 3.2 使用 Let's Encrypt（通过 Nginx）

**这是生产环境最推荐的方案**

1. **修改 docker-compose.yml，应用使用 HTTP：**
```yaml
services:
  server:
    environment:
      # 注释掉 SSL 配置
      # - SSL_CERT_PATH=/app/certs/cert.pem
      # - SSL_KEY_PATH=/app/certs/key.pem
    ports:
      - "127.0.0.1:8080:8080"  # 只监听本地，不暴露到外网
```

2. **在远程服务器安装 Nginx：**
```bash
sudo apt update
sudo apt install nginx certbot python3-certbot-nginx
```

3. **配置 Nginx：**
```bash
sudo nano /etc/nginx/sites-available/rust-server
```

内容：
```nginx
server {
    listen 80;
    server_name your-domain.com;

    location / {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

4. **启用配置：**
```bash
sudo ln -s /etc/nginx/sites-available/rust-server /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

5. **获取 SSL 证书：**
```bash
sudo certbot --nginx -d your-domain.com
```

证书会自动配置并自动续期！

---

## 快速决策树

```
是否有域名？
├─ 否 → 使用方案1（HTTP）或方案2（自签名证书）
└─ 是 → 有公网 IP？
    ├─ 否 → 使用方案2（自签名证书）
    └─ 是 → 使用方案3.2（Let's Encrypt + Nginx）★ 推荐
```

---

## 推荐配置

### 开发/测试环境
```bash
# 不使用 HTTPS，简单快速
# .env 中注释掉 SSL 配置
```

### 生产环境
```bash
# 使用 Nginx + Let's Encrypt
# 应用使用 HTTP，Nginx 处理 HTTPS
```

---

## 故障排查

### 证书文件找不到
```bash
# 检查证书文件是否存在
docker-compose exec server ls -la /app/certs/

# 检查挂载是否正确
docker-compose exec server cat /proc/mounts | grep certs
```

### 权限问题
```bash
# 容器内用户是 appuser (uid=1000)
# 确保宿主机上的证书文件可读
chmod 644 certs/cert.pem
chmod 600 certs/key.pem
chown 1000:1000 certs/*.pem
```

### SSL 握手失败
```bash
# 测试证书
openssl s_client -connect localhost:8080 -showcerts

# 查看服务器日志
docker-compose logs -f server
```

---

## 完整部署示例（生产环境）

```bash
# 1. 本地构建并部署
./deploy.sh your-server-ip root

# 2. SSH 到服务器
ssh root@your-server-ip
cd /opt/rust-server

# 3. 配置使用 HTTP（不在应用层使用 HTTPS）
nano .env
# 注释掉 SSL 配置

nano docker-compose.yml
# 修改 ports 为 "127.0.0.1:8080:8080"
# 注释掉 SSL 环境变量

# 4. 重启服务
docker-compose restart

# 5. 安装并配置 Nginx + SSL
sudo apt install nginx certbot python3-certbot-nginx
# 按照上面的步骤配置 Nginx 和 Let's Encrypt

# 6. 测试
curl https://your-domain.com/health
```

这样配置后，证书由 Nginx 管理，自动续期，应用层保持简单！
