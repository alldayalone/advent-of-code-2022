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

// ускорение 2x - благодаря bitmap
Best pressure: 2299
18, calls total: 46M

171, calls total: 427M
Best pressure: 2341
171, calls total: 428M

Best pressure: 2422
385, calls total: 958M

15! / 1e+9 * 400 /60/60/24 = 6 days to run for full solution.
we need to improve...

669, calls total: 1663.0001M
669, calls total: 1664M
669, calls total: 1664.9999M
670, calls total: 1666M
670, calls total: 1667.0001M
671, calls total: 1668M
671, calls total: 1668.9999M
671, calls total: 1670M
672, calls total: 1671.0001M
672, calls total: 1672M
673, calls total: 1672.9999M
673, calls total: 1674M
674, calls total: 1675.0001M
674, calls total: 1676M
674, calls total: 1676.9999M
675, calls total: 1678M
675, calls total: 1679.0001M
676, calls total: 1680M
676, calls total: 1680.9999M
676, calls total: 1682M
677, calls total: 1683.0001M
677, calls total: 1684M
678, calls total: 1684.9999M
678, calls total: 1686M
678, calls total: 1687.0001M
679, calls total: 1688M
679, calls total: 1688.9999M
680, calls total: 1690M


LOL 2422 IS THE CORRECT ONE!
