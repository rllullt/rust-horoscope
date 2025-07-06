// random numbers
use rand::Rng;
use std::io;

fn main() {
    // We create arrays with immutable strings
    let first = [
        "Hoy es un día perfecto para nuevos comienzos.",
        "¡El mejor día para tomar una decisión valiente!",
        "Ten cuidado, las estrellas de hoy pueden afectar tu salud financiera.",
        "El mejor momento para comenzar una nueva relación, o lidiar con una antigua.",
        "Un día fructífero para hacer frente a los quehaceres acumulados.",
    ];

    let second = [
        "Pero recuerda: incluso en este caso, no debes olvidar:",
        "Si vas fuera de la ciudad, piensa en ello de antemano:",
        "Los que hoy se esfuerzan por hacer muchas cosas deben recordar:",
        "Si estás en declive, presta atención a lo siguiente:",
        "Recuerda que los pensamientos son materiales, lo que significa que tienes que pensar constantemente durante el día en:",
    ];

    let second_add = [
        "las relaciones con amigos y seres queridos.",
        "el trabajo y los asuntos de negocios que pueden obstaculizar tan inapropiadamente los planes.",
        "a TI mismo y a tu salud, de lo contrario, por la noche, es posible una ruptura completa.",
        "los problemas cotidianos, especialmente los que no terminaste ayer.",
        "descansar para no convertirte en un caballo acorralado a final del mes.",
    ];

    let third = [
        "Las malas lenguas pueden decirte lo contrario, pero hoy no necesitas escucharlas.",
        "Tienes que saber que el éxito sólo favorece a los perseverantes, así que dedica este día a nutrir el espíritu.",
        "Incluso si no puedes reducir el impacto de mercurio retrógrado, al menos lleva el asunto hasta el final.",
        "No tengas miedo de los encuentros solitarios, hoy es cuando más significan.",
        "Si te encuentras con un extraño en el camino, interactúa, así, esta reunión te prometerá problemas agradables.",
    ];

    // We show the user the zodiac signs
    println!("1 — Aries");
    println!("2 — Tauro");
    println!("3 — Géminis");
    println!("4 — Cáncer");
    println!("5 — Leo");
    println!("6 — Virgo");
    println!("7 — Libra");
    println!("8 — Escorpio");
    println!("9 — Sagitario");
    println!("10 — Capricornio");
    println!("11 — Acuario");
    println!("12 — Piscis");
    
    // Ask the user, what is their zodiac sign:
    println!("Enter the number of your zodiac sign:");

    // Read the input for the user
    let mut input = String::new();
    // If reading fails, it shows an error message
    io::stdin().read_line(&mut input).expect("Couldn’t read line.");
    // In other langauges like Python, all variables are links to objects in memory.
    // Rust creates complete copies of elementos.
    // So, to not overload the memory, it is necessary to indicate we are referencing an object already created with the ‘&’ symbol.

    // We create a variable to store an usize number
    // We delete additional spaces and newlines from the string entered by th user
    let zodiac: usize = match input.trim().parse() {
        // If the line is convertible in a number, convert it
        Ok(num) => num,
        // If not, throw an error
        Err(_) => {
            println!("Error! Enter a number between 1 and 12.");
            // and stop the program
            return;
        }
    };

    // If the entered number is in the range from 1 to 12, we generate an horoscope
    // As in journal horoscopes, we really don’t care what is the user’s zodiac sign.
    if zodiac > 0 && zodiac < 13 {
        // Create random number generator for selecting random phrases from previous arrays
        let mut rng = rand::thread_rng();
        // Create four variables and assign one string to each
        let first_choice = rng.gen_range(0..first.len());
        let second_choice = rng.gen_range(0..second.len());
        let second_add_choice = rng.gen_range(0..second_add.len());
        let third_choice = rng.gen_range(0..third.len());
        // Show the four selected phrases in order:
        println!(
            "{} {} {} {}",
            first[first_choice], second[second_choice], second_add[second_add_choice], third[third_choice]
        );
    } else { // error if the inserted number is not in range
        println!("The number is not correct. Run the program again.");
    }
}
