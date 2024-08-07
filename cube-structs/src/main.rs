struct Cuboid {
    length: u64,
    bredth: u64,
    height: u64,
}

impl Cuboid {
    fn volume(&self) -> u64 {
        self.length * self.bredth * self.height
    }

    fn canfit(&self, another_cuboid: &Cuboid) -> bool {
        self.volume() > another_cuboid.volume()
    }

    fn cube(size: u64) -> Cuboid {
        return Cuboid {
            length: size,
            height: size,
            bredth: size,
        };
    }
}

impl std::fmt::Display for Cuboid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Cuboid(length: {}, bredth: {}, height: {})",
            self.length, self.bredth, self.height
        )
    }
}

fn main() {
    let cuboid1: Cuboid = Cuboid {
        length: 10,
        bredth: 5,
        height: 2,
    };
    let cuboid2: Cuboid = Cuboid {
        length: 8,
        bredth: 4,
        height: 3,
    };
    println!("{}", cuboid1);
    println!("Cuboid1 can fit Cuboid2: {}", cuboid1.canfit(&cuboid2));

    let cube: Cuboid = Cuboid::cube(23);

    println!("Cube: {}", cube);
}
