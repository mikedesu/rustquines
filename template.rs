const COL_LEN : i32 = 200;

fn handle_spaces(num_spaces:i32){
    for _ in 0..num_spaces{
        print!(" ");
    }
}

fn handle_newline_i_update(i:&mut i32,count:&mut i32){
    if *i>=COL_LEN{
        *i=0;
        *count+=1;
    }
}

fn count_newlines(index:usize,v:&[u32])->i32{
    let mut count=0;
    let mut i=0;
    let mut is_inside_quotes=false;
    for k in index..v.len(){
        let c = v[k];
        // any newlines present in the source will be skipped
        // handle the @ symbol
        if c==64{
            let mut last_num = 0;
            for j in 0..v.len(){
                if v[j]>=100{
                    i+=3;
                } else if v[j]>=10{
                    i+=2;
                } else{
                    i+=1;
                }
                handle_newline_i_update(&mut i,&mut count);
                if j<v.len()-1{
                    i+=1;
                    handle_newline_i_update(&mut i,&mut count);
                }
                last_num=v[j];
            }
            if last_num>=100{
                i+=3;
            } else if last_num>=10{
                i+=2;
            } else{
                i+=1;
            }
            handle_newline_i_update(&mut i,&mut count);
            i+=1;
            handle_newline_i_update(&mut i,&mut count);
        }
        else{
            i+=1;
            if c==32||c==40||c==41||c==59||c==123||c==125{
                if !is_inside_quotes{
                    handle_newline_i_update(&mut i,&mut count);
                }
            }
            else if c==34{
                is_inside_quotes=!is_inside_quotes;
            }
        }
    }
    return count;
}


fn handle_array_spacing(i:&mut i32,current_row:&mut i32,current_column:&mut i32,max_columns:i32,newline_ct:i32){
    const MAX_ROWS:i32=40;
    //if (*current_row < MAX_ROWS && *i>=COL_LEN)||(newline_ct<=MAX_ROWS&&*i>=COL_LEN){
    if *current_column==1&&*i>=COL_LEN/4{
        println!();
        *current_row+=1;
        *i=0;
        *current_column=0;
    }
    else if *current_row<MAX_ROWS&&*i>=COL_LEN{
        println!();
        *current_row+=1;
        *i=0;
        *current_column=0;
    }
    else if *i>=COL_LEN{
        println!();
        *current_row+=1;
        *i=0;
        *current_column=0;
    }
    //else if *current_row >= MAX_ROWS*2 && *i>=COL_LEN{
    //    println!();
    //    *current_row+=1;
    //    *i=0;
    //    *current_column=0;
    //}
    else if *current_column==0&&*i>=COL_LEN/4{
        handle_spaces(COL_LEN/2-(*i-(COL_LEN/4)));
        *i=0;
        *current_column+=1;
    }
    else if *current_column==1&&*i>=COL_LEN/4{
        println!();
        *current_row+=1;
        *current_column=0;
        *i=0;
    }
    else if *i>=COL_LEN&&*current_row>=MAX_ROWS{
        println!();
        *current_row+=1;
        *current_column=0;
        *i=0;
    }

}


fn print_array(i:&mut i32,current_row:&mut i32,v:&[u32],current_column:&mut i32,max_columns:i32,newline_ct:i32) {
    print!("[");
    *i+=1;
    handle_array_spacing(i,current_row,current_column,max_columns,newline_ct);       
    for j in 0..v.len(){
        print!("{}",v[j]);
        if v[j]>=100{
            *i+=3;
        } else if v[j]>=10{
            *i+=2;
        } else{
            *i+=1;
        }
        handle_array_spacing(i,current_row,current_column,max_columns,newline_ct);       
        if j<v.len()-1{
            print!(",");
            *i+=1;
            handle_array_spacing(i,current_row,current_column,max_columns,newline_ct);       
        }
    }
    print!("]");
    *i+=1;
    handle_array_spacing(i,current_row,current_column,max_columns,newline_ct);       
    print!(";");
    *i+=1;
    handle_array_spacing(i,current_row,current_column,max_columns,newline_ct);       
}



fn handle_special_char_spacing(i:&mut i32,current_row:&mut i32,current_column:&mut i32,max_columns:i32,is_inside_quotes:bool,newline_ct:i32){
    const MAX_ROWS:i32=20;
    if *current_row<MAX_ROWS&&*i>=COL_LEN{
        if !is_inside_quotes{
            println!();
            *current_row+=1;
            *i=0;
        }
    }
    else if *current_row>=MAX_ROWS&&*i>=COL_LEN/4&&newline_ct>=MAX_ROWS{
        if !is_inside_quotes{
            if *current_column==0{
                handle_spaces(COL_LEN/2-(*i-(COL_LEN/4)));
                *i=0;
                *current_column+=1;
            }
            else {
                println!();
                *current_row+=1;
                *current_column=0;
                *i=0;
            }
        }
    }
    else if *current_column==1&&*i>=COL_LEN/4{
        if !is_inside_quotes{
            println!();
            *current_row+=1;
            *current_column=0;
            *i=0;
        }
    }
    else if *i>=COL_LEN {
        if !is_inside_quotes{
            println!();
            *current_row+=1;
            *i=0;
        }
    }
}


fn handle_special_char(c:u32,i:&mut i32,current_row:&mut i32,is_inside_quotes:&mut bool,current_column:&mut i32,max_columns:i32,newline_ct:i32){
    print!("{}",char::from_u32(c).unwrap());
    *i+=1;
    // space
    // open paren
    // close paren
    // asterisk
    // comma
    // colon
    // semicolon
    // open curly brace
    // close curly brace
    // open bracket
    // close bracket
    //if c==32||c==40||c==41||c==59||c==123||c==125{ 
    if c==32||c==40||c==41||c==42||c==44||c==58||c==59||c==91||c==93||c==123||c==125{ 
        handle_special_char_spacing(i,current_row,current_column,max_columns,*is_inside_quotes,newline_ct);
    }
    else if c==34{
        *is_inside_quotes=!*is_inside_quotes;
    }
}


fn main(){
    let v=@
    let mut i:i32=0;
    let mut current_row:i32=0;
    let mut is_inside_quotes=false;
    let mut max_columns=1;
    let mut current_column=0;
    for k in 0..v.len(){
        let c=v[k];
        let newline_ct=count_newlines(k,&v);
        // any newlines present in the source will be skipped
        if c==10{
            continue;
        }
        // handle the @ symbol
        else if c==64{
            print_array(&mut i,&mut current_row,&v,&mut current_column,max_columns,newline_ct);
        }
        else{
            handle_special_char(c,&mut i,&mut current_row,&mut is_inside_quotes,&mut current_column,max_columns,newline_ct);
        }
    }
}

