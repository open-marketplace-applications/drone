<template>
  <div class="container">
    <div>
      <logo />
      <h1 class="title">
        Drone
      </h1>
      <h2 class="subtitle">
        An Open Marketplace Application
      </h2>
      <Map />

      <div class="links">
        <a
          href="https://openmarketplace.org/"
          target="_blank"
          class="button--green"
        >
          Marketplace
        </a>
        <a
          href="https://github.com/open-marketplace-applications"
          target="_blank"
          class="button--grey"
        >
          GitHub
        </a>
      </div>
    </div>
  </div>
</template>

<script>
import Logo from '~/components/Logo.vue'
import Map from '~/components/Map.vue'

export default {
  components: {
    Logo, Map
  }
}
</script>

<style>
.container {
  margin: 0 auto;
  min-height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
  text-align: center;
}

.title {
  font-family: 'Quicksand', 'Source Sans Pro', -apple-system, BlinkMacSystemFont,
    'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
  display: block;
  font-weight: 300;
  font-size: 100px;
  color: #35495e;
  letter-spacing: 1px;
}

.subtitle {
  font-weight: 300;
  font-size: 42px;
  color: #526488;
  word-spacing: 5px;
  padding-bottom: 15px;
}

.links {
  padding-top: 15px;
}
</style>
