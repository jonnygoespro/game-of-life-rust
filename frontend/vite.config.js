export default {
  root: "src",
  build: {
    outDir: "../dist",
    emptyOutDir: true,
    rollupOptions: {
      input: {
        main: "./src/index.html",
        rules: "./src/rules.html",
        burger: "./src/burger-menu.js",
      },
    },
  },
};
