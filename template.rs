fn main() {
    let v=@
    let mut i=0;
    let x=20;
    let mut is_inside_quotes=false;
    for c in v{
        if c==10{
            continue;
        }
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
                if i>x{
                    print!("\n");
                    i=0;
                }
                if j<v.len()-1{
                    print!(",");
                    i+=1;
                    if i>x{
                        print!("\n");
                        i=0;
                    }
                }
            }
            print!("];");
        }
        else{
            print!("{}",char::from_u32(c).unwrap());
            i+=1;
            if(c==59||c==123||c==125||c==32)&&i>x{
                if !is_inside_quotes{
                    println!();
                    i=0;
                }
            }
            else if c==34{
                is_inside_quotes=!is_inside_quotes;
            }
        }
    }
}
