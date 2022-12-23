import * as esbuild from "https://deno.land/x/esbuild@v0.15.10/mod.js";

    export default async function buildCode(){
// const ts = "let test: boolean = true";
const result = await esbuild.build({
  entryPoints: [
    "./src/js/app.ts",
    "./src/css/index.css",
    "./src/manifest.json",
  ],
  bundle: false,
  minify: false,
  sourcemap: true,
  // target: ["chrome58", "firefox57", "safari11", "edge16"],
  outdir: "public",
  loader: { ".json": "copy", ".ts": "ts", ".jpg": "file" },
  allowOverwrite: true,
  //   outfile: "out.js",
});
// const result = await esbuild.transform(ts, { loader: "ts" });
console.log("result:", result);
esbuild.stop();
}
