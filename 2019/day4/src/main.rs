fn main() {
    let lower = 145852;
    let upper = 616942;
    let mut counter = 0;

    'outer: for i in lower..=upper {
        let mut doubles = false;
        let mut skip = 0;
        let s = i.to_string();
        let cv: Vec<char> = s.chars().collect();
        let iv = vec![cv[0].to_digit(10), cv[1].to_digit(10), cv[2].to_digit(10), cv[3].to_digit(10), cv[4].to_digit(10), cv[5].to_digit(10)];

        
        'inner: for j in 0..=4 {
            if skip > 0 {
                skip -= 1;
                continue 'inner;
            }

            if iv[j] > iv[j+1] {
                continue 'outer;
            }

            if iv[j] == iv[j+1] { //doubles
                if j <= 3 {
                    if iv[j] == iv[j+2] && !doubles { //triples 
                        doubles = false;
                        skip += 1 ;
                        if j<= 2 { 
                            if iv[j] == iv[j+3] { //quads
                                skip +=1;
                                if j<= 1 { 
                                    if iv[j] == iv[j+4] { //quints (can't be any doubles)
                                        continue 'outer;
                                    }
                                }
                            }
                        }

                        continue 'inner;
                    }
                }
                doubles = true;
            }
        }

        if !doubles {
            continue 'outer;
        }

        counter += 1;
        
    }

    dbg!(counter);
}

