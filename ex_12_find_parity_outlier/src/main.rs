fn main() {
    //  std::i32::MAX == 2_147_483_647

    let array = [
        -33921, 20625, -3109, 5617, -27911, 60641, 43885, -15719, -48409, -37787, -35943, 50405,
        -20223, 35543, -22519, 63053, 62055, 40959, 15827, -32077, 30665, -21649, 22367, 6209,
        4509, -63573, 12115, -34741, 27117, -55759, -62941, -40353, -31343, -40717, -39285, -29461,
        28493, -57865, 10587, -23417, 32269, -42687, -26179, 40451, 29167, -39043, 29859, -35975,
        31117, 10495, 17247, 17253, 55489, -60639, 52035, -6675, -63101, -36085, 17105, 51303,
        -59085, 50473, 53433, 48171, 8099, 61773, 55687, -27021, -8303, 19765, 62519, -21566,
        -32659, 44287, 57349, -14439, 28625, -25565, 24483, 22187, 47713, 12797, 41905, 31811,
        -48035, -45275, 57865, 63937, -29839, -36205, 14701, -21153, 60417, 48597, -18061, -8561,
        47743, -31451, 49357, -21869, 1609, 58005, 45345, 19579, -5699, -28119, -52775, 25925,
        -54697, 46635, -6497, 55207, 45041, 5365, -18943, -8915, 13547, 5067, 58565, -33181, 63849,
        15797, 58143, 22293, -6721, 31353, -49605, -1705, -11191, 8363, 14775, -24301, 64251,
        61867, 40841, 15309, 55551, 42393, 16751, -49227, -3267, -37137, -54497, -23785, 53787,
        28843, -17239, 62077, -43973, 51001, 1061, -62137, -16215, 7045, -38125, -10671, -40647,
        -59843, -28345, 9189, -17369, -41071, -20087, -28563, 765, 55999, 17489, -12577, -53311,
        -29335, -4567, -15455, -34731, -13477, -47811, -64023, -21171, 30871, 59353, 36615, -48179,
        -35265, -36713, -50001, -56759, -6983, 7025, -5561, -21473, -47409, -38715, -61911, 64197,
        17839,
    ];
    let result = find_outlier(&array);
    //let result = find_outlier(&[ std::i32::MAX, 0, 1]);
    println!("{result}");
}

#[test]
fn basic_test() {
    let t1 = [2, 6, 8, -10, 3];
    let t2 = [
        206847684, 1056521, 7, 17, 1901, 21104421, 7, 1, 35521, 1, 7781,
    ];
    let t3 = [std::i32::MAX, 0, 1];
    assert_eq!(3, find_outlier(&t1));
    assert_eq!(206847684, find_outlier(&t2));
    assert_eq!(0, find_outlier(&t3));

    assert_eq!(11, find_outlier(&[2, 4, 0, 100, 4, 11, 2602, 36]));
    assert_eq!(160, find_outlier(&[160, 3, 1719, 19, 11, 13, -21]));
}

fn find_outlier_version_no_pasa_test(values: &[i32]) -> i32 {
    let cant = values.len() / 3;
    println!("cantidad: {cant}");
    let mut temporal = [0; 3];

    for n in 0..cant {
        //println!("revisando : {n}");
        temporal[0] = values[n * 3] % 2;
        temporal[1] = values[(n * 3) + 1] % 2;
        temporal[2] = values[(n * 3) + 2] % 2;

        if temporal[0] == temporal[1] && temporal[1] != temporal[2] {
            return values[(n * 3) + 2];
        }
        if temporal[0] == temporal[2] && temporal[2] != temporal[1] {
            return values[(n * 3) + 1];
        }
        if temporal[1] == temporal[2] && temporal[2] != temporal[0] {
            return values[n * 3];
        }
    }
    let residuo = values.len() % 3;
    if residuo > 0 {
        for n in &values[values.len() - residuo..] {
            if n % 2 != temporal[0] {
                return *n;
            }
        }
    }
    return 0;
}

fn find_outlier(values: &[i32]) -> i32 {
    if values[0].abs() % 2 != values[1].abs() % 2 {
        if values[0].abs() % 2 == values[2].abs() % 2 {
            return values[1];
        } else {
            return values[0];
        }
    }

    let current = values[0].abs() % 2;
    for i in &values[2..] {
        if i.abs() % 2 != current {
            return *i;
        }
    }
    panic!("no hay !!!");
}
