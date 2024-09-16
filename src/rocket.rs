use crate::parachute::DeploymentConfig;
use crate::parachute::Parachute;
use crate::solid_motor::SolidMotor;

use nalgebra as na;

pub struct Rocket {
    pub motor: SolidMotor,
    pub parachutes: Vec<Parachute>,
    pub mass: f64,
    pub position: na::Vector3<f64>,
    pub velocity: na::Vector3<f64>,
    pub orientation: na::UnitQuaternion<f64>,
    pub angular_velocity: na::Vector3<f64>,
    pub drag_coefficient: f64,
    pub reference_area: f64,
}

impl Rocket {
    pub fn new(mass: f64, motor: SolidMotor, parachutes: Vec<Parachute>) -> Self {
        Rocket {
            mass: mass,
            parachutes: parachutes,
            position: na::Vector3::new(0.0, 0.0, 0.0),
            velocity: na::Vector3::new(0.0, 0.0, 0.0),
            orientation: na::UnitQuaternion::from_euler_angles(0.0, 0.1, 0.0),
            angular_velocity: na::Vector3::new(0.0, 0.0, 0.0),
            drag_coefficient: 0.5 * 1.225 * 0.1 * 0.1,
            // NOTE: 5.589E+04 = 0.5 * 1.225 * 0.1 * 0.1
            reference_area: 0.1,
            motor: motor,
        }
    }
    pub fn deploy_parachutes(&mut self, trigger: bool) {
        for parachute in &mut self.parachutes {
            match parachute.deployment_config {
                DeploymentConfig::Apogee => {
                    if trigger
                        && !parachute.deployed
                        && self.velocity.z <= 0.0
                        && self.position.z >= 0.0
                    {
                        parachute.deploy();
                    }
                }
                DeploymentConfig::Altitude(altitude, rising) => {
                    if self.position.z == altitude && !parachute.deployed {
                        if (self.velocity.z > 0.0) == rising {
                            parachute.deploy();
                        }
                    }
                }
            }
        }
    }
}
