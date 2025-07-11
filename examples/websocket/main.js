const socket = new WebSocket("ws://localhost:8080");

socket.addEventListener("error", function (event) {
  console.error("WebSocket error observed:", event);
});
socket.addEventListener("open", function (event) {
  console.log("WebSocket connection established.");
  socket.send("Hello Server!");
});
socket.addEventListener("message", function (event) {
  console.log("Message from server:", event.data);
});
socket.addEventListener("close", function (event) {
  console.log("WebSocket connection closed:", event);
});
