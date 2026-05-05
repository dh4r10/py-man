#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("img/logo_sin_fondo.ico");
    res.compile().expect("failed to compile Windows resources");
}

#[cfg(not(windows))]
fn main() {}
