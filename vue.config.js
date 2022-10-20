module.exports = {
    publicPath: process.env.NODE_ENV === "production" ? "/rekkice-pf/" : "/",
    configureWebpack: {
        experiments: {
            asyncWebassembly: true
        }
    }
}