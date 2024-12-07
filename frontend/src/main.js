import init, { life } from "../rust-dependencies/game_of_life.js";

const infiniteCheckbox = document.getElementById("infinite");
const infiniteLabel = document.getElementById("infinite-label");
const iterationsInput = document.getElementById("iterations");
const form = document.getElementById("config-form");
const configForm = document.getElementById("config-form");
const sizeInput = document.getElementById("board-size");
const speedInput = document.getElementById("speed");

document.addEventListener("DOMContentLoaded", async () => {
  await init();
});

infiniteCheckbox.addEventListener("change", () => {
  iterationsInput.disabled = infiniteCheckbox.checked;
  infiniteLabel.style.opacity = infiniteCheckbox.checked ? 1 : 0.6;
});

form.addEventListener("submit", (e) => {
  e.preventDefault();
  configForm.style.opacity = 0;

  let boardSize = parseInt(sizeInput.value);
  let speed = parseInt(speedInput.value);
  let infinite = infiniteCheckbox.checked;
  let iterations = parseInt(iterationsInput.value);

  let canvas = document.getElementById("canvas");
  const minDimension = Math.min(window.innerWidth, window.innerHeight);
  const cellSize = Math.floor(minDimension / boardSize);
  canvas.width = cellSize * boardSize;
  canvas.height = cellSize * boardSize;

  let iteration = 0;

  const render = () => {
    if (!infinite && iteration >= iterations) {
      return;
    }

    life(iteration, boardSize);
    iteration += 1;

    setTimeout(() => {
      requestAnimationFrame(render);
    }, speed);
  };

  render();
});
