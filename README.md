<!--lint disable no-literal-urls-->

<p align="center">
  <br>
  <a href="https://openmarketplace.org/">
     <img
      alt="open marketplace apps"
      src="./static/oma.jpg"
      width="200"
    />
  </a>
</p>


<h1 align="center"><a href="https://openmarketplace.org/"> Open Marketplace Apps</a></h1>

<p align="center">Open marketplace apps are decentralised open source apps for smart cities and local communities. </p>

<p align="center">
  <a title="MIT License" href="LICENSE">
    <img src="https://img.shields.io/github/license/gridsome/gridsome.svg?style=flat-square&label=License&colorB=6cc24a">
  </a>
  <a title="Follow on Twitter" href="https://twitter.com/marketplace_org">
    <img src="https://img.shields.io/twitter/follow/marketplace_org.svg?style=social&label=Follow%20@marketplace_org">
  </a>
  <br>
  <br>
</p>

# Drone Application

**Demo:** [https://simulated-drone-1.herokuapp.com/](https://simulated-drone-1.herokuapp.com/)

> This programm simulates a drone, which is show on the open marketplace map.


 🛠️ 🛠️ 🛠️ UNDER DEVELOPMENT 🛠️ 🛠️ 🛠️

 Contribute to this project - join our [Discord Server](https://discord.gg/XDQQcJC).


## Local Setup

```bash
git clone https://github.com/open-marketplace-applications/drone
cd drone
cargo run --bin drone
```

Open http://localhost:3000/ in your browser.


### Build with docker

Build docker image
```bash
docker build -t drone .
```

Run the image
```bash
docker run -p 5000:5000 drone:latest
```
