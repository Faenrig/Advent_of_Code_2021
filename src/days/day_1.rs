pub fn calc_measurements() {
    let _measurements: Vec<u32> = vec![199, 200, 208, 210, 200,
                                       207, 240, 269, 260, 263];

    let mut _increasing_measurements: u32 = 0;
    let mut _position: usize = 0;
    for _value in _measurements.iter() {
        if _position == 0 {
            _position += 1;
            continue;
        }

        if _value > &_measurements[_position - 1] {
            _increasing_measurements += 1;
        }

        _position += 1;
    }
    println!("The number of increasing measurements is: {}", _increasing_measurements);
}
