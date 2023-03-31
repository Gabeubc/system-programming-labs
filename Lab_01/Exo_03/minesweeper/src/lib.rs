pub fn annotate(minefield: &[&str]) -> Vec<String> {
    //unimplemented!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{minefield:#?}\n");

    minesweeper(minefield)
}


fn minesweeper(minefield: &[&str]) -> Vec<String>{

 let mut result : Vec<String>=Default::default();
 let mut result_row: String= Default::default(); 
 let row_len : usize = minefield.len();
 let mut str: String = Default::default();

    if row_len == 0{

        return result;

    }

 let col_len: usize = minefield.get(0).unwrap().len();


    //only vertical
    if col_len == 1 {

        let mut count: u32 = 0;
        let mut c: String= Default::default();

        for i in 0..row_len{

            result_row += &match minefield.get(i).unwrap(){

                &" " => {
                    
        //up
        if i > 0{
            
        if minefield.get(i - 1).unwrap().eq(&"*")  {

            count += 1;

        }

        }
        
        //down
        if i < row_len -1{
            
        if minefield.get(i + 1).unwrap().eq(&"*") {

            count += 1;

        }

        } 

        if count == 0 {

             c = " ".to_string();

        }else{

            c = count.to_string();

        }

        c


                },


                &"*" => {
                    "*".to_string()
                },
                _ => {
                    "".to_string()
                }

            };
            result.push(result_row.clone());
            result_row.clear();
            count = 0;

        }

        return result;

    }


    //only horizontal
    if row_len == 1 {

        let mut count = 0;
        let mut tmp: String= "".to_string();

        for i in 0..col_len{

            result_row += match minefield.get(0).unwrap().chars().nth(i).unwrap(){

                ' ' => {
                    
        //left
        if i > 0{
            
        if minefield.get(0).unwrap().chars().nth(i-1).unwrap() == '*' {

            count += 1;

        }

        }
        
        //rigth
        if i < col_len - 1{
            
        if minefield.get(0).unwrap().chars().nth(i+1).unwrap() == '*' {

            count += 1;

        }

        }



        if count == 0{

            tmp = " ".to_string();

        }else{

            tmp = count.to_string();

        }

        &tmp

                },

                '*' => {
                    "*"
                },

                _ => {
                    ""
                }

            };
            count = 0;

        }

        result.push(result_row.clone());
        
        return result;

    }


    //horizontal x vertical
    for i in 0..row_len {


        for j in 0..col_len{

             str = count_mine_for_pos(i, j, minefield, row_len, col_len);

            result_row += match minefield.get(i).unwrap().chars().nth(j).unwrap() {

                ' ' => {&str},
                '*' =>  {"*"},
                _ => todo!()
                
            };

        }

        result.push(result_row.clone());
        result_row.clear();
    }

 result   

}

fn count_mine_for_pos (row: usize, col: usize, minefield: &[&str], row_len: usize, col_len: usize) -> String {

    let mut count: u32 = Default::default();

    if row == 0 && col == 0 {

        
        //rigth
        if minefield.get(row).unwrap().chars().nth(col + 1).unwrap() == '*'{

            count += 1;

        }
        
        //down
        if minefield.get(row + 1).unwrap().chars().nth(col).unwrap() == '*'{

            count += 1;

        }

        //down right
        if minefield.get(row + 1).unwrap().chars().nth(col + 1).unwrap() == '*'{

            count += 1;

        }


    } else if row == 0 && col == col_len - 1{

        //left
        if minefield.get(row).unwrap().chars().nth(col - 1).unwrap() == '*'{

            count += 1;

        }
        
        //down
        if minefield.get(row + 1).unwrap().chars().nth(col).unwrap() == '*'{

            count += 1;

        }

        //down left
        if minefield.get(row + 1).unwrap().chars().nth(col - 1).unwrap() == '*'{

            count += 1;

        }

    } else if row == row_len - 1 && col == 0{

        //rigth
        if minefield.get(row).unwrap().chars().nth(col + 1).unwrap() == '*'{

            count += 1;

        }
        
        //up
        if minefield.get(row - 1).unwrap().chars().nth(col).unwrap() == '*'{

            count += 1;

        }

        //up rigth
        if minefield.get(row - 1).unwrap().chars().nth(col + 1).unwrap() == '*'{

            count += 1;

        }

    } else if row == row_len -1 && col == col_len - 1{

        //left
        if minefield.get(row).unwrap().chars().nth(col - 1).unwrap() == '*'{

            count += 1;

        }
        
        //up
        if minefield.get(row - 1).unwrap().chars().nth(col).unwrap() == '*'{

            count += 1;

        }

        //up left
        if minefield.get(row - 1).unwrap().chars().nth(col - 1).unwrap() == '*'{

            count += 1;

        }

    } else if row == 0 && ( 0 < col && col < col_len - 1){

        //rigth
        if minefield.get(row).unwrap().chars().nth(col + 1).unwrap() == '*'{

            count += 1;

        }

        //left
        if minefield.get(row).unwrap().chars().nth(col - 1).unwrap() == '*'{

            count += 1;

        }
        
        //down
        if minefield.get(row + 1).unwrap().chars().nth(col).unwrap() == '*'{

            count += 1;

        }

        //down right
        if minefield.get(row + 1).unwrap().chars().nth(col + 1).unwrap() == '*'{

            count += 1;

        }

        //down left
        if minefield.get(row + 1).unwrap().chars().nth(col - 1).unwrap() == '*'{

            count += 1;

        }
        
    }else if col == 0 && ( 0 < row && row < row_len - 1  ){

        
        //rigth
        if minefield.get(row).unwrap().chars().nth(col + 1).unwrap() == '*'{

            count += 1;

        }
        
        //up
        if minefield.get(row - 1).unwrap().chars().nth(col).unwrap() == '*'{

            count += 1;

        }
        
        //down
        if minefield.get(row + 1).unwrap().chars().nth(col).unwrap() == '*'{

            count += 1;

        }



        //up rigth
        if minefield.get(row - 1).unwrap().chars().nth(col + 1).unwrap() == '*'{

            count += 1;

        }


        //down right
        if minefield.get(row + 1).unwrap().chars().nth(col + 1).unwrap() == '*'{

            count += 1;

        }

    } else if col == (col_len - 1) && ( 0 < row && row < row_len - 1  ){
     
        //left
        if minefield.get(row).unwrap().chars().nth(col - 1).unwrap() == '*'{

            count += 1;

        }
        
        //up
        if minefield.get(row - 1).unwrap().chars().nth(col).unwrap() == '*'{

            count += 1;

        }
        
        //down
        if minefield.get(row + 1).unwrap().chars().nth(col).unwrap() == '*'{

            count += 1;

        }

        //up left
        if minefield.get(row - 1).unwrap().chars().nth(col - 1).unwrap() == '*'{

            count += 1;

        }

        //down left
        if minefield.get(row + 1).unwrap().chars().nth(col - 1).unwrap() == '*'{

            count += 1;

        }

    } else if row == (row_len - 1) && ( 0 < col && col < col_len - 1) {
        
        //rigth
        if minefield.get(row).unwrap().chars().nth(col + 1).unwrap() == '*'{

            count += 1;

        }

        //left
        if minefield.get(row).unwrap().chars().nth(col - 1).unwrap() == '*'{

            count += 1;

        }
        
        //up
        if minefield.get(row - 1).unwrap().chars().nth(col).unwrap() == '*'{

            count += 1;

        }

        //up rigth
        if minefield.get(row - 1).unwrap().chars().nth(col + 1).unwrap() == '*'{

            count += 1;

        }

        //up left
        if minefield.get(row - 1).unwrap().chars().nth(col - 1).unwrap() == '*'{

            count += 1;

        }

    } else {
        
        //rigth
        if minefield.get(row).unwrap().chars().nth(col + 1).unwrap() == '*'{

            count += 1;

        }

        //left
        if minefield.get(row).unwrap().chars().nth(col - 1).unwrap() == '*'{

            count += 1;

        }
        
        //up
        if minefield.get(row - 1).unwrap().chars().nth(col).unwrap() == '*'{

            count += 1;

        }
        
        //down
        if minefield.get(row + 1).unwrap().chars().nth(col).unwrap() == '*'{

            count += 1;

        }



        //up rigth
        if minefield.get(row - 1).unwrap().chars().nth(col + 1).unwrap() == '*'{

            count += 1;

        }

        //up left
        if minefield.get(row - 1).unwrap().chars().nth(col - 1).unwrap() == '*'{

            count += 1;

        }

        //down right
        if minefield.get(row + 1).unwrap().chars().nth(col + 1).unwrap() == '*'{

            count += 1;

        }

        //down left
        if minefield.get(row + 1).unwrap().chars().nth(col - 1).unwrap() == '*'{

            count += 1;

        }
    }

    if count == 0{
        return ' '.to_string();
    }
    count.to_string()

}
