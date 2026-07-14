use crate::error::{Error, Result};

pub fn validate_country(country: &str) -> Result<()> {
    const COUNTRIES: &[&str] = &["CA", "US"];
    let country_upper = country.to_uppercase();
    if COUNTRIES.contains(&country_upper.as_str()) {
        Ok(())
    } else {
        Err(Error::UnprocessableEntity("Invalid country".into()))
    }
}

pub fn validate_email(email: &mut String) -> Result<()> {
    // todo: regex
    validate_string(email, 10, 50, "Invalid e-mail")
}

pub fn validate_number<T: PartialOrd>(val: T, min_val: T, max_val: T, message: &str) -> Result<()> {
    if val < min_val || val > max_val {
        return Err(Error::UnprocessableEntity(message.into()));
    }
    Ok(())
}

pub fn validate_number_opt<T: PartialOrd>(
    val: Option<T>,
    min_val: T,
    max_val: T,
    message: &str,
) -> Result<()> {
    if let Some(val) = val {
        if val < min_val || val > max_val {
            return Err(Error::UnprocessableEntity(message.into()));
        }
    }
    Ok(())
}

pub fn validate_password(password: &mut String) -> Result<()> {
    // todo: regex
    validate_string(password, 8, 30, "Invalid password")
}

pub fn validate_string(
    str: &mut String,
    min_len: usize,
    max_len: usize,
    message: &str,
) -> Result<()> {
    *str = str.trim().to_string();
    let len = str.len();
    if len < min_len || len > max_len {
        return Err(Error::UnprocessableEntity(message.into()));
    }
    Ok(())
}

pub fn validate_string_opt(
    str: &mut Option<String>,
    min_len: usize,
    max_len: usize,
    message: &str,
) -> Result<()> {
    if let Some(str) = str {
        *str = str.trim().to_string();
        let len = str.len();
        if len < min_len || len > max_len {
            return Err(Error::UnprocessableEntity(message.into()));
        }
    }
    Ok(())
}
