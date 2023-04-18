use crate::particle::Particle;
use crate::util::{distance, collide, v, overlap, solve_overlap, rcolor};
use ggez::graphics::Color;

#[test] 
fn util_collide_distance_particle_update() {
    let pos = v(600.0, 300.0);
    let vel = v(15.0, 15.0);
    let acc = v(-15.0, -15.0);

    let mut p1 = Particle::new(pos, vel, acc, 6.0, 10, Color::BLACK);
    let mut p2 = Particle::new(pos, acc, acc, 6.0, 10, Color::BLACK);
    let p3 = Particle::new(pos, vel, acc, 6.0, 10, Color::BLACK);

    assert_eq!(&p1, &p3);
    assert_ne!(&p1, &p2);
    assert!(collide(&p1, &p2));
    assert!(distance(&p1, &p2) == 0.0);
        
    let dt = 1.0 / 60.0;
    for _i in 0..60 {
        p1.update(dt);    
        p2.update(dt)
    }

    assert_ne!(distance(&p1, &p2), 0.0);
    assert!(!collide(&p1, &p2)); 
}

#[test] 
fn util_overlap_solve_overlap() {
    let mut p1 = Particle::new(v(5.0, 10.0), v(0.0, 0.0), v(0.0, 0.0), 100.0, 10, rcolor());  
    let mut p2 = Particle::new(v(10.0, 10.0), v(0.0, 0.0), v(0.0, 0.0), 100.0, 10, rcolor());  

    assert_eq!(distance(&p1, &p2), 5.0);
    assert_eq!(overlap(&p1, &p2), 195.0);

    solve_overlap(&mut p1, &mut p2);

    assert_eq!(p1.p.x, 5.0 - 195.0 * 0.5);
    assert_eq!(p2.p.x, 10.0 + 195.0 * 0.5);

}


