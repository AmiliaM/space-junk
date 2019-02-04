use rand::Rng;
use std::io;

type Coord = (isize, isize);

#[derive(Debug, Clone)]
struct Ship {
    pos: Coord,
    vel: Coord,
    view_distance: isize,
    food: isize,
    fuel: isize,
}

#[derive(Debug, Clone)]
struct Planet {
    pos: Coord,
}

fn is_in_view(viewer: &Ship, obj: &Planet) -> bool {
    if  (obj.pos.0 > viewer.pos.0-viewer.view_distance) & 
        (obj.pos.0 < viewer.pos.0+viewer.view_distance) &
        (obj.pos.1 > viewer.pos.1-viewer.view_distance) &
        (obj.pos.1 < viewer.pos.1+viewer.view_distance) {
            true
    }
    else {
        false
    }
}

fn move_ship(ship: &mut Ship) {
    ship.pos.0 += ship.vel.0;
    ship.pos.1 += ship.vel.1;
}

fn random_pos() -> Coord {
    (rand::thread_rng().gen_range(-100, 100), rand::thread_rng().gen_range(-100, 100))
}

fn game_over(reason: &str) {
    println!("You lose!");
    println!("Cause of failure: {}", reason);
}

fn main() {
    let mut planets: Vec<Planet> = vec!();
    for _ in 0..30 {
        planets.push(Planet {pos: random_pos()});
    }
    let mut ship = Ship {
        view_distance: 30, 
        vel: (0, 0),
        pos: random_pos(),
        food: 25,
        fuel: 10,
    };
    let mut input: String;
    loop {
        println!("");
        println!("Your location is {:?}", ship.pos);
        println!("Your velocity is {:?}", ship.vel);
        println!("You have {} food", ship.food);
        println!("You have {} fuel", ship.fuel);
        for p in planets.iter() {
            if is_in_view(&ship, &p) {
                println!("You can see a planet at position {:?}", p.pos);
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
                    if ship.fuel < 1 {
                        println!("Out of fuel!");
                        break;
                    }
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
                        _ => println!("invalid direction"),
                    }
                },
                "wait" => break,
                _ => println!("invalid option"),
            }
        }
        ship.food -= 1;
        if ship.food < 0 {
            game_over("Out of food");
            return;
        }
        move_ship(&mut ship);
    }
}
