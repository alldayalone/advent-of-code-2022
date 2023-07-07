1. какие доступны действия? (открыть кран, идти по кратчайшему маршруту в комнату с ненулевым краном)
2. какая фитнес функция? pressure

1 2 3 4 5 6 7 8


best so far: 
SolutionTree {
    depth: 30,
    flow_rate: 122,
    pressure: 1691,
    current_valve_tag: "UV",
    opened_valves: [
        "KF",
        "SE",
        "NA",
        "IJ",
        "CO",
        "AE",
        "EU",
        "UK",
    ],
    closed_valves: [
        "QN",
        "DS",
        "GJ",
        "QB",
        "MN",
        "CS",
        "XM",
    ],
    visited_valves: [
        "UK",
        "UK",
    ],
    actions: [
        Move(
            "LK",
        ),
        Move(
            "XM",
        ),
        Move(
            "BQ",
        ),
        Move(
            "KF",
        ),
        Open(
            "KF",
        ),
        Move(
            "JW",
        ),
        Move(
            "AD",
        ),
        Move(
            "SE",
        ),
        Open(
            "SE",
        ),
        Move(
            "YR",
        ),
        Move(
            "OS",
        ),
        Move(
            "NA",
        ),
        Open(
            "NA",
        ),
        Move(
            "WE",
        ),
        Move(
            "IJ",
        ),
        Open(
            "IJ",
        ),
        Move(
            "XN",
        ),
        Move(
            "ED",
        ),
        Move(
            "CO",
        ),
        Open(
            "CO",
        ),
        Move(
            "PT",
        ),
        Move(
            "AE",
        ),
        Open(
            "AE",
        ),
        Move(
            "IR",
        ),
        Move(
            "EU",
        ),
        Open(
            "EU",
        ),
        Move(
            "DT",
        ),
        Move(
            "UK",
        ),
        Open(
            "UK",
        ),
        Move(
            "UV",
        ),
    ],
    children: [],
}

// before optimization
109, calls total: 25560000
109, calls total: 25570000

// 5x: change String to &str and remove cloning
25, calls total: 25560000

// 13x: change &str to usize 
2, calls total: 2.5M
Best pressure: 2299 - нахожу за 40 секунд

384, calls total: 427M
Best pressure: 2341
385, calls total: 428M

// 25% ускорение за счет u16 = u8 + u8

