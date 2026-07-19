const
  PRECACHE = 'precache-v1',
  RUNTIME = 'runtime',
  PRECACHE_URLS = [
    '/favicon.ico',
    '/logo.svg',
    '/manifest.json',
  ];

self.addEventListener('install', e => e.waitUntil(caches
  .open(PRECACHE)
  .then(cache => cache.addAll(PRECACHE_URLS))
  .then(self.skipWaiting())
));

self.addEventListener('activate', e => e.waitUntil(caches
  .keys()
  .then(cacheNames => cacheNames
  .filter(cacheName => ![PRECACHE, RUNTIME]
    .includes(cacheName)))
  .then(cachesToDelete => Promise
  .all(cachesToDelete
    .map(cacheToDelete => caches
    .delete(cacheToDelete))))
  .then(() => self.clients.claim())
));

self.addEventListener('fetch', e => {
  if (e.request.url.startsWith(self.location.origin)) {
  e.respondWith(
    caches.match(e.request).then(cachedResponse => {
    if (cachedResponse) {
      return cachedResponse;
    }
    return caches
      .open(RUNTIME)
      .then(cache => fetch(e.request)
      .then(response => cache
        .put(e.request, response.clone())
        .then(() => response)));
    })
  );
  }
});
