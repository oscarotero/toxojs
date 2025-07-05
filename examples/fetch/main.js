const url = "https://oscarotero.com";

const abortSignal = new AbortController();

fetch(url, {
  signal: abortSignal.signal,
}).then((response) => {
  console.log("Response received:", response.status);
  return response.text();
}).catch((error) => {
  if (error.name === "AbortError") {
    console.log("Request was aborted");
  } else {
    console.error("Fetch error:", error);
  }
});

abortSignal.abort(); // Abort the request

const response = await fetch(url);
const text = await response.text();
console.log("Request completed");
console.log(text.length);
