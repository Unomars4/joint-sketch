<script setup lang="ts">
import { onMounted, useTemplateRef } from 'vue';

const CELL_WIDTH = 20;

const { clientWidth, clientHeight } = document.querySelector("body")!;
const width = clientWidth * devicePixelRatio, height = clientHeight * devicePixelRatio;
const canvasRef = useTemplateRef<HTMLCanvasElement>("playground")

onMounted(() => {
  if (!canvasRef.value) return;
  const canvas = canvasRef.value;
  const ctx = canvas.getContext("2d")!;

  ctx.fillStyle = "orange";
  ctx.strokeStyle = "grey";
  ctx.lineWidth = 2;
  ctx.lineJoin = "round";

  for (let x = 0; x <= width / CELL_WIDTH; x++) {
    const x_ordinate = x * CELL_WIDTH;
    ctx.moveTo(x_ordinate, 0)
    ctx.lineTo(x_ordinate, height);
    for (let y = 0; y <= height / CELL_WIDTH; y++) {
      const y_ordinate = y * CELL_WIDTH;
      ctx.moveTo(0, y_ordinate);
      ctx.lineTo(width, y_ordinate);
    }
  }

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
#playground {}
</style>
