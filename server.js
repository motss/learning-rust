// @ts-check

const http = require('http');
const cluster = require('cluster');
const os = require('os');

if (cluster.isMaster) {
  let i = os.cpus().length;

  while (i--) {
    cluster.fork();
  }
} else {
  http.createServer((_, res) => {
    return res.end('Hello, World!');
  })
    .listen(5000);
}
