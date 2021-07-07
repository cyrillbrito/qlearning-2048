<template>
  <img alt="Vue logo" src="./assets/logo.png" />
  <HelloWorld msg="Welcome to Your Vue.js + TypeScript App" />
  <!-- <Piece v-bind:value="1" />
  <Piece v-bind:value="2" />
  <Piece v-bind:value="3" />
  <Piece v-bind:value="4" />
  <Piece v-bind:value="5" />
  <Piece v-bind:value="6" />
  <Piece v-bind:value="7" v-bind:style="{ marginLeft: margin }" />
  <Piece v-bind:value="8" />
  <Piece v-bind:value="9" />
  <Piece v-bind:value="10" />
  <Piece v-bind:value="11" />
  <Piece v-bind:value="12" /> -->

  <button v-on:click="move()">CC</button>

  <Board v-bind:pieces="b" />
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import HelloWorld from "./components/HelloWorld.vue";
// import Piece from "./components/Piece.vue";
import Board, { PieceDef } from "./components/Board.vue";

export default defineComponent({
  name: "App",
  components: {
    HelloWorld,
    // Piece,
    Board,
  },
  setup() {
    const b = ref([
      { id: 1, value: 3, position: 0 },
      { id: 2, value: 4, position: 1 },
      { id: 3, value: 2, position: 3 },
      { id: 4, value: 6, position: 6 },
      { id: 5, value: 6, position: 8 },
    ] as PieceDef[]);
    const margin = ref("0px");

    return {
      b,
      margin,
      move: () =>
        (b.value = [
          { id: 1, value: 3, position: 1 },
          { id: 2, value: 4, position: 2 },
          { id: 3, value: 2, position: 5 },
          { id: 4, value: 7, position: 8 },
        ]),
    };
  },
});
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
