const staticCacheName = "site-static-v1.4";
// used for prefetching assets
const assests = [
  "/",
  "/index.html",
  "/js/app.js",
  "/css/index.css",
  "/favicon.ico",
  "/manifest.json",
  "/img/qr.jpg",
];
// install service worker
self.addEventListener("install", (e) => {
  console.log("service worker installed");
  console.log({ e });
  e.waitUntil(
    (async () => {
      const cache = await caches.open(staticCacheName);
      console.log("[Service Worker] Caching all: app shell and content");
      await cache.addAll(assests);
    })()
  );
});

self.addEventListener("activate", (e) => {
  e.waitUntil(
    caches.keys().then((keyList) => {
      return Promise.all(
        keyList.map((key) => {
          if (key === staticCacheName) {
            return;
          }
          return caches.delete(key);
        })
      );
    })
  );
});

self.addEventListener("fetch", (e) => {
  e.respondWith(
    (async () => {
      const r = await caches.match(e.request);
      console.log(`[Service Worker] Fetching resource: ${e.request.url}`);
      if (r) {
        return r;
      }
      const response = await fetch(e.request);
      const cache = await caches.open(staticCacheName);
      console.log(`[Service Worker] Caching new resource: ${e.request.url}`);
      cache.put(e.request, response.clone());
      return response;
    })()
  );
});
