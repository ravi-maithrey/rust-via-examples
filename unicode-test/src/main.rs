fn language_demo() {
    println!("Hello, World!");
    let german = "Grüß Gott!";
    let japanese = "ハロー・ワールド";

    let languages = [german, japanese];
    for language in languages.iter() {
        println!("{}", &language);
    }
}

fn main() {
    language_demo();
}
