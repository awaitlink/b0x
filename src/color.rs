use super::colored::*;

/// Print which type is recognized
pub fn found(data: &str, ty: &str) {
    let data = data.red().bold();
    let ty = ty.yellow().bold();

    println!("{} {}({})", "found".white().bold(), ty, data);
}
