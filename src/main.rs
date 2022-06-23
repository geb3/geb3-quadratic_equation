use std::io;

fn main(){
    // ax^2 + bx + c = 0
    // D = b^2 - 4*(a*c)
    loop {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();
    println!("Запуск программы..\n");
    println!("РЕШИТЬ КВАРДРАТНОЕ УРАВНЕНИЕ: ax^2 + bx + c = 0");

    println!("Введите a: ");
    io::stdin()
        .read_line(&mut a)
        .expect("Неизвестная ошибка!");

    println!("Введите b: ");
    io::stdin()
        .read_line(&mut b)
        .expect("Неизвестная ошибка!");

    println!("Введите c: ");
    io::stdin()
        .read_line(&mut c)
        .expect("Неизвестная ошибка!");
    
    // - Необходимо обработать ситуацию, когда пользователь вводит не только число
    let a: f64 = match a.trim().parse() {
        Ok(a) => a,
        Err(_) => {
            println!("\nОшибка!\nЗавершение программы..\n");
            println!("----------------------------------------------------------------");
            continue
        },
    };
    let b: f64 = match b.trim().parse() {
        Ok(b) => b,
        Err(_) => {
            println!("\nОшибка!\nЗавершение программы..\n");
            println!("----------------------------------------------------------------");
            continue
        },
    };
    let c: f64 = match c.trim().parse() {
        Ok(c) => c,
        Err(_) => {
            println!("\nОшибка!\nЗавершение программы..\n");
            println!("----------------------------------------------------------------");
            continue
        },
    };
    // - trim удаляет пробелы в строке, parse переводит в тип float из str
    
    print!("\na = {}, b = {}, c = {}\n", a, b, c);
    let d: f64 = (b * b) - 4.0 * (a * c);
    
    if d > 0.0 {
        let x1 = ((-b) + d.sqrt()) / (2.0 * a);
        let x2 = ((-b) - d.sqrt()) / (2.0 * a);

        println!("\nРешено!\nD = {}\nКорень 1 = {}, Корень 2 = {}\n", d, x1, x2);
        println!("Завершение программы..\n");
        println!("----------------------------------------------------------------");
        continue;
    }
    if d == 0.0 {
        let x = (-b) / (2.0 * a);

        println!("\nРешено!\nD = 0\nЕсть 1 корень = {}\n", x);
        println!("Завершение программы..\n");
        println!("----------------------------------------------------------------");
        continue;
    }
    if d < 0.0 {
        println!("\nКорней нет!\nD < 0\nD = {}\n", d);
        println!("Завершение программы..\n");
        println!("----------------------------------------------------------------");
        continue;
    }
    }
}