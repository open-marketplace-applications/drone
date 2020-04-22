<template>
  <div id="map-wrap">
    <client-only>
      <l-map
        class="map"
        :zoom="zoom"
        :center="center"
      >
        <l-tile-layer url="http://{s}.tile.osm.org/{z}/{x}/{y}.png"></l-tile-layer>
        <l-marker :icon="iconTarget" :lat-lng="drone.target_location">
          <l-popup>{{drone}}</l-popup>
        </l-marker>
        <l-marker :icon="iconDrone" :lat-lng="drone.location">
          <l-popup>{{drone}}</l-popup>
        </l-marker>
        <l-marker
          v-for="(shop, index) in shops"
          :key="index"
          :icon="iconShop"
          :lat-lng="shop.location"
        >
          <l-popup>
            <h3>{{ shop.name }}</h3>
            <p>{{ shop.description }}</p>
            <nuxt-link :to="{ path: 'shops', query: { root: shop.root }}">Visit the shop</nuxt-link>
          </l-popup>
        </l-marker>
      </l-map>
    </client-only>
  </div>
</template>

<script>
const iotaAreaCodes = require("@iota/area-codes");
export default {
  components: {},
  data() {
    return {
      url: "http://{s}.tile.osm.org/{z}/{x}/{y}.png",
      zoom: 15,
      center: [process.env.cityLatitude, process.env.cityLongitude],
      drone: {
        location: [process.env.cityLatitude, process.env.cityLongitude],
        target_location: [0, 0]
      },
      bounds: null,
      shops: []
    };
  },
  async created() {
    let self = this;
    let intervalinms = 3000
    let oldPosition
    setInterval(async () => {
      try {
        const { data } = await self.$axios.get(process.env.droneUrl + "/drone");
        if(typeof oldPosition == 'undefined'){
            oldPosition = data.location
        }
        //increase value slowly until target is reached
        let distancex = data.location[0]-oldPosition[0]
        let distancey = data.location[1]-oldPosition[1]
        let iterations = 70
        let timenow = Date.now()
        for(let j= 0; j<iterations-5; j++){
          if(Date.now()-timenow>intervalinms){
            console.log("time reached, break");
            break
          }
          let newPos = []
          if(distancex > 0){
            newPos[0] = parseFloat(self.drone.location[0])+(Math.abs(distancex)/iterations)
          } else {
            newPos[0] = parseFloat(self.drone.location[0])-(Math.abs(distancex)/iterations)
          }
          if(distancey > 0){
            newPos[1] = parseFloat(self.drone.location[1])+(Math.abs(distancey)/iterations)
          } else {
            newPos[1] = parseFloat(self.drone.location[1])-(Math.abs(distancey)/iterations)
          }
          self.drone.location = newPos
          await new Promise(resolve => setTimeout(resolve, intervalinms/iterations));
        }
        //set final value
        oldPosition = data.location
        self.drone = data;
        self.center = self.drone.target_location
      } catch (error) {
        console.log("error fetching marketmap data", error);
      }
    }, intervalinms);
  },
  computed: {
    iconDrone() {
      if (process.browser) {
        require("vue2-leaflet");
        console.log("th", this);
        console.log("th", L);
        return L.icon({
          iconUrl: require("@/assets/drone.svg"),
          iconSize: [70, 70],
          iconAnchor: [35, 35]
        });
      }
    },
    iconShop() {
      if (process.browser) {
        require("vue2-leaflet");
        console.log("th", this);
        console.log("th", L);
        return L.icon({
          iconUrl: require("@/assets/shop_smal.svg"),
          iconSize: [40, 40],
          iconAnchor: [20, 20]
        });
      }
    },
    iconTarget() {
      if (process.browser) {
        require("vue2-leaflet");
        console.log("th", this);
        console.log("th", L);
        return L.icon({
          iconUrl: require("@/assets/target.svg"),
          iconSize: [35, 35],
          iconAnchor: [17.5, 17.5]
        });
      }
    }
  }
};
</script>

<style>
#map-wrap {
  height: 100vh;
  width: 100%;
}
.spots {
  padding-top: 80px;
  padding-bottom: 20px;
  float: flex;
  flex-direction: column;
  flex-wrap: nowrap;
  justify-content: center;
  width: 80%;
  display: block !important;
  margin-right: auto !important;
  margin-left: auto !important;
  float: none !important; /* Added */
}
</style>
