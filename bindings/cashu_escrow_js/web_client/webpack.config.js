const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
    crossOriginLoading: "anonymous",
  },
  mode: "development",
  plugins: [new CopyWebpackPlugin({ patterns: ["index.html"] })],
  experiments: {
    asyncWebAssembly: true,
    syncWebAssembly: true,
  },
  devServer: {
    static: {
      directory: path.join(__dirname, "dist"),
    },
    port: 8081,
    headers:  {
      "Access-Control-Allow-Origin": "*",
    },
    compress: true,
  },
};
