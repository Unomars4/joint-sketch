<script setup lang="ts">
import { onMounted, useTemplateRef } from 'vue';

const { clientWidth, clientHeight } = document.querySelector("body")!;
const width = clientWidth * devicePixelRatio, height = clientHeight * devicePixelRatio;
const canvasRef = useTemplateRef<HTMLCanvasElement>("playground")

onMounted(() => {
  if (!canvasRef.value) return;
  const canvas = canvasRef.value;
  const ctx = canvas.getContext("2d")!;

  ctx.fillStyle = "orange";
  ctx.strokeStyle = "black";
  ctx.lineWidth = 5;
  ctx.lineJoin = "round";
  ctx.moveTo(100, 100);
  ctx.lineTo(200, 100);
  ctx.lineTo(200, 200);
  ctx.lineTo(100, 200);
  ctx.closePath();
  ctx.fill();
  ctx.stroke();

  ctx.scale(devicePixelRatio, devicePixelRatio);

  canvas.style.width = `${clientWidth}px`;
  canvas.style.height = `${clientHeight}px`;
})
</script>

<template>
  <canvas id="playground" :width="width" :height="height" ref="playground" />
</template>

<style scoped>
#playground {
  border: solid 10px black;
}
</style>
