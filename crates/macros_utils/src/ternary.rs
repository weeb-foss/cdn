#[macro_export]
macro_rules! ternary {
    ($condition:expr => $true_expr:expr ; $false_expr:expr) => {
        if $condition { $true_expr } else { $false_expr }
    };
}
