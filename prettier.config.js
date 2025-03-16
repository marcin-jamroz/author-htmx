module.exports = {
  plugins: [ "prettier-plugin-jinja-template", "prettier-plugin-tailwindcss"],
  tailwindStylesheet: "./assets/main.css",
  overrides: [
    {
      files: ["*.html"],
      options: {
        parser: "jinja-template"
      }
    }
  ]
};
