use legion::*;

#[derive(Clone, Copy, Debug)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Clone, Copy, Debug)]
struct Velocity {
    dx: f32,
    dy: f32,
}

fn main() {
    let mut world = World::default();

    let _ = world.extend(vec![
        (Position { x: 0.0, y: 0.0 }, Velocity { dx: 0.1, dy: 0.0 }),
        (Position { x: 1.0, y: 1.0 }, Velocity { dx: 0.1, dy: 0.1 }),
        (Position { x: 2.0, y: 2.0 }, Velocity { dx: 0.0, dy: 0.1 }),
    ]);

    for frame in 1..=3 {
        println!("frame {}", frame);

        // Update.
        let mut query = <(&Velocity, &mut Position)>::query()
            .filter(maybe_changed::<Position>());

        for (velocity, position) in query.iter_mut(&mut world) {
            position.x += velocity.dx;
            position.y += velocity.dy;
        }

        // Draw.
        let mut query = <&Position>::query();

        for position in query.iter(&world) {
            println!("x: {:.2}, y: {:.2}", position.x, position.y);
        }

        println!();
    }
}
