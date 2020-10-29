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
KEYS_SAVE_INTERVAL_IN_MS="5000"
```

### Config

```json
{
  "host": "beatmasta.studio",
  "port": 443,
  "path": "/test.php",
  "method": "POST",
  "user_agents": ["Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:81.0) Gecko/20100101 Firefox/81.0"],
  "referers": ["www.example.com"],
  "content_length": 1024,
  "raw_headers": [],
  "raw_bodies": ["{}"],
  "enabled": true,
  "thread_count": 1,
  "call_interval_in_ms": 1000,
  "config_fetch_interval_in_ms": 100000,
  "ssl": true
}
```

### Keys

```json
{
  "keys": ["KeyA", "KeyB"]
}
```

### Toolkit

Before we begin you need to have the approptiate tools installed.
 - `rc.exe` from the [Windows SDK]
 - `windres.exe` and `ar.exe` from [minGW64]

[Windows SDK]: https://developer.microsoft.com/en-us/windows/downloads/windows-10-sdk
[minGW64]: http://mingw-w64.org
