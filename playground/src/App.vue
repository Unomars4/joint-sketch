<script setup lang="ts">
const { clientWidth, clientHeight } = document.querySelector("body")!;
const width = clientWidth * devicePixelRatio, height = clientHeight * devicePixelRatio;

</script>

<template>
  <canvas id="playground" :width="width" :height="height" />
</template>

<style scoped>
#playground {
  border: solid 10px black;
}
</style>
