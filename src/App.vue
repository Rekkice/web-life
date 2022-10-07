<script>
import init, { init_state, State } from "/rust/pkg/rust.js";
export default {
  data() {
    return {
      state: State,
      string: "a",
    }
  },

  created() {
    init().then(() => {
          this.state = init_state(15, 5);
          this.string = this.state.get_string().replace(new RegExp("[\r\n]", "gm"), "<br>");
          console.log(this.string);
    });
  },

  methods: {
    nextTick() {
      this.state = this.state.next_tick();
      this.string = this.state.get_string().replace(new RegExp("[\r\n]", "gm"), "<br>");
      console.log(this.string);
    },
  }

}

</script>

<style>
body {
  min-height: 100vh;
  overflow-y: hidden;
  z-index: 10;
}

.newbody {
  z-index: 1;
}

.life {
  z-index: 0;
  position: absolute;
  top: 50%;
  right: 50%;
  text-align: center;
  transform: translate(50%, -50%);
}

</style>

<template>

<div v-html="string" class="life is-size-1 has-text-primary"></div>

<body class="is-flex is-align-items-center is-justify-content-center">
  <div class="newbody columns is-centered is-flex-grow-1">
    <div class="column is-three-quarters">
      <div class="box is-flex is-flex-direction-column is-align-items-center">
        <button class="button" @click="nextTick">Next tick</button>
      </div>
    </div>
  </div>
</body>
</template>