use console::style;

const ERROR_PREFIX: &str = "Error: ";

pub fn format_error_message(message: &str) -> String {
    let err_prefix_red = format!("{}", style(ERROR_PREFIX).red().bold());

    format!("{err_prefix_red} {message}")
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    unused
)]
fn to_readable_size(size: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB", "PB"];

    if size == 0 {
        // size is 0, so this is safe
        return format!("{size} {}", units[size as usize]);
    }

    // size is always positive, so this is safe
    let exp = (size as f64).log(1024.0).floor() as u64;

    // precision loss is ok here because we are only interested in the integer part
    let pot = 1024f64.powf(exp as f64);

    // precision loss is ok here because we are only interested in the integer part
    let res = size as f64 / pot;

    // exp is always positive, so this is safe
    format!("{res:.0} {}", units[exp as usize])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "does not work without a terminal"]
    fn test_format_error_message() {
        let message = "We have a problem.";
        assert_eq!(
            "\u{1b}[31m\u{1b}[1mError: \u{1b}[0m We have a problem.",
            format_error_message(message)
        );
    }

    #[test]
    fn test_to_readable_zero() {
        let size = 0u64;
        assert_eq!("0 B", to_readable_size(size));
    }

    #[test]
    fn test_to_readable_b() {
        let size = 12u64;
        assert_eq!("12 B", to_readable_size(size));
    }

    #[test]
    fn test_to_readable_kb() {
        let size = 12500_u64;
        assert_eq!("12 KB", to_readable_size(size));
    }

    #[test]
    fn test_to_readable_mb() {
        let size = 12_500_000_u64;
        assert_eq!("12 MB", to_readable_size(size));
    }

    #[test]
    fn test_to_readable_gb() {
        let size = 12_500_000_000_u64;
        assert_eq!("12 GB", to_readable_size(size));
    }

    #[test]
    fn test_to_readable_tb() {
        let size = 12_500_000_000_000_u64;
        assert_eq!("11 TB", to_readable_size(size));
    }

    #[test]
    fn test_to_readable_pb() {
        let size = 12_500_000_000_000_000_u64;
        assert_eq!("11 PB", to_readable_size(size));
    }
}
