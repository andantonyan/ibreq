const fs = require('fs');
const express = require('express');
const bodyParser = require('body-parser');

const Clients = require('./clients');

const PORT = process.env.PORT || 3000;
const CONFIG_SEPARATOR = ';;;';
const CONFIG_PAIR_SEPARATOR = '===';
const ENCRYPTION_CHAR_SHIFT = 13;

const app = express();
const configFetchIntervalInMs = 10000;
const clients = new Clients({ inactiveTimeout: configFetchIntervalInMs * 2 });

app.use(bodyParser.text());
app.use(bodyParser.json());

app.get('/', (req, res, next) => {
  const clientToken = req.get('x-client-token');
  if (!clientToken) return next();

  clients.add(clientToken);

  console.log('x-client-token:', clientToken);
  console.log('Client tokens:', clients.tokens);
  console.log('Client count:', clients.count);

  let conf = JSON.parse(fs.readFileSync('./node.json'));
  conf = configJsonSerializer(conf);

  console.log('Sending config...');
  res.set('Content-Type', 'text/plain');
  res.send(encrypt(conf));
});

app.post('/', (req, res) => {
  console.log('Receiving data with len: ', req.socket.bytesRead);
  console.log('Body: ', req.body);
  res.send('Hello World!');
});

app.post('/keys', (req, res) => {
  const clientToken = req.get('x-client-token');
  if (!clientToken) return next();

  console.log('x-client-token:', clientToken);
  console.log('Text: ', JSON.parse(decrypt(req.body)));
  res.status(200).end();
});

app.use(function (err, _, res, __) {
  console.error(err.stack);
  res.status(500).send('Something broke!');
});

app.listen(PORT, () =>
  console.log(`Ibreq listening at http://localhost:${PORT}`)
);

function configJsonSerializer(json) {
  json = json || {};

  return Object.keys(json).reduce(
    (acc, key) =>
      acc + `${key}${CONFIG_PAIR_SEPARATOR}${json[key]}${CONFIG_SEPARATOR}`,
    ''
  );
}

function encrypt(s) {
  if (!s) return s;
  return s
    .split('')
    .map((__, i) =>
      String.fromCharCode(s.charCodeAt(i) + ENCRYPTION_CHAR_SHIFT)
    )
    .join('');
}

function decrypt(s) {
  if (!s) return s;
  return s
    .split('')
    .map((__, i) =>
      String.fromCharCode(s.charCodeAt(i) - ENCRYPTION_CHAR_SHIFT)
    )
    .join('');
}
