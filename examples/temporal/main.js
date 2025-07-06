const dur1 = Temporal.Duration.from({ years: 1 });
const dur2 = Temporal.Duration.from({ months: 1 });

const startingPoint = Temporal.PlainDate.from("2021-01-01");
console.log(startingPoint.add(dur1).add(dur2).since(startingPoint));
