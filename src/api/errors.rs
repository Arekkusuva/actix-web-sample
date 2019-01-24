pub trait IntoValidationErrorStr {
    fn into_validation_error_str(self) -> &'static str;
}

impl<'a> IntoValidationErrorStr for &'a str {
    fn into_validation_error_str(self) -> &'static str {
        match self {
            "email" => "must be email address",
            "password" => "must have at least 7 characters",
            _ => "validation failed",
        }
    }
}
