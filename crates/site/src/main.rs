fastly_static_site::serve!({
    directory: "../../dist",
    exclude: ["*.map"],
    root: "index.html",
});
