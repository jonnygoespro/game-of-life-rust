import init, { life } from "../rust-dependencies/game_of_life.js";

const header = document.getElementById("header");
const infiniteCheckbox = document.getElementById("infinite");
const iterationsInput = document.getElementById("iterations");
const form = document.getElementById("config-form");
const configForm = document.getElementById("config-form");
const sizeInput = document.getElementById("board-size");
const speedInput = document.getElementById("speed");
const distributionInput = document.getElementById("starting-board");
const canvasContainer = document.getElementById("canvas-container");

document.addEventListener("DOMContentLoaded", async () => {
  await init();
});

infiniteCheckbox.addEventListener("change", () => {
  iterationsInput.disabled = infiniteCheckbox.checked;
});

form.addEventListener("submit", (e) => {
  e.preventDefault();
  configForm.style.display = "none";
  canvasContainer.style.display = "block";
  header.style.display = "none";
  document.body.style.display = "block";

  let boardSize = parseInt(sizeInput.value);
  let speed = parseInt(speedInput.value);
  let infinite = infiniteCheckbox.checked;
  let iterations = parseInt(iterationsInput.value);

  let canvas = document.getElementById("canvas");
  const minDimension = Math.min(window.innerWidth, window.innerHeight);
  const cellSize = Math.floor(minDimension / boardSize);
  canvas.width = cellSize * boardSize;
  canvas.height = cellSize * boardSize;

  // Generate a random number seed to have a deterministic outcome of the life function
  const seed = Math.floor(Math.random() * Number.MAX_SAFE_INTEGER).toString();
  console.log("Random Seed:", seed);

  let iteration = 0;

  const render = () => {
    if (!infinite && iteration >= iterations) {
      return;
    }

    life(iteration, boardSize, distributionInput.value == "random", seed);
    iteration += 1;

    setTimeout(() => {
      requestAnimationFrame(render);
    }, speed);
  };

  render();
});
