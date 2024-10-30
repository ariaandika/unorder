export default {
  content: ["./*.html","./templates/**/*.html"],
  theme: { extend: {} },
  plugins: [
    ({ addBase }) => {
      addBase({
        ".app": {
          background: "red",
        },
      });
    },
  ],
};

