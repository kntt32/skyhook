# Skyhook
A lightweight CLI tool for seamless data sharing across terminals and devices.

## Installation
```
cargo install --path .
```

## Usage
### Start the daemon
```
skyhookd
```

### Run locally
```
> echo "Hello" | skyhook
> skyhook
Hello
```

### Connect to a remote device
```
> echo "Hello" | skyhook 192.168.0.1
> skyhook 192.168.0.1
Hello
```
