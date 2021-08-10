const webpack = require('webpack');
var UglifyJsPlugin = require('uglifyjs-webpack-plugin');
var path = require("path");

module.exports = {
    mode: 'production',
    entry: { site: './app.js' },
    output: {
        path: path.resolve(__dirname, "../"),
        filename: "[name].js",
        publicPath: "/dist"
    },
    module: {
        rules: []
    },
    plugins: [
        // make sure to include the plugin!
        new VueLoaderPlugin(),
        new MiniCssExtractPlugin({
            filename: "[name].css",
            chunkFilename: "[id].css"
        })
    ],
    optimization: {
        minimize: true,
        usedExports: false,
        sideEffects: false
      },
    watchOptions: {
        ignored: /node_modules/
    }
}