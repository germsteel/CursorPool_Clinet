import http from 'http';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
import { networkInterfaces } from 'os';

// 获取当前文件的目录
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// MSI文件路径
const MSI_FILE_PATH = 'D:\\full-stack\\cursorpool\\src-tauri\\target\\release\\bundle\\msi\\cursor-pool_1.7.0_x64_en-US.msi';

// 获取本地IP地址
function getLocalIpAddress() {
  const nets = networkInterfaces();
  
  for (const name of Object.keys(nets)) {
    for (const net of nets[name]) {
      // 跳过内部IP和非IPv4地址
      if (!net.internal && net.family === 'IPv4') {
        return net.address;
      }
    }
  }
  return '127.0.0.1'; // 默认返回localhost
}

const localIp = getLocalIpAddress();

// 更新信息配置
const updateInfo = {
  version: '1.7.0', // 比当前版本高的版本号
  notes: '这是一个测试更新\n\n- 修复了一些bug\n- 添加了新功能\n- 优化了性能',
  pub_date: new Date().toISOString(),
  platforms: {
    'windows-x86_64': {
      signature: 'dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNlY3JldCBrZXkKUlVUbTlqSytZQmhseVYwbTFuSHpLZnZ3WllOZ29mVVJOQXBrcHdzUDBUN2QyV1V0eW1ZeTlneW9JaTZBV1pqSXRiUSt4cGFGRUFSS1paelRZM0lMYlhQNXJEWGs3NCtPVVFnPQp0cnVzdGVkIGNvbW1lbnQ6IHRpbWVzdGFtcDoxNzQxOTU0MTE4CWZpbGU6Y3Vyc29yLXBvb2xfMS43LjBfeDY0X2VuLVVTLm1zaQpLRGczUFBEYzhPaHFFZmxWNVlxWjZYbEVaSlppL1BlK3hFbDNkVmdJOENRc2F0SnZGbk5tVXdMQ3hML1VNRkVmMXhkb0I1dGV4MHhRbE10dWpOVnlDQT09Cg==', // 实际环境中需要有效签名
      url: `http://${localIp}:8878/download/cursor-pool_1.7.0_x64_en-US.msi`
    },
  }
};

// 创建HTTP服务器
const server = http.createServer((req, res) => {
  // 设置CORS头，允许所有来源
  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Access-Control-Allow-Methods', 'GET, OPTIONS');
  res.setHeader('Access-Control-Allow-Headers', 'Content-Type');
  
  // 处理OPTIONS请求
  if (req.method === 'OPTIONS') {
    res.statusCode = 204;
    res.end();
    return;
  }
  
  // 处理GET请求
  if (req.method === 'GET') {
    // 更新信息端点
    if (req.url === '/updater') {
      res.statusCode = 200;
      res.setHeader('Content-Type', 'application/json');
      res.end(JSON.stringify(updateInfo, null, 2));
      console.log('已返回更新信息');
      return;
    }
    
    // 下载端点 - 提供实际的MSI文件
    if (req.url === '/download/cursor-pool_1.7.0_x64_en-US.msi') {
      // 检查文件是否存在
      if (!fs.existsSync(MSI_FILE_PATH)) {
        console.error(`错误：MSI文件不存在: ${MSI_FILE_PATH}`);
        res.statusCode = 404;
        res.end('文件不存在');
        return;
      }
      
      try {
        // 获取文件信息
        const stat = fs.statSync(MSI_FILE_PATH);
        const fileSize = stat.size;
        const fileName = path.basename(MSI_FILE_PATH);
        
        // 设置响应头
        res.statusCode = 200;
        res.setHeader('Content-Type', 'application/octet-stream');
        res.setHeader('Content-Disposition', `attachment; filename="${fileName}"`);
        res.setHeader('Content-Length', fileSize);
        
        // 创建文件读取流并将其传输到响应
        const fileStream = fs.createReadStream(MSI_FILE_PATH);
        
        // 处理可能的错误
        fileStream.on('error', (error) => {
          console.error(`读取文件时出错: ${error.message}`);
          if (!res.headersSent) {
            res.statusCode = 500;
            res.end('服务器内部错误');
          } else {
            res.end();
          }
        });
        
        // 流式传输文件
        fileStream.pipe(res);
        
        console.log(`正在提供下载: ${fileName} (${fileSize} 字节)`);
        return;
      } catch (error) {
        console.error(`提供文件下载时出错: ${error.message}`);
        res.statusCode = 500;
        res.end('服务器内部错误');
        return;
      }
    }
    
    // 其他路径返回404
    res.statusCode = 404;
    res.end('Not Found');
    return;
  }
  
  // 其他HTTP方法返回405
  res.statusCode = 405;
  res.end('Method Not Allowed');
});

// 监听端口
const PORT = 8878;
const HOST = '0.0.0.0';

server.listen(PORT, HOST, () => {
  console.log(`更新服务器运行在 http://${HOST}:${PORT}`);
  console.log(`本机IP地址: ${localIp}`);
  console.log(`测试更新信息: http://${localIp}:${PORT}/updater`);
  console.log(`测试下载链接: http://${localIp}:${PORT}/download/cursor-pool_1.7.0_x64_en-US.msi`);
  console.log(`MSI文件路径: ${MSI_FILE_PATH}`);
}); 