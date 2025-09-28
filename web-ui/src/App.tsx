import { useState, useEffect } from "react";
import init, { Drivetrain } from "sim-core-rs";
import wasmUrl from "sim-core-rs/sim_core_rs_bg.wasm?url";

import "./App.css";

export default function App() {
  useEffect(() => {
    (async () => {
      await init(wasmUrl);
      const dt = new Drivetrain();
      console.log("Browser default Drivetrain:", {
        rpm: dt.rpm(),
        speed_kmh: dt.speed_kmh(),
        gear: dt.gear(),
        throttle: dt.throttle(),
        clutch: dt.clutch(),
        engine_on: dt.engine_on(),
      });
    })();
  }, []);
  return (
    <div style={{ padding: 16 }}>
      Check the console for Drivetrain defaults.
    </div>
  );
}
