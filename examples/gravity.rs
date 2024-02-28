use complewise_iter::{IntoComplewiseIterator, LendingIterator};

#[derive(Debug)]
struct GravitationalBody {
    position: (f64, f64),
    mass: f64,
    force: (f64, f64)
}

/// Calculate the force on each body due to the other bodies.
fn main() {

    let mut bodies = vec![
        GravitationalBody { position: (0.0, 0.0), mass: 100.0, force: (0.0, 0.0) },
        GravitationalBody { position: (1.0, 1.0), mass: 200.0, force: (0.0, 0.0) },
        GravitationalBody { position: (2.0, 2.0), mass: 300.0, force: (0.0, 0.0) },
        GravitationalBody { position: (3.0, 3.0), mass: 400.0, force: (0.0, 0.0) },
        GravitationalBody { position: (4.0, 4.0), mass: 500.0, force: (0.0, 0.0) },
    ];

    let mut iter = bodies.complewise();
    while let Some((this, others)) = iter.next() {
        for other in others {
            let dx = other.position.0 - this.position.0;
            let dy = other.position.1 - this.position.1;
            let r2 = dx * dx + dy * dy;
            let f = other.mass / r2;
            this.force.0 += f * dx;
            this.force.1 += f * dy;
        }
    }

    for body in &bodies {
        println!("{:?}", body);
    }
}