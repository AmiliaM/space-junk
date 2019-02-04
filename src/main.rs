use rand::Rng;
use std::io;
use std::collections::HashMap;

type Coord = (isize, isize, isize);

#[derive(Debug, Clone)]
struct Ship {
    pos: Coord,
    vel: Coord,
    view_distance: isize,
    food: isize,
    fuel: isize,
    landed: bool,
}

#[derive(Debug, Clone)]
struct Planet {
    remaining_resources: isize,
}

fn is_within(pos: &Coord, pos2: &Coord, range: isize) -> bool {
    if  (pos.0 > pos2.0-range) & 
        (pos.0 < pos2.0+range) &
        (pos.1 > pos2.1-range) &
        (pos.1 < pos2.1+range) &
        (pos.2 > pos2.2-range) &
        (pos.2 < pos2.2+range) {
            true
    }
    else {
        false
    }
}

fn move_ship(ship: &mut Ship) {
    ship.pos.0 += ship.vel.0;
    ship.pos.1 += ship.vel.1;
    ship.pos.2 += ship.vel.2;
}

fn random_pos(x: isize) -> Coord {
    (rand::thread_rng().gen_range(-x, x), rand::thread_rng().gen_range(-x, x), rand::thread_rng().gen_range(-x, x))
}

fn game_over(reason: &str) {
    println!("You lose!");
    println!("Cause of failure: {}", reason);
}

fn main() {
    let mut planets: HashMap<Coord, Planet> = HashMap::new();
    for _ in 0..60 {
        planets.insert(random_pos(100), Planet { remaining_resources: 3 });
    }
    let mut ship = Ship {
        view_distance: 30, 
        vel: (0, 0, 0),
        pos: random_pos(50),
        food: 25,
        fuel: 10,
        landed: false,
    };
    let mut input: String;
    loop {
        println!("");
        println!("Your location is {:?}", ship.pos);
        println!("Your velocity is {:?}", ship.vel);
        if ship.landed {
            println!("You are landed");
        }
        println!("You have {} food", ship.food);
        println!("You have {} fuel", ship.fuel);
        for (c, p) in planets.iter() {
            if is_within(&c, &ship.pos, ship.view_distance) {
                println!("You can see a planet at position {:?}", c);
            }
        }
        loop {
            println!("What would you like to do next? ");
            input = String::from("");
            io::stdin().read_line(&mut input).unwrap();
            input = input.to_lowercase();
            let args: Vec<_> = input.trim().split(' ').collect();
            match args[0] {
                "burn" => {
                    if ship.landed {
                        println!("You need to unland first!");
                    }
                    else if ship.fuel < 1 {
                        println!("Out of fuel!");
                    }
                    else {
                        match args[1] {
                            "east" | "e" => {
                                ship.vel.0 += 1;
                                ship.fuel -= 1;
                                break;
                            },
                            "west" | "w" => {
                                ship.vel.0 -= 1;
                                ship.fuel -= 1;
                                break;
                            },
                            "north" | "n" => {
                                ship.vel.1 += 1;
                                ship.fuel -= 1;
                                break;
                            },
                            "south" | "s" => {
                                ship.vel.1 -= 1;
                                ship.fuel -= 1;
                                break;
                            },
                            "up" | "u" => {
                                ship.vel.2 += 1;
                                ship.fuel -= 1;
                                break;
                            },
                            "down" | "d" => {
                                ship.vel.2 -= 1;
                                ship.fuel -= 1;
                                break;
                            },
                            _ => println!("invalid direction"),
                        }
                    }
                },
                "unland" => {
                    if ship.landed {
                        if ship.fuel < 1 {
                            println!("Not enough fuel!");
                        }
                        else {
                            ship.fuel -= 1;
                            ship.landed = false;
                            break;
                        }
                    }
                    else {
                        println!("You are not landed!");
                    }
                }
                "land" => {
                    if ship.landed {
                        println!("You are already landed!");
                    }
                    else if planets.contains_key(&ship.pos) {
                        if (ship.vel.0 == 0) & (ship.vel.1 == 0) & (ship.vel.2 == 0) {
                            if ship.fuel < 1 {
                                println!("Not enough fuel!");
                            }
                            else {
                                ship.landed = true;
                                ship.fuel =- 1;
                                println!("You have landed");
                                break;
                            }
                        }
                        else {
                            println!("You are moving too quickly to land!");
                        }
                    }
                    else {
                        println!("You can not land here!");
                    }
                }
                "mine" => {
                    if ship.landed {
                        if planets[&ship.pos].remaining_resources > 0 {
                            planets.get_mut(&ship.pos).unwrap().remaining_resources -= 1;
                            ship.food += 4;
                            println!("There are {} resources remaining", planets[&ship.pos].remaining_resources);
                            break;
                        }
                        else {
                            println!("Nothing left to mine");
                        }
                    }
                    else {
                        println!("You need to land first!");
                    }
                    
                }
                "wait" => break,
                _ => println!("invalid option"),
            }
        }
        ship.food -= 1;
        if ship.food < 0 {
            game_over("Starvation");
            return;
        }
        move_ship(&mut ship);
    }
}
