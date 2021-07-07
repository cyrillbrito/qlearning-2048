<template>
  <div class="board">
    <Piece
      v-for="piece in managedPieces"
      :key="piece.id"
      v-bind:value="piece.value"
      v-bind:style="{ left: piece.x, top: piece.y, zIndex: piece.z }"
    />
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, PropType, ref, watch } from "vue";
import Piece from "./Piece.vue";

export interface PieceAction {
  id: number;
  action: "add" | "move" | "upgrade" | "delete";
  position: number;
}

interface PieceInternal {
  id: number;
  action: "add" | "move" | "upgrade" | "delete";

  position: number;
  x?: string;
  y?: string;
  z?: number;
}

export default defineComponent({
  name: "Board",
  components: { Piece },
  props: {
    pieces: Array as PropType<PieceDef[]>,
  },
  setup(props, context) {
    console.log(context);

    let prevPieces = props.pieces ?? [];

    const deletee = (pp: PieceDef, pieces: PieceDef[]) => {
      pp.z = -1;
      pieces.push(pp);
      setTimeout(() => {
        const index = pieces.indexOf(pp);
        pieces.splice(index, 1);
      }, 500);
    };

    var managedPieces = computed((ctx) => {
      console.log(ctx);

      const pieces = props.pieces ?? [];

      for (const piece of pieces) {
        piece.x = (piece.position % 3) * 120 + "px";
        piece.y = Math.floor(piece.position / 3) * 120 + "px";
      }

      for (const pp of prevPieces) {
        const deleted = pieces.every((p) => p.id !== pp.id);
        if (deleted) {
          deletee(pp, pieces);
        }
      }

      prevPieces = props.pieces ?? [];

      return pieces;
    });

    return {
      managedPieces,
    };
  },
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.board {
  position: relative;
}
</style>
