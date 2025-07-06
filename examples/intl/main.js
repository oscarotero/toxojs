const number = 123456.789;
const format = new Intl.NumberFormat("es-ES", {
  style: "currency",
  currency: "EUR",
});

console.log(format.format(number));
