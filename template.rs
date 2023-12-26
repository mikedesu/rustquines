const COL_LEN : i32 = 100;

//fn handle_spaces(num_spaces : i32) {
//    for _ in 0..num_spaces{
//        print!(" ");
//    }
//}

fn count_newlines(v : &[u32]) -> i32{
    let mut count=0;
    let mut i=0;
    let mut is_inside_quotes=false;
    for k in 0..v.len(){
        let c = v[k];
        // any newlines present in the source will be skipped
        // handle the @ symbol
        if c==64{
            for j in 0..v.len(){
                if v[j]>100{
                    i+=3;
                } else if v[j]>10{
                    i+=2;
                } else{
                    i+=1;
                }
                if i>=COL_LEN{
                    i=0;
                    count+=1;
                }
                if j<v.len()-1{
                    i+=1;
                    if i>=COL_LEN{
                        i=0;
                        count+=1;
                    }
                }
            }
            i+=1;
            if i>=COL_LEN{
                count+=1;
                i=0;
            }
            i+=1;
            if i>=COL_LEN{
                count+=1;
                i=0;
            }
        }
        else{
            i+=1;
            if(c==32||c==40||c==41||c==59||c==123||c==125) && i>=COL_LEN{
                if !is_inside_quotes{
                    i=0;
                    count+=1;
                }
            }
            else if c==34{
                is_inside_quotes=!is_inside_quotes;
            }
        }
    }
    return count;
}


fn print_array(i : &mut i32, current_row : &mut i32, v : &[u32]) {
    print!("[");
    for j in 0..v.len(){
        print!("{}",v[j]);
        if v[j]>100{
            *i+=3;
        } else if v[j]>10{
            *i+=2;
        } else{
            *i+=1;
        }
        if *i>=COL_LEN{
            println!();
            *i=0;
            *current_row+=1;
        }
        if j<v.len()-1{
            print!(",");
            *i+=1;
            if *i>=COL_LEN{
                println!();
                *i=0;
                *current_row+=1;
            }
        }
    }
    print!("]");
    *i+=1;
    if *i>=COL_LEN{
        println!();
        *i=0;
        *current_row+=1;
    }
    print!(";");
    *i+=1;
    if *i>=COL_LEN{
        println!();
        *i=0;
        *current_row+=1;
    }
}


fn main() {
    let v=@
    let mut i : i32 = 0;
    // set col_len to a random number from 10 to 40
    let newline_ct=count_newlines(&v);
    let mut current_row : i32 = 0;
    let mut is_inside_quotes=false;
    for k in 0..v.len(){
        let c = v[k];
        // any newlines present in the source will be skipped
        if c==10{
            continue;
        }
        // handle the @ symbol
        else if c==64{
            print_array(&mut i,&mut current_row,&v);
        }
        else{
            print!("{}",char::from_u32(c).unwrap());
            i+=1;
            if(c==32||c==40||c==41||c==59||c==123||c==125) && i>=COL_LEN{
                if !is_inside_quotes{
                    println!();
                    current_row+=1;
                    i=0;
                }
            }
            else if c==34{
                is_inside_quotes=!is_inside_quotes;
            }
        }
    }
}

