<template>
  <header class="header" v-bind:class="{ collapsed: collapsed }">
    <div class="header--logo">
      <img src="@/assets/oma.svg" alt="Open Marketplace Applications" />
    </div>
    <div
      class="header--machine-name"
      :style="{'background-image': `url(${require('../assets/name-box.svg')})`}"
    >
      <h1>Drone Application</h1>
    </div>
    <div class="header--menu">
      <button class="btn btn-primary">
        <img src="@/assets/icons/settings.svg" alt="Settings Icon" />
        <span>Settings</span>
      </button>
    </div>
  </header>
</template>

<script>
export default {
  props: ["collapsed"]
};
</script>

<style lang="scss">
.header {
  height: 100px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-top: var(--border);
  box-sizing: border-box;
  position: fixed;
  z-index: 1;
  transition: var(--transition-cubic);
  width: 100%;
  top: 0px;
  z-index: 1000;
  color: #fff;
  &:before {
    content: "";
    position: absolute;
    bottom: 100px;
    width: 100%;
    height: 300px;
    background-color: var(--dark);
    z-index: 5;
  }
  &.collapsed {
    top: 300px;
  }
  &--logo {
    width: 100%;
    padding-left: 25px;
    img {
      height: 50px;
    }
  }
  &--machine-name {
    min-width: 600px;
    height: 105px;
    background-size: contain;
    background-repeat: no-repeat;
    position: relative;
    top: -8px;
    display: flex;
    justify-content: center;
    align-items: center;
  }
  &--menu {
    width: 100%;
    padding-right: 25px;
    display: flex;
    justify-content: flex-end;
  }
}
.btn {
  background-color: transparent;
  border-left: 5px solid var(--primary);
  border-right: 5px solid var(--primary);
  border: none;
  position: relative;
  background-color: var(--primary);
  display: flex;
  align-items: center;
  padding: 5px 15px;
  img {
    margin-right: 8px;
  }
  span {
    font-family: "Oswald", sans-serif;
    color: var(--white);
  }
  &:before,
  &:after {
    content: "";
    position: absolute;
    left: 0;
    right: 0;
    bottom: 100%;
    border-bottom: 5px solid var(--primary);
    border-left: 5px solid transparent;
    border-right: 5px solid transparent;
  }
  &:after {
    bottom: auto;
    top: 100%;
    border-bottom: none;
    border-top: 5px solid var(--primary);
  }
  &-primary {
  }
}
</style>