mod parachute;
mod rocket;
mod simulation;
mod solid_motor;
mod thrust_curve;

use std::fs::File;
use std::io::Write;

use chrono::Local;
use env_logger::Builder;
use log::{debug, info, warn, LevelFilter};
use nalgebra as na;
use rand::Rng;

use parachute::{DeploymentConfig, Parachute};
use rocket::Rocket;
use simulation::{Environment, SimulationState};
use solid_motor::SolidMotor;

fn main() -> std::io::Result<()> {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Debug)
        .write_style(env_logger::WriteStyle::Auto)
        .init();

    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..2.0 * std::f64::consts::PI);
    let wind_speed = 10.0;
    let wind_velocity = na::Vector3::new(wind_speed * angle.cos(), wind_speed * angle.sin(), 0.0);
    debug!("Wind velocity: {:?}", wind_velocity);

    let thrust_curve = thrust_curve::ThrustCurve::load_from_eng("./inputs/sample_3G_L645.eng")?;
    let solid_motor = SolidMotor::new(thrust_curve, 5.33);
    debug!("Solid motor loaded from .eng file");

    let drogue_parachute = Parachute::new(DeploymentConfig::Apogee, 0.8);
    let main_parachute = Parachute::new(DeploymentConfig::Altitude(450.0), 0.8);
    debug!("Parachutes loaded");

    let reference_area = 0.00933131557; // NOTE: RA = π * (D/2)^2 (at largest diameter D)
    let drag_coefficient = 0.006125; // NOTE: 5.589E+04 = 0.5 * 1.225 * 0.1 * 0.1
    debug!(
        "Using sample RA and CD values: RA = {}, CD = {}",
        reference_area, drag_coefficient
    );

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

    info!("Starting simulation");
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
            debug!(
                "sim_time: {:.1}s, z_dist: {:.2}m, z_speed: {:.2}m/s",
                state.time, state.rocket.position.z, state.rocket.velocity.z
            );
        }

        if state.rocket.position.z <= 0.01 && state.rocket.velocity.magnitude() < 0.1 {
            info!("Rocket has landed. Simulation ended at {:.2}s", state.time);
            break;
        }

        if state.time > 500.0 {
            warn!("Simulation timed out at 300 seconds");
            break;
        }
    }

    info!("Simulation complete. Results written to rocket_simulation.csv");
    Ok(())
}
