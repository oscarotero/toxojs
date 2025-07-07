alert("This is an alert message!");
if (confirm("Do you want to continue?")) {
  const name = prompt("What is your name?", "Guest");
  console.log(`Hello, ${name}!`);
}
