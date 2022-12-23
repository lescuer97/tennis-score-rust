import { serve } from "https://deno.land/std@0.152.0/http/server.ts";
import {
  serveDir,
  serveFile,
} from "https://deno.land/std@0.152.0/http/file_server.ts";
    import esbuildSetup from './esbuild.ts'
import { config } from "./deps.ts";

const { PORT } = await config({ safe: true });

await esbuildSetup();

const controller = (req: Request) => {
  const pathname = new URL(req.url).pathname;

  setTimeout(() => {}, 800000);
  if (pathname == "/") {
    return serveFile(req, "public/index.html");
  }

  if (pathname.startsWith("/")) {
    return serveDir(req, {
      fsRoot: "public",
    });
  }
  return new Response();
};

console.log(`HTTP webserver running. Access it at: http://0.0.0.0:${PORT}/`);
await serve(controller, { port: Number(PORT), hostname: "0.0.0.0" });
// deno run  --allow-env --allow-net --allow-read --allow-run index.ts

