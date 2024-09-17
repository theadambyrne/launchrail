use crate::rocket::Rocket;
use nalgebra as na;

pub struct Environment {
    pub gravity: na::Vector3<f64>,
    pub air_density: f64,
    pub wind_velocity: na::Vector3<f64>,
}

pub struct SimulationState {
    pub time: f64,
    pub rocket: Rocket,
    pub environment: Environment,
}

impl SimulationState {
    // Calculate forces acting on the rocket (gravity, thrust, and drag)
    pub fn calculate_forces(&self) -> na::Vector3<f64> {
        let gravity = self.environment.gravity * self.rocket.mass;

        // Thrust logic based on burn time of the solid motor
        let thrust = if self.time < self.rocket.motor.burn_time {
            self.rocket.orientation
                * na::Vector3::new(0.0, 0.0, self.rocket.motor.thrust_at(self.time))
        } else {
            na::Vector3::zeros()
        };

        let velocity_relative_to_air = self.rocket.velocity - self.environment.wind_velocity;
        let velocity_magnitude = velocity_relative_to_air.magnitude();
        let drag_direction = -velocity_relative_to_air.normalize();
        let drag = 0.5 // 0.5 * rho * v^2 * Cd * A
            * self.environment.air_density
            * velocity_magnitude.powi(2)
            * self.rocket.drag_coefficient
            * self.rocket.reference_area
            * drag_direction;

        let parachute_drag =
            self.rocket
                .parachutes
                .iter()
                .fold(na::Vector3::zeros(), |sum, parachute| {
                    if parachute.deployed {
                        sum + 0.5
                            * self.environment.air_density
                            * self.rocket.velocity.magnitude().powi(2)
                            * parachute.cd
                            * self.rocket.reference_area
                            * -self.rocket.velocity.normalize()
                    } else {
                        sum
                    }
                });

        gravity + thrust + drag + parachute_drag
    }

    // Simulate one step of the simulation
    pub fn simulate_step(&mut self, dt: f64) {
        let forces = self.calculate_forces();

        // Update linear motion
        let acceleration = forces / self.rocket.mass;
        self.rocket.velocity += acceleration * dt;
        self.rocket.position += self.rocket.velocity * dt;

        // Update angular velocity and orientation (placeholder for angular acceleration)
        let angular_acceleration = na::Vector3::new(0.0, 0.0, 0.0); // Placeholder for angular acceleration
        self.rocket.angular_velocity += angular_acceleration * dt;
        let delta_orientation =
            na::UnitQuaternion::from_scaled_axis(self.rocket.angular_velocity * dt);
        self.rocket.orientation = delta_orientation * self.rocket.orientation;

        // Parachute deployment logic
        let apogee_reached = self.rocket.velocity.z <= 0.0;
        self.rocket.deploy_parachutes(apogee_reached);

        // Ground collision check
        if self.rocket.position.z < 0.0 {
            self.rocket.position.z = 0.0;
            if self.rocket.velocity.z < 0.0 {
                self.rocket.velocity.z = -self.rocket.velocity.z * 0.5;
                self.rocket.velocity.x *= 0.9;
                self.rocket.velocity.y *= 0.9;
            }
        }

        // Update time
        self.time += dt;
    }
}
