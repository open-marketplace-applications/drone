# Drone application

This programm simulates a drone, which is show on the open marketplace map. 

Build and run it
```bash
cargo run --bin drone
```

### Build with docker

```bash
docker build -t drone .
```

```bash
docker run --rm -it -p 5000:5000 drone
```
