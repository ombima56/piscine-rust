use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(name: &str, d: &str) -> Self {
        Flag {
            short_hand: format!("-{}", &name[0..1]),
            long_hand: format!("--{}", name),
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert((flag.short_hand, flag.long_hand), func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        for ((short, long), callback) in &self.flags {
            if input == short || input == long {
                if argv.len() < 2 {
                    return Err("Not enough arguments".to_string());
                }
                
                match callback(argv[0], argv[1]) {
                    Ok(result) => return Ok(result),
                    Err(e) => return Err(e.to_string()),
                }
            }
        }
        
        Err(format!("Flag {} not found", input))
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a_num = a.parse::<f64>()?;
    let b_num = b.parse::<f64>()?;
    
    if b_num == 0.0 {
        return Ok("inf".to_string());
    }
    
    Ok((a_num / b_num).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a_num = a.parse::<f64>()?;
    let b_num = b.parse::<f64>()?;
    
    if b_num == 0.0 {
        return Ok("NaN".to_string());
    }
    
    Ok((a_num % b_num).to_string())
}