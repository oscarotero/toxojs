// Note: the localStorage is stored in the same directory as the entry point.
{
  let number = parseInt(localStorage.getItem("number") || "0", 10);
  number += 1;
  localStorage.setItem("number", number.toString());
  console.log("Number in localStorage:", number);
}

{
  let number = parseInt(sessionStorage.getItem("number") || "0", 10);
  number += 1;
  sessionStorage.setItem("number", number.toString());
  console.log("Number in sessionStorage:", number);
}
