use std::io;

fn print_type_of<T>( _: &T ) {
    println!( "{}", std::any::type_name::<T>() )
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let ruta = stdin.read_line( &mut buffer )?;
    println!( "{:?}: {:?}", ruta, buffer );
    print_type_of( &buffer );
    Ok(())
}