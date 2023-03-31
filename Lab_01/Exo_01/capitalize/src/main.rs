use std::env::args;

fn main() {


    let args: Vec<String>= args().skip(1).collect();

    for s in args.iter(){

        println!("{}", capitalize(s));
    }

}



fn capitalize(s: &str) -> String {

    let mut first_letter_is_comming: bool=true;
    let mut copy: String=Default::default();


    for c in s.chars(){
        
        if first_letter_is_comming && c!=' ' {

            copy += &c.to_uppercase().to_string();
            first_letter_is_comming = false;

        }else if c == ' ' {

            first_letter_is_comming = true;
            copy += &c.to_string();

        }else{

            copy += &c.to_string();

        }
    }

    return copy;

}


mod my_tests {

    use super::capitalize;

#[test]
fn string_with_more_than_one_word(){
    assert_eq!("Hello World",capitalize("hello world"));
}

#[test]
fn string_with_no_space_word(){
    assert_eq!("Bonjour", capitalize("bonjour"));
}

#[test]
fn string_with_word_containing_special_char(){
    assert_eq!("È Gia Fatto", capitalize("è gia fatto"));
}

#[test]
fn empty_string(){
    assert_eq!("", capitalize(""));
}

#[test]
fn only_space_string(){
    assert_eq!("          ", capitalize("          "));
}

}