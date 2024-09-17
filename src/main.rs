mod parachute;
mod rocket;
mod simulation;
mod solid_motor;
mod thrust_curve;

use nalgebra as na;
use parachute::{DeploymentConfig, Parachute};
use rand::Rng;
use rocket::Rocket;
use simulation::{Environment, SimulationState};
use solid_motor::SolidMotor;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..2.0 * std::f64::consts::PI);
    let wind_speed = 10.0;
    let wind_velocity = na::Vector3::new(wind_speed * angle.cos(), wind_speed * angle.sin(), 0.0);

    let thrust_curve = thrust_curve::ThrustCurve::load_from_eng("./inputs/sample_3G_L645.eng")?;
    let solid_motor = SolidMotor::new(thrust_curve, 5.33);

    let drogue_parachute = Parachute::new(DeploymentConfig::Apogee, 0.8);
    let main_parachute = Parachute::new(DeploymentConfig::Altitude(450.0, false), 0.8);

    let reference_area = 0.00933131557; // NOTE: RA = π * (D/2)^2 (at largest diameter D)
    let drag_coefficient = 0.006125; // NOTE: 5.589E+04 = 0.5 * 1.225 * 0.1 * 0.1

    let mut state = SimulationState {
        time: 0.0,
        rocket: Rocket::new(
            9.0,
            solid_motor,
            vec![main_parachute, drogue_parachute],
            drag_coefficient,
            reference_area,
        ),
        environment: Environment {
            gravity: na::Vector3::new(0.0, 0.0, -9.81),
            air_density: 1.225,
            wind_velocity,
        },
    };

    let dt = 0.01;
    let mut file = File::create("./outputs/position.csv")?;
    writeln!(file, "Time,X,Y,Z,Vx,Vy,Vz")?;

    loop {
        state.simulate_step(dt);

        writeln!(
            file,
            "{},{},{},{},{},{},{}",
            state.time,
            state.rocket.position.x,
            state.rocket.position.y,
            state.rocket.position.z,
            state.rocket.velocity.x,
            state.rocket.velocity.y,
            state.rocket.velocity.z
        )?;

        if state.time.fract() < dt {
            println!(
                "Time: {:.1}s, Height: {:.2}m, Vertical Speed: {:.2}m/s",
                state.time, state.rocket.position.z, state.rocket.velocity.z
            );
        }

        if state.rocket.position.z <= 0.01 && state.rocket.velocity.magnitude() < 0.1 {
            println!("Rocket has landed. Simulation ended at {:.2}s", state.time);
            break;
        }

        if state.time > 500.0 {
            println!("Simulation timed out at 300 seconds");
            break;
        }
    }

    println!("Simulation complete. Results written to rocket_simulation.csv");
    Ok(())
}
