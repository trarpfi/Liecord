use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NitroPlans {
    pub basic: NitroPlan,
    pub classic: NitroPlan,
    pub full: NitroPlan,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NitroPlan {
    pub name: String,
    pub price: f64,
    pub features: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribeNitroRequest {
    pub tier: String, // "basic", "classic", "full"
}

impl NitroPlans {
    pub fn default_plans() -> Self {
        Self {
            basic: NitroPlan {
                name: "Nitro Basic".to_string(),
                price: 2.99,
                features: vec![
                    "Custom emoji anywhere".to_string(),
                    "50MB upload limit".to_string(),
                    "Special Nitro badge".to_string(),
                    "Custom avatar decoration".to_string(),
                    "HD video streaming".to_string(),
                ],
            },
            classic: NitroPlan {
                name: "Nitro Classic".to_string(),
                price: 4.99,
                features: vec![
                    "All Basic features".to_string(),
                    "50MB upload limit".to_string(),
                    "Animated avatar".to_string(),
                    "Custom discriminator".to_string(),
                    "HD video streaming".to_string(),
                ],
            },
            full: NitroPlan {
                name: "Nitro".to_string(),
                price: 9.99,
                features: vec![
                    "All Classic features".to_string(),
                    "500MB upload limit".to_string(),
                    "4K video streaming".to_string(),
                    "2 Server Boosts".to_string(),
                    "Animated server icon".to_string(),
                    "Custom profile banner".to_string(),
                    "HD video streaming".to_string(),
                    "Larger Go Live streams".to_string(),
                ],
            },
        }
    }
}
