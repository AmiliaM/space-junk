use rand::Rng;
use std::io;

type Coord = (isize, isize);

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

#[derive(Debug, Clone)]
struct Ship {
    pos: Coord,
    vel: Coord,
    view_distance: isize,
}
#[derive(Debug, Clone)]
struct Planet {
    pos: Coord,
}

fn main() {
    let mut planets: Vec<Planet> = vec!();
    for _ in 0..30 {
        planets.push(Planet {pos: random_pos()});
    }
    let mut ship = Ship {
        view_distance: 30, 
        vel: (0, 0),
        pos: random_pos()};
    let mut input = String::new();
    while true {
        move_ship(&mut ship);
        println!("Your location is {:?}", ship.pos);
        println!("Your velocity is {:?}", ship.vel);
        for p in planets.iter() {
            if is_in_view(&ship, &p) {
                println!("You can see a planet at position {:?}", p.pos);
            }
        }
        println!("What would you like to do next? ");
        input = String::from("");
        io::stdin().read_line(&mut input).unwrap();
        let args: Vec<_> = input.trim().split(' ').collect();
        match args[0] {
            "burn" => {
                match args[1] {
                    "east" | "e" => ship.vel.0 += 1,
                    "west" | "w" => ship.vel.0 -= 1,
                    "north" | "n" => ship.vel.1 += 1,
                    "south" | "s" => ship.vel.1 -= 1,
                    _ => println!("invalid direction"),
                }
            },
            "wait" => {},
            _ => println!("invalid option"),
        }
    }
}
