use super::colored::*;

/// Print `a` in bold blue, and `b` in bold green
pub fn print(a: &str, b: &str) {
    let a = a.blue().bold();
    let b = b.green().bold();

    println!("{} {}", a, b);
}

/// Print `"found"` in bold white, `data` in bold red, and `ty` in bold yellow according to this format:
/// `found ty(data)`
pub fn found(data: &str, ty: &str) {
    let data = data.red().bold();
    let ty = ty.yellow().bold();

    println!("{} {}({})", "found".white().bold(), ty, data);
}
