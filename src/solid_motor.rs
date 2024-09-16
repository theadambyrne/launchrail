use crate::thrust_curve;

pub struct SolidMotor {
    pub thrust_curve: thrust_curve::ThrustCurve,
    pub burn_time: f64,
}

impl SolidMotor {
    pub fn new(thrust_curve: thrust_curve::ThrustCurve, burn_time: f64) -> Self {
        SolidMotor {
            thrust_curve,
            burn_time,
        }
    }

    pub fn thrust_at(&self, time: f64) -> f64 {
        self.thrust_curve.thrust_at(time)
    }
}
