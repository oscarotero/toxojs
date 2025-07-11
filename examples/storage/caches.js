const url = "https://oscarotero.com";
const cache = await caches.open("my-cache");

if (!cache) {
  throw new Error("Cache not available");
}
if (await cache.match(url)) {
  console.log("Cache hit for:", url);
  const cachedResponse = await cache.match(url);
  const cachedHtml = await cachedResponse.text();
  console.log("Cached HTML size:", cachedHtml.length);
} else {
  console.log("Cache miss for:", url);
  const response = await fetch("https://oscarotero.com");
  if (!response.ok) {
    throw new Error(`Network response was not ok: ${response.statusText}`);
  }
  cache.put(url, response.clone());
  console.log("Response cached for:", url);
  console.log("Response HTML size:", (await response.text()).length);
  console.log("Response status:", response.status);
}
