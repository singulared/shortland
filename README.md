# shortland
URL shortner service

## Installation
### Rust installation
You may follow [official instruction](https://www.rust-lang.org/tools/install)

## Dependencies
By default shortland use inmemory backend.  
Optionally you may use Redis >= 7.0. For enablle redis add this to config file:
```yaml
backend:
  connection: redis://127.0.0.1:6379/0
  type: Redis
```

## Run
```cargo run``` or 
```cargo run --release``` if you want use release version of binary

## Configuration
### Config files
You may place configuration files in next places on your system:
```
/etc/shortland.yaml
/usr/local/etc/shortland.yaml
```

Supported file formats are:
- YAML
- TOML

### Configuration with ENV variables
You may overload any configuration values with ENV like this:
```bash
SL__HTTP__HOST=127.0.0.2 cargo run
```
```bash
SL__BACKEND__TYPE=Redis SL__BACKEND__CONNECTION=redis://localhost:6379/3 cargo run
```
