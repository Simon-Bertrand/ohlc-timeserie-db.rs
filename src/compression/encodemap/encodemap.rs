use std::{io::{Error, ErrorKind}};

pub struct EncodeMap {}
impl EncodeMap {
    pub fn char_to_bytes(character : &char) -> Result<u8, Error> {
        match character {
            '-' => Ok(0b00000000),
            '0' => Ok(0b00000001),
            '1' => Ok(0b00000010),
            '2' => Ok(0b00000011),
            '3' => Ok(0b00000100),
            '4' => Ok(0b00000101),
            '5' => Ok(0b00000110),
            '6' => Ok(0b00000111),
            '7' => Ok(0b00001000),
            '8' => Ok(0b00001001),
            '9' => Ok(0b00001010),
            '.' => Ok(0b00001011),
            ';' => Ok(0b00001100),
            '*' => Ok(0b00001101),
            _ => Err(Error::new(ErrorKind::NotFound, "Invalid conversion Char to Bytes"))
        }
     }


    pub fn bytes_to_char(byte : &u8) -> Result<char, Error> {
        match byte {
                0b00000000 => Ok('-'),
                0b00000001 => Ok('0'),
                0b00000010 => Ok('1'),
                0b00000011 => Ok('2'),
                0b00000100 => Ok('3'),
                0b00000101 => Ok('4'),
                0b00000110 => Ok('5'),
                0b00000111 => Ok('6'),
                0b00001000 => Ok('7'),
                0b00001001 => Ok('8'),
                0b00001010 => Ok('9'),
                0b00001011 => Ok('.'),
                0b00001100 => Ok(';'),
                0b00001101 => Ok('*'),
            _ => Err(Error::new(ErrorKind::NotFound, "Invalid conversion Char to Bytes"))
        }
    }

}


     

  

   

    // pub fn decode(bits : &[u8]) -> Vec<TsPointDataRaw> {

    //     let mut all_lines : Vec<Vec<Vec<u8>>> = Vec::new();
    //     let mut temp_values : Vec<u8> = Vec::new();
    //     let mut computed_data : Vec<Vec<u8>>= Vec::new();

    //     let mut iterator = bits.iter();

    //     let start_byte = EncodeMap::char_to_bytes(&'*').unwrap();
    //     let sep_byte = EncodeMap::char_to_bytes(&';').unwrap();

    //     if '*' == EncodeMap::bytes_to_char(iterator.next().unwrap()).unwrap() {
    //         while let Some(chars_bit_slice) = iterator.next() {
    //             if chars_bit_slice == &EncodeMap::char_to_bytes(&'*').unwrap() {
    //                 all_lines.push(computed_data);
    //                 computed_data= vec![];
    //             }
    //             else { 
    //                 if chars_bit_slice == &EncodeMap::char_to_bytes(&';').unwrap() {
    //                     computed_data.push(temp_values);
    //                     temp_values= Vec::new();
    //                 } 
    //                 else {
    //                     temp_values.push(*chars_bit_slice)
    //                 }
    //             }  
    //         }
    //         all_lines.push(computed_data);

    //     }

    //     all_lines.iter().map(|x| TsPointDataRaw{
    //         o:&x[0].to_owned(),
    //         h:&x[1].to_owned(),
    //         c:&x[2].to_owned(),
    //         l:&x[3].to_owned()
    //     } ).collect::<Vec<TsPointDataRaw>>()
        


    
    // }


   



    







