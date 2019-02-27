use ansi_term::Color::*;
use rand::Rng;
use std::collections::HashMap;
use std::io;

type Coord = (isize, isize, isize);

#[derive(Debug, Clone)]
struct Ship {
    pos: Coord,
    vel: Coord,
    view_distance: isize,
    food: isize,
    fuel: isize,
    metal: isize,
    crew: isize,
    food_consumption: isize,
    fuel_consumption: isize,
    landed: bool,
}

#[derive(Debug, Clone)]
struct Planet {
    remaining_resources: isize,
}

#[derive(Debug, Clone)]
struct Asteroid {
    remaining_resources: isize,
    resource: Resource,
}

#[derive(Debug, Clone)]
enum Resource {
    Metal,
    Fuel,
}

#[derive(Debug, Clone)]
enum Object {
    Planet(Planet),
    Asteroid(Asteroid),
}

fn is_within(pos: &Coord, pos2: &Coord, range: isize) -> bool {
    let d = ((pos.0 - pos2.0).pow(2) + (pos.1 - pos2.1).pow(2) + (pos.2 - pos2.2).pow(2)) as f64;
    if d.sqrt() < range as f64 {
        true
    } else {
        false
    }
}

fn move_ship(ship: &mut Ship) {
    ship.pos.0 += ship.vel.0;
    ship.pos.1 += ship.vel.1;
    ship.pos.2 += ship.vel.2;
}

fn random_pos(x: isize) -> Coord {
    (
        rand::thread_rng().gen_range(-x, x),
        rand::thread_rng().gen_range(-x, x),
        rand::thread_rng().gen_range(-x, x),
    )
}

fn game_over(reason: &str) {
    let m = String::from("You lose!\n") + "Cause of failure: " + reason;
    println!("{}", Red.normal().paint(m));
}

fn main() {
    let mut objects: HashMap<Coord, Object> = HashMap::new();
    for _ in 0..25 {
        let mut b = true;
        let r = random_pos(100);
        for (c, _) in objects.clone() {
            if is_within(&c, &r, 3) {
                b = false;
                break;
            }
        }
        if b {
            objects.insert(
                r,
                Object::Planet(Planet {
                    remaining_resources: 3,
                }),
            );
        }
    }
    for _ in 0..80 {
        let mut b = true;
        let r = random_pos(100);
        for (c, _) in objects.clone() {
            if is_within(&c, &r, 3) {
                b = false;
                break;
            }
        }
        if b {
            objects.insert(
                r,
                Object::Asteroid(Asteroid {
                    remaining_resources: 3,
                    resource: match rand::thread_rng().gen_bool(0.5) {
                        true => Resource::Fuel,
                        false => Resource::Metal,
                    },
                }),
            );
        }
    }
    let mut ship = Ship {
        view_distance: 30,
        vel: (0, 0, 0),
        pos: random_pos(50),
        food: 210,
        fuel: 50,
        metal: 9,
        crew: 3,
        fuel_consumption: 2,
        food_consumption: 2,
        landed: false,
    };
    println!("Controls:");
    println!("burn (N/S/E/W/U/D)");
    println!("land/launch");
    println!("mine");
    println!("upgrade");
    println!("wait");
    let mut input: String;
    loop {
        println!("");
        println!("                       (E/W, N/S, U/D)");
        println!("Your location is       {:<3?}", ship.pos);
        println!("Your velocity is       {:<3?}", ship.vel);
        for (c, o) in objects.iter() {
            if is_within(&c, &ship.pos, ship.view_distance) {
                match o {
                    Object::Planet(_) => println!("You see a planet at    {:<3?}", c),
                    Object::Asteroid(_) => println!("You see an asteroid at {:<3?}", c),
                }
            }
        }
        if ship.landed {
            println!("You are landed");
        }
        println!("You have {} food", ship.food);
        println!("You have {} fuel", ship.fuel);
        println!("You have {} crew", ship.crew);
        println!("You have {} metal", ship.metal);

        loop {
            println!("What would you like to do next? ");
            input = String::from("");
            io::stdin().read_line(&mut input).unwrap();
            input = input.to_lowercase();
            let args: Vec<_> = input.trim().split(' ').collect();
            match args[0] {
                "burn" => {
                    if ship.landed {
                        let m = "You need to launch first!";
                        println!("{}", Red.normal().paint(m));
                    } else if ship.fuel < 1 {
                        let m = "Out of fuel!";
                        println!("{}", Red.normal().paint(m));
                    } else {
                        match args[1] {
                            "east" | "e" => {
                                ship.vel.0 += 1;
                                ship.fuel -= ship.fuel_consumption;
                                break;
                            }
                            "west" | "w" => {
                                ship.vel.0 -= 1;
                                ship.fuel -= ship.fuel_consumption;
                                break;
                            }
                            "north" | "n" => {
                                ship.vel.1 += 1;
                                ship.fuel -= ship.fuel_consumption;
                                break;
                            }
                            "south" | "s" => {
                                ship.vel.1 -= 1;
                                ship.fuel -= ship.fuel_consumption;
                                break;
                            }
                            "up" | "u" => {
                                ship.vel.2 += 1;
                                ship.fuel -= ship.fuel_consumption;
                                break;
                            }
                            "down" | "d" => {
                                ship.vel.2 -= 1;
                                ship.fuel -= ship.fuel_consumption;
                                break;
                            }
                            _ => println!("invalid direction"),
                        }
                    }
                }
                "launch" => {
                    if ship.landed {
                        if ship.fuel < 1 {
                            let m = "Out of fuel!";
                            println!("{}", Red.normal().paint(m));
                        } else {
                            ship.fuel -= 1;
                            ship.landed = false;
                            break;
                        }
                    } else {
                        let m = "You are not landed!";
                        println!("{}", Red.normal().paint(m));
                    }
                }
                "land" => {
                    if ship.landed {
                        println!("You are already landed!");
                    } else if objects.contains_key(&ship.pos) {
                        if (ship.vel.0 == 0) & (ship.vel.1 == 0) & (ship.vel.2 == 0) {
                            if ship.fuel < 1 {
                                let m = "Out of fuel!";
                                println!("{}", Red.normal().paint(m));
                            } else {
                                ship.landed = true;
                                ship.fuel -= 1;
                                let m = "You have landed";
                                println!("{}", Green.normal().paint(m));
                                break;
                            }
                        } else {
                            let m = "You are moving too quickly to land!";
                            println!("{}", Red.normal().paint(m));
                        }
                    } else {
                        let m = "You can not land here!";
                        println!("{}", Red.normal().paint(m));
                    }
                }
                "mine" => {
                    if ship.landed {
                        objects.entry(ship.pos).and_modify(|object| match object {
                            Object::Planet(ref mut p) => {
                                if p.remaining_resources > 0 {
                                    p.remaining_resources -= 1;
                                    ship.food += ship.crew;
                                    println!(
                                        "There are {} resources remaining",
                                        p.remaining_resources
                                    );
                                } else {
                                    let m = "Nothing left to mine";
                                    println!("{}", Red.normal().paint(m));
                                }
                            }
                            Object::Asteroid(ref mut a) => {
                                if a.remaining_resources > 0 {
                                    a.remaining_resources -= 1;
                                    match a.resource {
                                        Resource::Fuel => ship.fuel += ship.crew,
                                        Resource::Metal => ship.metal += ship.crew,
                                    }
                                    println!(
                                        "There are {} resources remaining",
                                        a.remaining_resources
                                    );
                                } else {
                                    let m = "Nothing left to mine";
                                    println!("{}", Red.normal().paint(m));
                                }
                            }
                        });
                        break;
                    } else {
                        let m = "You need to land first!";
                        println!("{}", Red.normal().paint(m));
                    }
                }
                "wait" => break,
                "airlock" => {
                    let m = "You throw one of the crew out of the airlock!";
                    println!("{}", Red.normal().paint(m));
                    ship.crew -= 1;
                    break;
                }
                "upgrade" => {
                    if ship.metal >= 18 {
                        ship.food_consumption -= 1;
                        ship.fuel_consumption -= 1;
                        ship.metal -= 18;
                        break;
                    } else {
                        let m = "Not enough metal!";
                        println!("{}", Red.normal().paint(m));
                    }
                }
                _ => {
                    let m = "Invalid option";
                    println!("{}", Red.normal().paint(m));
                }
            }
        }
        if rand::thread_rng().gen_range(1, 50) == 1 {
            ship.crew -= 1;
            let m = "One of the crew has died in an accident!";
            println!("{}", Red.normal().paint(m));
        }
        ship.food -= ship.food_consumption * ship.crew;
        if ship.food < 0 {
            game_over("Starvation");
            return;
        }
        if ship.crew < 1 {
            game_over("All crew are dead");
            return;
        }
        move_ship(&mut ship);
    }
}
