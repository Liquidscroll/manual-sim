import init, { Drivetrain } from "../sim-core-rs/pkg/sim_core_rs.js";

await init();
const dt = new Drivetrain();

console.log("Default Drivetrain:");
console.log({
  rpm: dt.rpm(),
  speed_kmh: dt.speed_kmh(),
  gear: dt.gear(),
  throttle: dt.throttle(),
  clutch: dt.clutch(),
  engine_on: dt.engine_on(),
});

