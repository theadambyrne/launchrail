# launchrail

[![Build and Test](https://github.com/theadambyrne/launchrail/actions/workflows/build_test.yml/badge.svg?branch=main)](https://github.com/theadambyrne/launchrail/actions/workflows/build_test.yml)

Experimenting with the idea of creating a 6-DOF High Power Rocketry simulator. This is a work in progress and is not yet functional.

## Status

Right now I have a 3-DOF simulator which simulates a rocket (mass with drag and inertia) being launched from a point, with event/altitude trigger parachutes and solid motor loading from thrust_curve.
Ancillary script to plot the trajectory, note due to lack of features graphs and numbers maybe incorrect.

![3D Trajectory Plot](/outputs/trajectory.png?raw=true "Trajectory")

## Roadmap

No promises.

- [x] 3-DOF simulation
- [x] Parachute deployment
- [x] Thrust curve loading
- [ ] Power on/off drag 
- [ ] Aerodynamics model
- [ ] Environment model
- [ ] 6-DOF simulation
- [ ] GUI


## Getting Started

```bash
git clone https://github.com/theadambyrne/launchrail.git
cd launchrail

# Run the simulation
cargo run --release

# Plot the trajectory
pip install matplotlib pandas
python plot_trajectory.py
```


