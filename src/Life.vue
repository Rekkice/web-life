<script>
import init, { init_state, State } from "/rust/pkg/rust_web_life.js";
export default {
  props: {
    color: String,
  },
  data() {
    return {
      state: State,
      stateX: 0,
      stateY: 0,

      canvasWidth: 10,
      canvasHeight: 0,

      windowWidth: window.innerWidth,

      cellSize: 60,

      canvas: null,
      alpha: 0,
      delta: 0.01,
      animationFrame: -1,
    }
  },

  mounted() {
    this.canvas = document.getElementById("c").getContext("2d");
    init().then(() => {
        this.lifeInit();
    });
    window.addEventListener("resize", this.lifeInit);
  },

  unmounted() {
    window.removeEventListener("resize", this.lifeInit);
  },

  methods: {
    lifeInit () {
      if (this.animationFrame != -1) cancelAnimationFrame(this.animationFrame);
      let canE = document.getElementById("c").parentElement;
      console.log(canE);
      let x = canE.getBoundingClientRect().width;
      let y = canE.getBoundingClientRect().height;

      this.canvasWidth = x;
      this.canvasHeight = y;

      this.state = init_state((x * 0.8 / this.cellSize), (y * 0.8 / this.cellSize));
      this.stateX = this.state.get_x();
      this.stateY = this.state.get_y();
      this.string = this.state.get_string().replace(new RegExp("[\r\n]", "gm"), "<br>");

      this.renderLoop();
    },
    renderLoop() {
      let canvas = this.canvas;
      this.windowWidth = window.innerWidth;

      let canE = document.getElementById("c").parentElement;
      this.canvasWidth = canE.getBoundingClientRect().width;
      this.canvasHeight = canE.getBoundingClientRect().height;

      canvas.beginPath();
      canvas.strokeStyle = "#2e2e2e";
      canvas.globalAlpha = this.alpha;

      this.alpha += this.delta;

      if (this.alpha <= 0) this.state = this.state.next_tick();
      if (this.alpha <= 0 || this.alpha >= 1) this.delta *= -1;

      canvas.clearRect(0, 0, this.canvasWidth, this.canvasHeight);
      
      const cellSize = this.cellSize;
      const gapX = (this.canvasWidth - (this.stateX * cellSize)) / this.stateX + 1;
      const gapY = (this.canvasHeight - (this.stateY * cellSize)) / this.stateY + 1;

      for (let i = 0; i < this.stateX; i++) {
        for (let j = 0; j < this.stateY; j++) {
          if (!this.state.get_cell(i, j)) continue;
          canvas.fillStyle = this.color
          canvas.fillRect(
            i * (cellSize + gapX),
            j * (cellSize + gapY),
            cellSize,
            cellSize
          );
        }
      }

      canvas.stroke();

      this.animationFrame = requestAnimationFrame(this.renderLoop);
    },
  }

}

</script>

<style>
canvas {
  height: 100%;
  width: 100%;
  /* min-height: 90vh;
  max-height: 90vh; */
}
</style>

<template>
<canvas id="c" :width="canvasWidth" :height="canvasHeight"></canvas>
</template>