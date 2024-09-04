const { defineConfig } = require("@vue/cli-service");
module.exports = defineConfig({
  transpileDependencies: true,
  devServer: {
    client: {
      webSocketURL: {
        protocol: "wss",
        hostname: "localhost",
        port: "8080",
        pathname: "/ws",
      },
    },
    allowedHosts: [
      "xxprmdidsnetjutwsmbs23clklfnmpi5l4gtezq557czq5xafxe4lxid.onion",
    ],
  },
});
