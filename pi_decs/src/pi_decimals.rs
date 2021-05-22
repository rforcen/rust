pub fn generate_pi_decimals(digits: usize) -> String {
    let words = digits / 4 + 3;
    let mut sum = vec![0_i32; words + 2];
    let mut term = vec![0_i32; words + 2];

    // -- 32*atan(1/10) -
    let mut denom = 3;
    sum[1] = 32;

    for firstword in 2..=words {
        let mut atan10 = |denom: i32| {
            sum[firstword] -= 3200 / denom;
            let mut remainder1 = 3200 % denom;
            sum[firstword] += 32 / (denom + 2);
            let mut remainder2 = 32 % (denom + 2);

            for x in firstword + 1..=words {
                let mut dividend = remainder1 * 10000;
                sum[x] -= dividend / denom;
                remainder1 = dividend % denom;
                dividend = remainder2 * 10000;
                sum[x] += dividend / (denom + 2);
                remainder2 = dividend % (denom + 2);
            }
        };
        atan10(denom);
        denom += 4;
    }

    let mut remainder = 40;

    for x in 2..=words {
        let digits = remainder * 10000;
        term[x] = digits / 239; // first term
        remainder = digits % 239;
        sum[x] -= term[x];
    }

    let mut firstword = 2;
    let mut denom = 3;

    while firstword < words {
        let mut atan239 = |denom: i32| {
            let (
                mut remainder1, // perform 1st divide implicitly
                mut remainder2,
                mut remainder3,
                mut remainder4,
            ) = (term[firstword], 0, 0, 0);
            firstword += 1;

            for x in firstword..=words {
                let mut temp = term[x];

                let mut dividend = remainder1 * 10000 + temp; // add next term
                temp = dividend / 57121;
                remainder1 = dividend % 57121;

                dividend = remainder2 * 10000 + temp;
                sum[x] += dividend / denom;
                remainder2 = dividend % denom;

                dividend = remainder3 * 10000 + temp; // subtract next term
                temp = dividend / 57121;
                remainder3 = dividend % 57121;

                dividend = remainder4 * 10000 + temp;
                sum[x] -= dividend / (denom + 2);
                remainder4 = dividend % (denom + 2);
                term[x] = temp;
            }

            firstword += 1;
            if term[firstword] == 0 {
                firstword += 1;
            }
        };

        atan239(denom);
        denom += 4;
    }

    // -- -16*atan(1/515) -
    firstword = 2;
    denom = 3;
    remainder = 160;

    for x in 2..=words {
        let digits = remainder * 10000;
        term[x] = digits / 515; // first term
        remainder = digits % 515;
        sum[x] -= term[x];
    }

    while firstword < words {
        let mut atan515 = |denom: i32| {
            let (mut remainder1, mut remainder2, mut remainder3, mut remainder4) =
                (term[firstword], 0, 0, 0);
            firstword += 1;

            for x in firstword..=words {
                let mut temp = term[x];
                if remainder1 < 214745 {
                    let dividend = remainder1 * 10000 + temp; // add next term
                    temp = dividend / 265225;
                    remainder1 = dividend % 265225;
                } else {
                    remainder1 -= 53045;
                    let dividend = remainder1 * 10000 + temp;
                    temp = dividend / 265225;
                    remainder1 = dividend % 265225;
                    temp += 2000;
                }
                let dividend = remainder2 * 10000 + temp;
                sum[x] += dividend / denom;
                remainder2 = dividend % denom;

                if remainder3 < 214745 {
                    // subtract next term
                    let dividend = remainder3 * 10000 + temp;
                    temp = dividend / 265225;
                    remainder3 = dividend % 265225;
                } else {
                    remainder3 -= 53045;
                    let dividend = remainder3 * 10000 + temp;
                    temp = dividend / 265225;
                    remainder3 = dividend % 265225;
                    temp += 2000;
                }
                let dividend = remainder4 * 10000 + temp;
                sum[x] -= dividend / (denom + 2);
                remainder4 = dividend % (denom + 2);
                term[x] = temp;
            }

            firstword += 1;
            if term[firstword] == 0 {
                firstword += 1;
            }
        };
        atan515(denom);
        denom += 4;
    }

    for x in (1..=words).rev() {
        // release carries & borrows
        if sum[x] < 0 {
            sum[x - 1] += sum[x] / 10000;
            sum[x] = sum[x] % 10000;
            sum[x - 1] -= 1;
            sum[x] += 10000;
        }
        if sum[x] >= 10000 {
            sum[x - 1] += sum[x] / 10000;
            sum[x] = sum[x] % 10000;
        }
    }

    // generate string from sum
    let mut s = String::with_capacity(digits + 3);
    s += "3.1";
    for i in (2..words).step_by(3) {
        s.push_str(&format!("{:04}{:04}{:04}", sum[i], sum[i + 1], sum[i + 2]));
    }
    for i in 3 * (words / 3) + 2..words {
        s.push_str(&format!("{:04}", sum[i]));
    }
    s
}
