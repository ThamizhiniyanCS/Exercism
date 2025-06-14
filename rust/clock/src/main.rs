use clock::Clock;

fn tests() {
    assert_eq!(Clock::new(8, 0).to_string(), "08:00");
    assert_eq!(Clock::new(11, 9).to_string(), "11:09");
    assert_eq!(Clock::new(24, 0).to_string(), "00:00");
    assert_eq!(Clock::new(25, 0).to_string(), "01:00");
    assert_eq!(Clock::new(100, 0).to_string(), "04:00");
    assert_eq!(Clock::new(1, 60).to_string(), "02:00");
    assert_eq!(Clock::new(0, 160).to_string(), "02:40");
    assert_eq!(Clock::new(0, 1723).to_string(), "04:43");
    assert_eq!(Clock::new(25, 160).to_string(), "03:40");
    assert_eq!(Clock::new(201, 3001).to_string(), "11:01");
    assert_eq!(Clock::new(72, 8640).to_string(), "00:00");

    assert_eq!(Clock::new(-1, 15).to_string(), "23:15");
    assert_eq!(Clock::new(-25, 0).to_string(), "23:00");
    assert_eq!(Clock::new(-91, 0).to_string(), "05:00");
    assert_eq!(Clock::new(1, -40).to_string(), "00:20");
    assert_eq!(Clock::new(1, -160).to_string(), "22:20");
    assert_eq!(Clock::new(1, -4820).to_string(), "16:40");
    assert_eq!(Clock::new(2, -60).to_string(), "01:00");
    assert_eq!(Clock::new(-25, -160).to_string(), "20:20");
    assert_eq!(Clock::new(-121, -5810).to_string(), "22:10");

    let clock = Clock::new(10, 0).add_minutes(3);
    assert_eq!(clock.to_string(), "10:03");

    let clock = Clock::new(6, 41).add_minutes(0);
    assert_eq!(clock.to_string(), "06:41");

    let clock = Clock::new(0, 45).add_minutes(40);
    assert_eq!(clock.to_string(), "01:25");

    let clock = Clock::new(10, 0).add_minutes(61);
    assert_eq!(clock.to_string(), "11:01");

    let clock = Clock::new(0, 45).add_minutes(160);
    assert_eq!(clock.to_string(), "03:25");

    let clock = Clock::new(23, 59).add_minutes(2);
    assert_eq!(clock.to_string(), "00:01");

    let clock = Clock::new(5, 32).add_minutes(1500);
    assert_eq!(clock.to_string(), "06:32");

    let clock = Clock::new(1, 1).add_minutes(3500);
    assert_eq!(clock.to_string(), "11:21");

    let clock = Clock::new(10, 3).add_minutes(-3);
    assert_eq!(clock.to_string(), "10:00");

    let clock = Clock::new(10, 3).add_minutes(-30);
    assert_eq!(clock.to_string(), "09:33");

    let clock = Clock::new(10, 3).add_minutes(-70);
    assert_eq!(clock.to_string(), "08:53");

    let clock = Clock::new(0, 3).add_minutes(-4);
    assert_eq!(clock.to_string(), "23:59");

    let clock = Clock::new(0, 0).add_minutes(-160);
    assert_eq!(clock.to_string(), "21:20");

    let clock = Clock::new(6, 15).add_minutes(-160);
    assert_eq!(clock.to_string(), "03:35");

    let clock = Clock::new(5, 32).add_minutes(-1500);
    assert_eq!(clock.to_string(), "04:32");

    let clock = Clock::new(2, 20).add_minutes(-3000);
    assert_eq!(clock.to_string(), "00:20");
}

fn main() {
    // println!("{}", Clock::new(1, -40))
    tests();
}
