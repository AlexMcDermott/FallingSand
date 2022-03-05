import Stats from 'stats.js';

import './styles.css';
import { Grid } from '../pkg';

const stats = new Stats();
stats.showPanel(0);
document.body.appendChild(stats.dom);

const canvasScale = 0.8;
const cellSize = 5;

const gridWidth = Math.floor((window.innerWidth * canvasScale) / cellSize);
const gridHeight = Math.floor((window.innerHeight * canvasScale) / cellSize);
const canvasDisplayWidth = gridWidth * cellSize;
const canvasDisplayHeight = gridHeight * cellSize;
const canvasWidth = canvasDisplayWidth * devicePixelRatio;
const canvasHeight = canvasDisplayHeight * devicePixelRatio;

const canvas = document.getElementById('canvas') as HTMLCanvasElement;
canvas.style.width = `${canvasDisplayWidth}px`;
canvas.style.height = `${canvasDisplayHeight}px`;
canvas.width = canvasWidth;
canvas.height = canvasHeight;

const ctx = canvas.getContext('2d');
ctx.scale(devicePixelRatio, devicePixelRatio);

const grid = new Grid(gridWidth, gridHeight);

canvas.addEventListener('mousemove', (event: MouseEvent) => {
  if (event.buttons != 1) return;
  const x = Math.floor((event.offsetX / canvasDisplayWidth) * gridWidth);
  const y = Math.floor((event.offsetY / canvasDisplayHeight) * gridHeight);
  grid.set_cell(x, y);
});

function loop() {
  grid.tick();
  stats.begin();
  grid.render(ctx, cellSize);
  stats.end();
  requestAnimationFrame(loop);
}

requestAnimationFrame(loop);
