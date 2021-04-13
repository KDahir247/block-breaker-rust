use bevy::prelude::*;
use bevy::winit::*;
use bevy_rapier2d::physics::*;
use bevy_rapier2d::render::*;
use bevy_rapier2d::rapier::dynamics::*;
use bevy_rapier2d::rapier::geometry::{ColliderBuilder, ColliderHandle, ContactEvent};
use bevy_rapier2d::rapier::na::{Vector, Vector2};
use bevy_rapier2d::na::Isometry;
use bevy_rapier2d::rapier::parry::na::Isometry2;
use rapier2d::parry::na::clamp;
use rand::{Rng, thread_rng};

pub struct Paddle(i8);
pub  struct Brick(i8);

fn main() {
    App::build()
        .insert_resource(WindowDescriptor{
            title : "BlockSmash".to_string(),
            width : 800.,
            height : 650.,
            resizable : false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WinitPlugin::default())
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(RapierRenderPlugin)
        .add_startup_system(setup_physics.system())
        .add_system(move_paddle.system())
        .add_system(handle_physics_event.system())
        .run();
}

fn setup_physics(mut commands:  Commands) {

    //Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let rand_linear_x:f32 = thread_rng().gen_range(-100..=100) as f32;
    let rand_linear_y:f32  = thread_rng().gen_range(-30..=-5) as f32;

    let paddle_rigidbody = RigidBodyBuilder::new_kinematic()
        .lock_rotations()
        .translation(0.,-300.);

    let paddle_collider = ColliderBuilder::cuboid(35.0, 5.0)
        .friction(0.);

    commands.spawn_bundle((paddle_rigidbody, paddle_collider))
        .insert(Paddle(8));

    // Dynamic rigid-body with ball shape.
    let rigid_body2 = RigidBodyBuilder::new_dynamic()
        .translation(0.0, 300.)
        .gravity_scale(9.81)
        .linvel(rand_linear_x,rand_linear_y);

    let collider2 = ColliderBuilder::ball(10.).friction(0.).restitution(2.02);
    commands.spawn_bundle((rigid_body2, collider2));

    //Might use a spawn bundle.
    //Left Wall
    let lhs_rigidbody = RigidBodyBuilder::new_static()
        .translation(-400., 0.)
        .can_sleep(true);

    let lhs_collider = ColliderBuilder::cuboid(35.,400.)
        .friction(0.);

    commands.spawn_bundle((lhs_rigidbody, lhs_collider));


    //Right Wall
    let rhs_rigidbody = RigidBodyBuilder::new_static()
        .translation(400.,0.)
        .can_sleep(true);

    let rhs_collider = ColliderBuilder::cuboid(35.,400.)
        .friction(0.);

    commands.spawn_bundle((rhs_rigidbody, rhs_collider));

    let top_rigidbody = RigidBodyBuilder::new_static()
        .translation(0., 300.);

    let top_collider = ColliderBuilder::cuboid(400., 35.)
        .friction(0.);

    commands.spawn_bundle((top_rigidbody, top_collider));

    spawn_bricks(commands, Vec2::new(3.,2.), Vec2::new(100.,65.));
}

fn move_paddle(key_board : Res<Input<KeyCode>>, mut rigid_bodies : ResMut<RigidBodySet>, mut physics_query : Query<(&mut Transform, &Paddle, &RigidBodyHandleComponent)>){
    for (mut transform, paddle, rigidbody_component) in physics_query.iter_mut() {
        let x_axis = (-(key_board.pressed(KeyCode::A) as i8) + (key_board.pressed(KeyCode::D) as i8)) * paddle.0;

        if let Some(rb) = rigid_bodies.get_mut(rigidbody_component.handle()){

            if x_axis != 0 {
                rb.set_next_kinematic_position(Isometry2::translation(clamp(transform.translation.x + x_axis as f32, -335.,335.), -300.0));
            }
        }
    }
}



fn spawn_bricks(mut commands : Commands, row_col : Vec2, padding : Vec2){
    for column in (-row_col.y as i8)..=row_col.y as i8 {
        for row in (-row_col.x as i8)..=row_col.x as i8 {

            let brick_rigidbody = RigidBodyBuilder::new_kinematic()
                .translation(row as f32 * padding.x , column as f32 * padding.y + 50.);

            let brick_collider = ColliderBuilder::cuboid(25., 5.)
                .friction(0.);


            commands.spawn_bundle((brick_rigidbody, brick_collider)).insert(Brick(3));
        }
    }
}


fn handle_physics_event(mut commands : Commands, event : Res<EventQueue>, mut destroyable_query : Query<(Entity, &ColliderHandleComponent, &mut Brick)>){
    while let Ok(evt) = event.contact_events.pop(){

        let mut collider_pair : (ColliderHandle, ColliderHandle) = (ColliderHandle::invalid(), ColliderHandle::invalid());

        match evt {
            ContactEvent::Stopped(collider_a, collider_b) => { collider_pair = (collider_a, collider_b);}
            _ => {}
        }

        for (entity, collider, mut brick) in destroyable_query.iter_mut() {

            if collider_pair.0 == collider_pair.1 {
                return;
            }

            if collider.handle().eq(&collider_pair.1) {

                brick.0 -= 1;

                if brick.0 <= 0 {

                    commands.entity(entity).despawn();

                }
            }

        }

        //println!("Collision happened {:?}",evt);
    }

}
