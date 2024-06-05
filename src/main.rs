// Конвертация температур между значениями по Фаренгейту к Цельсию.
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    // (F − 32) / 1.8
    (fahrenheit - 32.0) / 1.8
}

fn read_fahrenheit_temp() -> f64 {
    loop {
        println!("Please input temperature in fahrenheit.");

        let mut temperature = String::new();

        std::io::stdin().read_line(&mut temperature).expect("Failed to read line");
        let temperature: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You temperature in fahrenheit: {}", temperature);
        return temperature;
    }
}
// ------------------------------------------------------------------------------------------------

// Генерирование n-го числа Фибоначчи.
fn fib(numb: u32) -> u32 {
    if numb <= 1 {
        return numb
    }
    else {
        return fib(numb - 1) + fib(numb - 2)
    }

    //numb ? 1 : fib(numb - 1) + fib(numb - 2)
}

fn read_fib_n() -> u32 {
    loop {
        println!("Please input position of a number in the Fibonacci sequence.");

        let mut n = String::new();

        std::io::stdin().read_line(&mut n).expect("Failed to read line");
        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You position of a number in the Fibonacci sequence: {}", n);
        return n;
    }
}
// ------------------------------------------------------------------------------------------------

// Распечатайте текст рождественской песни "Двенадцать дней Рождества", воспользовавшись повторами в песне.
fn twelve_days_of_chris(){
    let a = ["And a Partridge in a Pear Tree", "Two Turtle Doves", "Three French Hens", "Four Calling Birds", "Five Golden Rings", "Six Geese a-Laying", 
    "Seven Swans a-Swimming", "Eight Maids a-Milking", "Nine Ladies Dancing", "Ten Lords a-Leaping", "Eleven Pipers Piping", "Twelve Drummers Drumming"];
    let b = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelth"];
    
    println!("On the first day of Christmas
My true love gave to me
a Partridge in a Pear Tree
");
    
    for i in 1..12 {
        //println!("{}", i);
        if (i % 4) == 0 {
            println!("Whats a partridge?
And whats a pear tree?
I don't know so please don't ask me
But I can bet those are terrible gifts to get.
");
        }
        println!("On the {} day of Christmas
My true love gave to me", b[i]);

        let mut j = 0;
        while j <= i {
            println!("{}", a[i-j]);
            j += 1;
        }
        println!("");
    }
}
// ------------------------------------------------------------------------------------------------

fn main() {

    // Конвертация температур между значениями по Фаренгейту к Цельсию.
    let temperature: f64 = read_fahrenheit_temp();
    let temperature: f64 = fahrenheit_to_celsius(temperature);
    println!("You temperature in celsius: {:.2}", temperature);

    // Генерирование n-го числа Фибоначчи.
    //let i = read_fib_n(); 
    //let numb = fib(i);
    //println!("The {i} position of a number in the Fibonacci sequence: {numb}");

    // Распечатайте текст рождественской песни "Двенадцать дней Рождества", воспользовавшись повторами в песне.

    //twelve_days_of_chris();
}
