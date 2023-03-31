

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    //unimplemented!("Is the Luhn checksum for {code} valid?");

    luhn_checksum(code)
}


fn luhn_checksum(code : &str) -> bool {


    let mut c: char;

    let mut sum: u32 = 0;

    let mut cursor: u32 = 0;

    let mut doubled: u32 = 0;

    let len = code.len();


    if !code.is_ascii() {
        return false;
    }

    if code.trim().len() <= 1 {
        return false;
   }

    
    for i in 1..=len {

        c = code.chars().nth(len - i).unwrap();

        if c != ' '{

            if !c.is_digit(10) {

                return false;

            }

            if cursor == 1 {

                cursor = 0;
                doubled = c.to_digit(10).unwrap() * 2;

                if doubled > 9 {
                     
                     doubled -= 9;

                }
                
                sum += doubled;

            } else{

                sum += c.to_digit(10).unwrap();
                cursor += 1;

            }

        }

    }

    return sum%10 == 0;

}