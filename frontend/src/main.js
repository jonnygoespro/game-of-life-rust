import init, { life } from "../rust-dependencies/game_of_life.js";

async function run() {
  await init();

  document.getElementById("test-button").addEventListener("click", () => {
    const iterations = 20;
    const gameStates = life(iterations);
    console.log(gameStates);
  });
}

run();
