//OPTIMIZE: DIA 1 GUIA DE GOOGLE
//TODO: TIPOS ESCALARES
/**
 //TODO ejemplo de variables mutables de espacio de memoria asignado y condicionales
 * 	                                     Types	Literals
Signed integers	i8, i16, i32, i64, i128, isize	-10, 0, 1_000, 123i64
Unsigned integers	u8, u16, u32, u64, u128, usize	0, 123, 10u16
Floating point numbers	f32, f64	3.14, -10.0e20, 2f32
Strings	&str	"foo", "two\nlines"
Unicode scalar values	char	'a', 'α', '∞'
Booleans	bool	true, false
 */
fn small(){
    let mut x: i32 = 6;
    print!("{x}");
    while x != 1 {
        if x % 2 == 0 {
            x = x / 2;
        }else {
            x = 3 * x + 1;
        }
        print!("-> {x}");
    }
    println!();
}
//TODO: Array asingacion y acceso como en todos los lenguajes empieza del indice 0
fn array(){
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    a[9]= 22;
    println!("a: {:?}", a);
}
//TODO Tuplas asignacion y acceso las tuplas se acede con .posicion no pueden ser mutables
fn tuplas(){
    let t: (i8, bool, char) = (7, true, 's');
    println!("1st index: {}", t.0);
    println!("2nd index: {}", t.1);
    println!("3er index: {}", t.2);
}
//TODO referencia a una variable
fn referencia(){
    let mut x: i32 = 10;
    let ref_x: &mut i32 = &mut x;//OPTIMIZE referencia al espacio de memoria ocupado por x
    *ref_x = 20;//OPTIMIZE cambiamos el valor de la referencia como es mutable cambia el valor original 
    println!("x : {x}");//TODO imprimimos el valor original y comprobamos el cambio ya que las referencias que designamos son mutables
}
//TODO: referencia colgante , su vida util
/* fn ref_colgante(){
    //let ref_x: i32 ;
    {
        //let x: i32 = 10;
        //ref_x = &x;//OPTIMIZE esto genera un error ya que la referencia muere dentro del scope para solucionarlo hay que sacar la asignacion de x al contexto global de la funcion  
    }
   // println!("ref_x: {ref_x}");
} */
fn prestamos_referencia(){
    //OPTIMIZE creamos un array de enteros de 32 bit con 6 posiciones cargados con datos
    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    //TODO imprimimos por consola el array
    println!("a: {a:?}");
    //OPTIMIZE creamos otra variable que es una referencia a los enteros de 32 bit y se carga con una referencia a los valores del array a donde toma del valor 2...4 pero siempre los valores se considerena desde el indice inicial hasta el final menos 1
    let s: &[i32] = &a[2..4];
    //TODO imprime los valores de la variable s con los valores prestados 
    println!("s: {s:?}");
    //FIXME importante cuando prestamos valores o referenciamos valores mientras esten siendo utilizados no podemos modificarlos por mas que sean mutables
    //FIXME esto significa que caulqueir operacion de variaxion o mutacion tiene que ser antes de hacerle referencia o despeus de que la referencia sea utilizada sino genera errores
}
//TODO String vs str para tener encuenta &str es un segmento de cadena inmutable y string es un buffer de cadena mutable
fn string_vs_str(){
    let s1: &str = "World";//OPTIMIZE &str referencia de valor str al guan le asignamos el valor de "World"
    println!("s1: {s1}");//TODO impimimos la variable con el valor inpuesto 

    let mut s2: String = String::from("Hello ");//OPTIMIZE generamos una variable mutable cullo valor va a ser un String::from("Hello ") lo cual parece un objeto con un metodo from, que requiere un argumento tipo string 
    println!("s2: {s2}");//TODO imprime la variable creada con el valor inpuesto Hello con un espacio
    s2.push_str(s1); //OPTIMIZE como es tomado como un buffer insertamos como con el metodo push_str() el valor del fracmento de str creado 
    println!("s2: {s2}");//TODO impirmimos la salida luego de la insercion o del pusheo el cual nos da como resultado Hello World
    
    let s3: &str = &s2[6..];//FIX importante aca tomamos como referencia una porcion del string creado desde la posicion 6.. hasta el final como es una porcion utilizamos la referencia &str 
    println!("s3: {s3}");//OPTIMIZE impirmimos el fragmento 
    //FIX podemos usar un String::from() y hacerlo inmutable solo sacandole el mut
}
//TODO funciones como llamarlas entre si
fn fizz_buzz(){
    print_fizzbuzz_to(20);//TODO llamamos la la funcion encargada de imprimir

    //FIX importante aclarar que cada cuncion nos dice que devuelva , si es bool, string , number etc 

    //OPTIMIZE esta funcion resive el numero a comprobar y el divisor con el cual hay que comprobar nos retirna si es o no divisible por dicho nuemro
    fn is_divisible(n: u32, divisor: u32) -> bool{
        if divisor == 0 {
            return false
        }
        n % divisor == 0
    }
    //OPTIMIZE esta funcion resive un numero y guarda en una variable un strin fizz o buzz hacemos uso de la funcion que comprueba si es divisible o no
    fn fizzbuzz(n: u32) -> String{
        let fizz = if is_divisible(n, 3) {"fizz"} else {""};//TODO usamos la funcion que nos dice si es divisible o no si es divisible por 3 nos devuelve fizz sino nada
        let buzz = if is_divisible(n, 5) {"buzz"} else {""};//TODO luego preguntamos si es divisible en 5 si es asi devolvemos buzz 
        //OPTIMIZE aca comprobamos con el metodo is_empty si estan cargadas ambas bariables si esto susede significa que es divisible en ambos nuemros
        if fizz.is_empty() && buzz.is_empty() {//TODO preguntamos si ambos estan con valores entonces returnamos un format! con el valor 
            return format!("{n}");
        }
        format!("{fizz}{buzz}")//FIX aprender como se usa format!
    }
    //TODO funcion destinada a recoreer todos los nuemros desde el 1 hasta el numero que nsootros le pasamos como argumento
    fn print_fizzbuzz_to(n: u32){
        for i in 1..=n {//FIX recorremos desde el 1 hasta el nuemro que nos apsaron caundo esto suseda la funcion para 
            println!("{}", fizzbuzz(i));//OPTIMIZE llamamos a la funcion que nos dise si es fizz o buzz el numero pasando cada nuemro hasta el seleccionado e imprimimos el resultado que debuelve que es un string
        }
    }
}

fn main() {
    small();
    array();
    tuplas();
    referencia();
    prestamos_referencia();
    string_vs_str();
    fizz_buzz();
}
