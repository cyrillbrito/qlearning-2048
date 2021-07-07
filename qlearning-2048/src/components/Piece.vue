<template>
  <div class="piece-border">
    <div class="piece-center" v-bind:style="{ backgroundColor: color }">
      {{ display }}
    </div>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, ref } from "vue";

export default defineComponent({
  name: "Piece",
  props: {
    value: Number,
  },
  setup(props) {
    return {
      display: computed(() => Math.pow(2, props.value ?? 0)),
      color: computed(() => `hsl(${25 + 20 * (props.value ?? 0)}, 40%, 50%)`),
    };
  },
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.piece-border {
  background-color: darkslategray;
  width: 100px;
  height: 100px;
  border-radius: 10px;
  padding: 2px;
  box-sizing: border-box;
  margin: 10px;
  transition-duration: 0.2s;
  transition-timing-function: ease-in;
  position: absolute;
}
.piece-center {
  border-radius: 9px;
  line-height: 92px;
  color: white;
  font-size: 34px;
  font-family: "Lilita One", cursive;
  -webkit-text-stroke-width: 2px;
  -webkit-text-stroke-color: darkslategray;
}
</style>
