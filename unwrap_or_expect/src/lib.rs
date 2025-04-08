pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => server.unwrap().to_string(),

        Security::Message => match server {
            Ok(url) => url.to_string(),
            Err(_) => panic!("ERROR: program stops"),
        },

        Security::Warning => match server {
            Ok(url) => url.to_string(),
            Err(_) => "WARNING: check the server".to_string(),
        },

        Security::NotFound => match server {
            Ok(url) => url.to_string(),
            Err(msg) => format!("Not found: {}", msg),
        },

        Security::UnexpectedUrl => match server {
            Ok(url) => panic!("{}", url),
            Err(msg) => msg.to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_data_ok_and_notfound() {
        assert_eq!(fetch_data(Ok("server1.com"), Security::Unknown), "server1.com");
        assert_eq!(fetch_data(Err("server2.com"), Security::NotFound), "Not found: server2.com");
        assert_eq!(fetch_data(Err("server.com"), Security::Warning), "WARNING: check the server");
    }

    #[test]
    #[should_panic]
    fn test_fetch_data_unknown_panic() {
        fetch_data(Err("server.com"), Security::Unknown);
    }

    #[test]
    #[should_panic(expected = "ERROR: program stops")]
    fn test_fetch_data_message_panic() {
        fetch_data(Err("server.com"), Security::Message);
    }

    #[test]
    #[should_panic(expected = "malicious_server.com")]
    fn test_fetch_data_unexpected_url_panic() {
        fetch_data(Ok("malicious_server.com"), Security::UnexpectedUrl);
    }
}

