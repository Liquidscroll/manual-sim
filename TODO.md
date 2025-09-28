1. Setters (inputs) + no-op step(dt)
   - Add set_inputs(throttle, clutch, gear, engine_on) in Rust.
   - Native test: setting values reflects via getters.
   - Bun script: set inputs, print again.

2. Fixed-step integrator skeleton
   - Add step(dt, grade_percent) with no forces (just clamps/stall guards).
   - Native test: calling step doesn’t crash; state remains bounded.

3. Engine torque curve function (pure, testable)
   - Unit tests on monotonicity vs throttle; values in expected ranges.

4. Clutch slip → torque (sign + clamp)
   - Add a pure function for slip torque; unit tests verify sign opposes slip.

5. Wheel & vehicle update with simple resistive forces
   - Unit tests: acceleration sign matches torque; speed never < 0.

6. Expose extra getters
   - slip_rpm(), target_rpm_for_gear(gear), etc.
   - Bun script prints those too.

7. React UI minimal controls
   - Sliders for throttle/clutch, buttons for gear, text output of rpm/speed.
   - Keep headless tests running in parallel.
