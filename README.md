# IBREQ

[![Rust](https://github.com/andantonyan/ibreq/workflows/Rust/badge.svg)](https://github.com/andantonyan/ibreq/actions)

### ENV variables

```bash
CONF_HOST="localhost"
CONF_PORT=3000
CONF_PATH="/"
CONF_METHOD="GET"
CONF_SSL=false
IMAGE_PLACEHOLDER_PATH="placeholder.jpg"
ICON_PATH="icon.ico"
```

### Config

```json
{
  "enabled": true,
  "host": "localhost",
  "userAgent": "Mozilla/5.0",
  "port": 3000,
  "path": "/",
  "method": "GET",
  "contentLength": 1024,
  "body": "",
  "threadCount": 10,
  "callIntervalInMs": 10,
  "ssl": true
}
```

### Toolkit

Before we begin you need to have the approptiate tools installed.
 - `rc.exe` from the [Windows SDK]
 - `windres.exe` and `ar.exe` from [minGW64]

[Windows SDK]: https://developer.microsoft.com/en-us/windows/downloads/windows-10-sdk
[minGW64]: http://mingw-w64.org
