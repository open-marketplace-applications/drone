# Drone application

This programm simulates a drone, which is show on the open marketplace map. 

Build and run it
```bash
cargo run --bin drone
```

### Build with docker

Build docker image
```bash
docker build -t drone .
```

Run the image
```bash
docker run -p 5000:5000 drone:latest
```
