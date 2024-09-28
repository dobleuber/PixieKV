use std::env;

fn main() {
    // Obtén el sysroot de arm-none-eabi
    let sysroot = "/usr/lib/arm-none-eabi"; // Asegúrate de que esta ruta sea correcta en tu sistema

    // Ajustar la variable de entorno para las bibliotecas del linker
    println!("cargo:rustc-link-search=native={}/lib", sysroot);

    // No uses `-I` aquí, ya que no es relevante para el linker
    println!("cargo:rustc-link-arg=-L{}/lib", sysroot);

    // Configuración adicional si se compila para thumbv7m-none-eabi
    let target = env::var("TARGET").expect("TARGET not set");
    if target.contains("thumbv7m-none-eabi") {
        println!("cargo:rustc-link-arg=-Tlink.x"); // Reemplaza 'link.x' con tu archivo de script de enlace si es necesario
    }
}
