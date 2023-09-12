//Convert strings to pig latin. 
//The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
//Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
// Keep in mind the details about UTF-8 encoding!

fn main() {
    let mut my_string = String::from("apple");
    // Using indexing to get the first character (byte)
    
    if let Some(first_char) = my_string.chars().next() {
        println!("The first character is: {}", first_char);
    } else {
        println!("The string is empty.");
    }

    if is_vowel(first_char) {
        my_string.push_str(first_char);
        println!("{}",my_string);
    }
}


fn is_vowel(c: char)-> bool {
    let lowercase_c=c.to_lowercase().next().unwrap();
    match lowercase_c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false ,
    }
}