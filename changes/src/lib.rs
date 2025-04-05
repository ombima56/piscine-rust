#[derive(Debug, Eq, PartialEq, Clone)]

pub struct Light {
    pub alias: String,
    pub brightness: u8,
}

impl Light {
    pub fn new(alias: &str) -> Self {
        Light {
            alias: alias.to_string(),
            brightness: 0,
        }
    }
}

pub fn change_brightness(lights: &mut [Light], alias: &str, value: u8) {
    for light in lights.iter_mut() {
        if light.alias == alias {
            light.brightness = value;
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_light_new() {
        let light = Light::new("kitchen");
        assert_eq!(light.alias, "kitchen");
        assert_eq!(light.brightness, 0);
    }

    #[test]
    fn test_change_brightness_success() {
        let mut lights = ["kitchen", "bedroom", "bathroom"].map(Light::new);
        change_brightness(&mut lights, "kitchen", 150);
        assert_eq!(lights[0].brightness, 150);
    }

    #[test]
    fn test_change_brightness_not_found() {
        let mut lights = ["kitchen", "bedroom", "bathroom"].map(Light::new);
        change_brightness(&mut lights, "garage", 200);
        for light in lights.iter() {
            assert_eq!(light.brightness, 0);
        }
    }
}
