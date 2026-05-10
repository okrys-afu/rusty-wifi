use crate::wifi::AccessPoint;

pub struct AccessPointGUI {
    raw: AccessPoint,
    points: Vec<[f64; 2]>,
}

impl AccessPointGUI {
    pub fn new(ap: AccessPoint) -> Self {
        Self {
            points: Self::build_points(&ap),
            raw: ap,
        }
    }

    pub fn raw(&self) -> &AccessPoint {
        &self.raw
    }

    pub fn points(&self) -> &Vec<[f64; 2]> {
        &self.points
    }

    pub fn scan() -> Vec<Self> {
        AccessPoint::scan()
            .into_iter()
            .map(AccessPointGUI::new)
            .collect()
    }

    fn build_points(w: &AccessPoint) -> Vec<[f64; 2]> {
        let b = w.bandwidth() / 20;

        let start = (w.channel() - b) * 50;
        let end = (w.channel() + b) * 50;

        let b = b as f64;
        let s = (*w.signal() as f64 / 10.0).max(0.5);
        let c = *w.channel() as f64;

        (start..=end)
            .map(|x| {
                // go back to original point
                let x = x as f64 / 50.0;
                let y: f64 = s * (1.0 - ((x - c) / b).powi(2));
                [x, y]
            })
            .collect::<Vec<[f64; 2]>>()
    }
}
