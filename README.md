# Server

一个简单的，基于 Actix-web 构建的 Rust Web 服务器

## ✨ 特性

- 🚀 **高性能**: 基于 Actix-web 框架，异步处理
- 🔐 **安全认证**: JWT 令牌认证，bcrypt 密码加密
- 📊 **数据存储**: MongoDB 数据库 + Redis 缓存
- 🔒 **HTTPS 支持**: 可选的 SSL/TLS 加密
- 🐳 **Docker 部署**: 完整的容器化支持
- 📝 **日志追踪**: 结构化日志记录

## 📋 技术栈

- **Web 框架**: [Actix-web](https://actix.rs/) 4.x
- **数据库**: [MongoDB](https://www.mongodb.com/) 7.x
- **缓存**: [Redis](https://redis.io/) 7.x
- **认证**: JWT + bcrypt
- **TLS**: rustls
- **日志**: tracing + tracing-subscriber

## 🚀 快速开始

### 前置要求

- Rust 1.75+
- MongoDB 7.0+
- Redis 7.0+
- Docker & Docker Compose (可选)

### 本地开发

1. **克隆项目**
```bash
git clone https://github.com/AHuangMeow/server.git
cd server
```

2. **配置环境变量**
```bash
cp .env.example .env
nano .env  # 修改配置
```

3. **生成 SSL 证书 (可选)**
```bash
chmod +x generate_certs.sh
./generate_certs.sh
```

4. **启动依赖服务**
```bash
# 使用 Docker 启动 MongoDB 和 Redis
docker-compose up -d mongodb redis
```

5. **运行服务器**
```bash
cargo run
```

服务器将在 `http://localhost:8080` 启动

### Docker 部署

#### 本地测试

```bash
# 启动所有服务
docker-compose up -d

# 查看日志
docker-compose logs -f

# 停止服务
docker-compose down
```

#### 部署到远程服务器

```bash
# 使用自动化部署脚本
chmod +x deploy.sh
./deploy.sh your-server-ip username

# 登录服务器配置
ssh username@your-server-ip
cd /opt/rust-server
nano .env  # 修改配置
docker-compose restart
```

详细部署文档请查看 [CERTIFICATES.md](CERTIFICATES.md)

## 📚 API 文档

### 健康检查

```http
GET /health
```

### 认证相关

```http
POST /auth/register    # 用户注册
POST /auth/login       # 用户登录
POST /auth/logout      # 用户登出
```

### 用户相关

```http
GET    /user/me           # 获取用户信息
PUT    /user/email        # 更新用户邮箱
PUT    /user/username     # 更新用户名
PUT    /user/password     # 更新用户密码
```

### 管理员相关

```http
GET    /admin/users       # 获取所有用户
POST   /admin/users       # 创建用户
GET    /admin/users/:id   # 获取用户信息
PUT    /admin/users/:id   # 更新用户信息
DELETE /admin/users/:id   # 删除用户
PUT    /admin/users/:id/admin # 设置用户权限
```

## ⚙️ 配置说明

### 环境变量

| 变量名 | 说明 | 默认值 |
|--------|------|--------|
| `APP_HOST` | 服务器监听地址 | `0.0.0.0` |
| `APP_PORT` | 服务器端口 | `8080` |
| `MONGO_URI` | MongoDB 连接字符串 | `mongodb://localhost:27017` |
| `MONGO_DB` | 数据库名称 | `actix_server` |
| `REDIS_URI` | Redis 连接字符串 | `redis://localhost:6379` |
| `JWT_SECRET` | JWT 密钥 | - |
| `JWT_EXP_HOURS` | JWT 过期时间（小时） | `24` |
| `SSL_CERT_PATH` | SSL 证书路径 (可选) | - |
| `SSL_KEY_PATH` | SSL 密钥路径 (可选) | - |

### Docker Compose 配置

修改 `docker-compose.yml` 可以调整：
- 端口映射
- 数据卷挂载
- 网络配置
- 资源限制

## 🔒 安全建议

1. **修改默认密钥**: 务必修改 `.env` 中的 `JWT_SECRET` 为强随机字符串
2. **使用 HTTPS**: 生产环境建议使用 Nginx 反向代理 + Let's Encrypt
3. **防火墙配置**: 只开放必要的端口
4. **定期更新**: 保持依赖库和系统更新
5. **备份数据**: 定期备份 MongoDB 数据

## 📦 项目结构

```
server/
├── src/
│   ├── auth/           # 认证模块
│   ├── config/         # 配置管理
│   ├── database/       # 数据库连接
│   ├── handlers/       # API 处理器
│   │   ├── admin.rs    # 管理员接口
│   │   ├── auth.rs     # 认证接口
│   │   ├── health.rs   # 健康检查
│   │   └── user.rs     # 用户接口
│   ├── models/         # 数据模型
│   ├── utils/          # 工具函数
│   ├── errors.rs       # 错误处理
│   ├── constants.rs    # 常量定义
│   └── main.rs         # 程序入口
├── certs/              # SSL 证书目录
├── Cargo.toml          # Rust 依赖配置
├── Dockerfile          # Docker 镜像定义
├── docker-compose.yml  # Docker Compose 配置
├── deploy.sh           # 部署脚本
├── .env.example        # 环境变量示例
├── CERTIFICATES.md     # 证书配置指南
└── README.md           # 项目文档
```

## 🛠️ 开发

### 编译项目

```bash
# 开发模式
cargo build

# 发布模式
cargo build --release
```

### 运行测试

```bash
cargo test
```

### 代码检查

```bash
# 代码格式化
cargo fmt

# 代码检查
cargo clippy
```

## 📊 监控和日志

### 查看 Docker 日志

```bash
# 查看所有服务日志
docker-compose logs -f

# 查看特定服务日志
docker-compose logs -f server
```

### 日志级别

通过环境变量 `RUST_LOG` 控制日志级别：

```bash
RUST_LOG=info cargo run       # 生产环境
RUST_LOG=debug cargo run      # 开发环境
RUST_LOG=trace cargo run      # 详细调试
```

## 🔧 故障排查

### 无法连接数据库

```bash
# 检查 MongoDB 是否运行
docker-compose ps mongodb

# 查看 MongoDB 日志
docker-compose logs mongodb

# 测试连接
docker-compose exec mongodb mongosh --eval "db.adminCommand('ping')"
```

### Redis 连接失败

```bash
# 检查 Redis 是否运行
docker-compose ps redis

# 测试连接
docker-compose exec redis redis-cli ping
```

### 端口被占用

```bash
# 检查端口占用
netstat -tlnp | grep 8080

# 或使用 lsof
lsof -i :8080
```

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📄 许可证

本项目采用 [MIT License](LICENSE) 开源协议
