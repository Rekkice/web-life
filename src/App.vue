<script>
import Life from "./Life.vue"
export default {
  components: {
    Life
  },
  data() {
    return {
      color: "#00a6a6",
    }
  },
}
</script>

<style>
body {
  min-height: 100vh;
  overflow: hidden;
}

.box {
  min-height: 90vh;
  max-height: 90vh;
  height: 100%;
  width: 100%;
}
</style>

<template>
<body class="is-flex is-align-items-center is-justify-content-center">
  <div class="columns is-centered is-flex-grow-1">
    <div class="column is-three-quarters">
      <div ref="box" class="box is-align-items-center has-background-black">

        <Life :color=color />

      </div>
    </div>
  </div>
</body>
</template>