use csv::ReaderBuilder;

fn csv_to_vec(v: &mut Vec<u32>) {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path("input").unwrap();

    for result in rdr.records() {
        let record = &result.unwrap();
        let mut counter = 0;

        while counter < record.len() {
            let realrecord = record[counter].parse::<u32>().unwrap();
            v.push(realrecord);
            counter += 1; 
        }
    }
}

fn run(v: &mut Vec<u32>) -> u32 {
    let mut counter = 0;
    while counter < v.len() {
        let opcode = v[counter];
        let in1pos = v[counter+1] as usize;
        let in2pos = v[counter+2] as usize;
        let outpos = v[counter+3] as usize;
        match opcode {
            1 =>v[outpos] = v[in1pos] + v[in2pos],
            2 =>v[outpos] = v[in1pos] * v[in2pos],
            99 =>break,
            _ =>println!("shouldn't happen, noun: {}, verb: {}", v[1], v[2]),
        }
        counter += 4;

    }

    v[0]
}

fn brute(v: &mut Vec<u32>) {

    for noun in 0..99 {
        v[1] = noun;
        for verb in 0..99 {
            v[2] = verb;

            let mut vc: Vec<u32> = Vec::new();
            vc.resize(v.len(), 0);
            vc.copy_from_slice(&v[0..165]);

            if run(&mut vc) == 19690720 {
                println!("Noun: {}, Verb: {}", noun, verb);
            }
        }
    }

}

fn main() {
    let mut vec: Vec<u32> = Vec::new();

    csv_to_vec(&mut vec);
    brute(&mut vec);

}

