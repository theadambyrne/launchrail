pub enum DeploymentConfig {
    Apogee,
    Altitude(f64, bool), // NOTE: metres, up/down (true/false)
}

pub struct Parachute {
    pub deployment_config: DeploymentConfig,
    pub cd: f64,
    pub deployed: bool,
}

impl Parachute {
    pub fn new(deployment_config: DeploymentConfig, cd: f64) -> Self {
        Parachute {
            deployment_config,
            cd,
            deployed: false,
        }
    }

    pub fn deploy(&mut self) {
        self.deployed = true;
    }
}
