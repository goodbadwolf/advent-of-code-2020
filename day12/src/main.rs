use std::fs;

#[derive(Debug)]
pub struct Ship {
    pub e: i32,
    pub n: i32,
    pub a: i32,
}

#[derive(Debug)]
pub struct Waypoint {
    pub e: i32,
    pub n: i32,
}

#[derive(Debug)]
pub struct ShipWithWaypoint {
    pub e: i32,
    pub n: i32,
    pub w: Waypoint,
}

fn parse_actions(input: &String) -> Vec<(char, i32)> {
    input
        .lines()
        .map(|i| {
            let (d, n) = i.split_at(1);
            (d.chars().nth(0).unwrap(), n.parse::<i32>().unwrap())
        })
        .collect()
}

fn simulate_nav(ship: &mut Ship, actions: &Vec<(char, i32)>) {
    for action in actions {
        let dir = action.0;
        let unit = action.1;
        match dir {
            'N' => {
                ship.n += unit;
            }
            'S' => {
                ship.n -= unit;
            }
            'E' => {
                ship.e += unit;
            }
            'W' => {
                ship.e -= unit;
            }
            'L' => {
                ship.a = (ship.a + unit) % 360;
            }
            'R' => {
                ship.a = (360 + ship.a - unit) % 360;
            }
            'F' => match ship.a {
                0 => {
                    ship.e += unit;
                }
                90 => {
                    ship.n += unit;
                }
                180 => {
                    ship.e -= unit;
                }
                270 => {
                    ship.n -= unit;
                }
                _ => {}
            },
            _ => {}
        }
    }
}

fn simulate_nav_with_waypoint(ship: &mut ShipWithWaypoint, actions: &Vec<(char, i32)>) {
    for action in actions {
        let dir = action.0;
        let unit = action.1;
        match dir {
            'N' => {
                ship.w.n += unit;
            }
            'S' => {
                ship.w.n -= unit;
            }
            'E' => {
                ship.w.e += unit;
            }
            'W' => {
                ship.w.e -= unit;
            }
            'L' => match unit {
                90 => {
                    ship.w = Waypoint {
                        e: -ship.w.n,
                        n: ship.w.e,
                    };
                }
                180 => {
                    ship.w = Waypoint {
                        e: -ship.w.e,
                        n: -ship.w.n,
                    };
                }
                270 => {
                    ship.w = Waypoint {
                        e: ship.w.n,
                        n: -ship.w.e,
                    };
                }
                _ => {}
            },
            'R' => match unit {
                90 => {
                    ship.w = Waypoint {
                        e: ship.w.n,
                        n: -ship.w.e,
                    };
                }
                180 => {
                    ship.w = Waypoint {
                        e: -ship.w.e,
                        n: -ship.w.n,
                    };
                }
                270 => {
                    ship.w = Waypoint {
                        e: -ship.w.n,
                        n: ship.w.e,
                    };
                }
                _ => {}
            },
            'F' => {
                ship.n += ship.w.n * unit;
                ship.e += ship.w.e * unit;
            }
            _ => {}
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input file");
    let actions = parse_actions(&input);
    let mut ship = Ship { n: 0, e: 0, a: 0 };
    simulate_nav(&mut ship, &actions);
    println!("Manhattan distance: {}", ship.n.abs() + ship.e.abs());
    let mut ship_w = ShipWithWaypoint {
        e: 0,
        n: 0,
        w: Waypoint { e: 10, n: 1 },
    };
    simulate_nav_with_waypoint(&mut ship_w, &actions);
    println!("Manhattan distance with waypoint: {}", ship_w.n.abs() + ship_w.e.abs());
}
