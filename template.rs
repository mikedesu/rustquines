

fn handle_spaces(num_spaces : i32) {
    for _ in 0..num_spaces{
        print!(" ");
    }
}


fn main() {
    let v=@
    let mut i=0;
    // set col_len to a random number from 10 to 40
    let col_len=rand::random::<i32>()%30+10;
    //let col_len=20;
    //let space_len=20;
    let space_len=rand::random::<i32>()%10+10;
    //let col_count=4;
    let col_count=rand::random::<i32>()%7+1;
    let mut col_num=0;
    let mut is_inside_quotes=false;
    //for c in v{
    for k in 0..v.len(){
        let c = v[k];
        // test comment
        // any newlines present in the source will be skipped
        if c==10{
            continue;
        }
        // handle the @ symbol
        else if c==64{
            print!("[");
            for j in 0..v.len(){
                print!("{}",v[j]);
                if v[j]>100{
                    i+=3;
                } else if v[j]>10{
                    i+=2;
                } else{
                    i+=1;
                }
                if i>col_len{
                    if col_num<=col_count-1 {
                        //let num_spaces = 20 - (i - col_len);
                        //for _ in 0..num_spaces{
                        //    print!(" ");
                        //}
                        handle_spaces(space_len - (i - col_len));
                        col_num+=1;
                    }
                    else {
                        println!();
                        col_num=0;
                    }
                    i=0;
                }
                if j<v.len()-1{
                    print!(",");
                    i+=1;
                    if i>=col_len{
                        if col_num<=col_count-1 {
                            //let num_spaces = 20 - (i - col_len);
                            //for _ in 0..num_spaces{
                            //    print!(" ");
                            //}
                            handle_spaces(space_len - (i - col_len));
                            col_num+=1;
                        }
                        else {
                            println!();
                            col_num=0;
                        }
                        i=0;
                    }
                }
            }
            print!("];");
        }
        else{
            print!("{}",char::from_u32(c).unwrap());
            i+=1;

            // do lookahead
            // check for open parenthesis or open curly
            //if k+1<v.len() && (v[k+1]==40||v[k+1]==123) && i >= col_len {
            //    if col_num==0{
            //        handle_spaces(20 - (i - col_len));
            //        col_num+=1;
            //    } else {
            //        println!();
            //        col_num=0;
            //    }
            //    i = 0;
            //}
            // space ( ) ; { } 
            //else if(c==32||c==40||c==41||c==59||c==123||c==125) && i>=col_len{
            if(c==32||c==40||c==41||c==59||c==123||c==125) && i>=col_len{
                if !is_inside_quotes{
                    if col_num<=col_count-1{
                        handle_spaces(space_len - (i - col_len));
                        col_num+=1;
                    }
                    else {
                        println!();
                        col_num=0;
                    }
                    i=0;
                }
            }
            else if c==34{
                is_inside_quotes=!is_inside_quotes;
            }
        }
    }
}
