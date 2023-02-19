mod mini_engine;
use std::collections::HashMap;

use kmath::*;
use mini_engine::*;
use rapier2d::{na::Point2, prelude::*};

const WORLD_SCALE_FACTOR: f32 = 0.05 / 20.0;

struct PlayerPointerInfo {
    moving_collider: Option<ColliderHandle>,
    cursor_position: Vec2,
    last_cursor_position: Vec2,
    offset: Vec2,
    cursor_down: bool,
    render: bool,
    color: (u8, u8, u8),
}

impl PlayerPointerInfo {
    fn new(color: (u8, u8, u8)) -> Self {
        Self {
            moving_collider: None,
            cursor_position: Vec2::ZERO,
            last_cursor_position: Vec2::ZERO,
            offset: Vec2::ZERO,
            cursor_down: false,
            render: false,
            color,
        }
    }
}

struct PhysicsObject {
    rigid_body_handle: RigidBodyHandle,
    color: (u8, u8, u8),
}

const COLORS: &[(u8, u8, u8)] = &[
    (88, 128, 211),
    (240, 64, 23),
    (15, 141, 86),
    (234, 183, 18),
    // (160, 95, 242),
];

fn main() {
    // The key is the player id and the pointer id.
    let mut player_pointers: HashMap<(u32, u32), PlayerPointerInfo> = HashMap::new();
    let mut player_colors = HashMap::new();

    let mut rapier = RapierIntegration::new();
    // let random = oorandom::Rand32::new(14);
    let mut physics_objects = Vec::new();

    let mut random = oorandom::Rand32::new(19);
    /*
    physics_objects.push(add_polyline(
        &mut rapier,
        3.0,
        5.0,
        &[Vec2::ZERO, Vec2::ONE, Vec2::X],
    ));
    */
    // physics_objects.push(add_ball(&mut rapier, &mut random, 0.5, 0.5, 0.1));

    let floor_half_depth = 0.02;
    let floor_height = 1.5 - floor_half_depth;
    physics_objects.push(add_rectangle(
        &mut rapier,
        &mut random,
        0.5,
        floor_height + floor_half_depth,
        4.0,
        floor_half_depth,
        true,
        Some((222, 175, 166)),
    ));

    let size = 0.1;
    let padding = 0.01;
    for i in 0..3 {
        for j in 0..3 {
            physics_objects.push(add_rectangle(
                &mut rapier,
                &mut random,
                0.8 + i as f32 * (size * 2.0 + padding),
                floor_height - j as f32 * (size * 2.0 + padding),
                size,
                size,
                false,
                None,
            ));
        }
    }

    /*
    for j in 0..5 {
        physics_objects.push(add_rectangle(
            &mut rapier,
            &mut random,
            1.8,
            j as f32 * 0.06,
            0.05,
            0.05,
            false,
        ));
    }
    */
    for j in 0..4 {
        physics_objects.push(add_rectangle(
            &mut rapier,
            &mut random,
            1.8,
            j as f32 * 0.2,
            0.3,
            0.05,
            false,
            None,
        ));
    }

    for _ in 0..5 {
        physics_objects.push(add_shape(&mut rapier, &mut random, 2.0, 0.5, 3, 0.12));
    }

    for _ in 0..3 {
        physics_objects.push(add_ball(&mut rapier, &mut random, 0.4, 0.5, 0.1));
    }

    mini_engine::run(move |event| match event {
        Event::FixedUpdate => {
            for pointer in player_pointers.values() {
                if let Some(collider) = pointer.moving_collider {
                    let collider = rapier.collider_set.get(collider).unwrap();
                    let rigid_body = rapier
                        .rigid_body_set
                        .get_mut(collider.parent().unwrap())
                        .unwrap();

                    let p = pointer.cursor_position + pointer.offset;
                    rigid_body.set_translation([p.x, p.y].into(), true);
                }
            }
            rapier.step();
        }
        Event::Draw => {
            for PhysicsObject {
                rigid_body_handle,
                color,
            } in physics_objects.iter()
            {
                let rigid_body = rapier.rigid_body_set.get(*rigid_body_handle).unwrap();
                for collider in rigid_body.colliders() {
                    let collider = rapier.collider_set.get(*collider).unwrap();
                    let shape = collider.shape();

                    let matrix = collider.position().to_matrix();
                    let matrix = matrix.scale(1.0 / WORLD_SCALE_FACTOR);
                    set_transform(
                        matrix[0], matrix[1], matrix[3], matrix[4], matrix[6], matrix[7],
                    );
                    set_color(color.0, color.1, color.2, 255);
                    match shape.shape_type() {
                        ShapeType::Ball => {
                            let ball = shape.as_ball().unwrap();

                            draw_circle(0.0, 0.0, ball.radius);
                        }
                        ShapeType::Cuboid => {
                            let rect = shape.as_cuboid().unwrap();
                            let extents = rect.half_extents;

                            draw_rect(-extents.x, -extents.y, extents.x * 2.0, extents.y * 2.0);
                        }
                        ShapeType::ConvexPolygon => {
                            let convex_polygon = shape.as_convex_polygon().unwrap();
                            convex_polygon.points();
                            begin_path();
                            let points = convex_polygon.points();
                            move_to(points[0].x, points[0].y);
                            for p in &convex_polygon.points()[1..] {
                                line_to(p.x, p.y);
                            }
                            line_to(points[0].x, points[0].y);

                            fill();
                        }
                        _ => {
                            log(&format!("Unexpected shape type: {:?}", shape.shape_type()));
                        }
                    }
                    reset_transform();
                }
            }

            for pointer in player_pointers.values() {
                if pointer.render {
                    let p = pointer.cursor_position / WORLD_SCALE_FACTOR;
                    let (radius, alpha) = if pointer.cursor_down {
                        (0.02, 255)
                    } else {
                        (0.03, 150)
                    };
                    set_color(pointer.color.0, pointer.color.1, pointer.color.2, alpha);
                    draw_circle(p.x, p.y, radius / WORLD_SCALE_FACTOR);
                }
            }
        }
        Event::PlayerJoined { player } => {
            log(&format!("Player Joined: {:?}", player));
            player_colors.insert(
                player,
                COLORS[random.rand_range(0..COLORS.len() as _) as usize],
            );
        }
        Event::PlayerLeft { player } => {
            log(&format!("Player left: {:?}", player));
            player_colors.remove(&player);

            let mut to_remove = Vec::new();
            for key in player_pointers.keys() {
                if key.0 == player {
                    to_remove.push(*key);
                }
            }
            for key in to_remove {
                player_pointers.remove(&key);
            }
        }
        Event::PointerMove {
            player,
            pointer_id,
            x,
            y,
        } => {
            if let Some(player_color) = player_colors.get(&player) {
                let entry = player_pointers.entry((player, pointer_id));
                let pointer = entry.or_insert_with(|| PlayerPointerInfo::new(*player_color));

                let world_position = Vec2::new(x, y) * WORLD_SCALE_FACTOR;
                pointer.last_cursor_position = pointer.cursor_position;
                pointer.cursor_position = world_position;
            }
        }
        Event::PointerDown {
            player,
            pointer_id,
            x,
            y,
        } => {
            if let Some(player_color) = player_colors.get(&player) {
                let entry = player_pointers.entry((player, pointer_id));
                let pointer = entry.or_insert_with(|| PlayerPointerInfo::new(*player_color));

                pointer.render = true;
                pointer.cursor_down = true;
                let world_position = Vec2::new(x, y) * WORLD_SCALE_FACTOR;
                pointer.cursor_position = world_position;
                if let Some((collider_handle, position)) = rapier.query_pipeline.project_point(
                    &rapier.rigid_body_set,
                    &rapier.collider_set,
                    &[world_position.x, world_position.y].into(),
                    true,
                    QueryFilter::only_dynamic(),
                ) {
                    let collider = rapier.collider_set.get(collider_handle).unwrap();
                    let rigid_body = rapier
                        .rigid_body_set
                        .get_mut(collider.parent().unwrap())
                        .unwrap();

                    let rigid_body_position = rigid_body.translation();
                    if position.is_inside {
                        pointer.moving_collider = Some(collider_handle);
                        pointer.offset = Vec2::new(rigid_body_position.x, rigid_body_position.y)
                            - world_position;
                        rigid_body.set_gravity_scale(0.0, true);
                        rigid_body.set_angvel(0.0, false);
                        rigid_body.set_angular_damping(0.99);
                    }
                }
            }
        }
        Event::PointerUp {
            player,
            pointer_id,
            is_mouse,
            x: _,
            y: _,
        } => {
            if let Some(player_color) = player_colors.get(&player) {
                let entry = player_pointers.entry((player, pointer_id));
                let pointer = entry.or_insert_with(|| PlayerPointerInfo::new(*player_color));

                // Do not render touch or stylus events that are no longer occurring.
                pointer.render = is_mouse;
                pointer.cursor_down = false;
                if let Some(collider) = pointer.moving_collider {
                    let collider = rapier.collider_set.get(collider).unwrap();
                    let rigid_body = rapier
                        .rigid_body_set
                        .get_mut(collider.parent().unwrap())
                        .unwrap();

                    let velocity = (pointer.cursor_position - pointer.last_cursor_position) * 30.0;
                    rigid_body.set_linvel([velocity.x, velocity.y].into(), true);
                    rigid_body.set_gravity_scale(1.0, true);
                    rigid_body.set_angular_damping(0.2);
                    pointer.moving_collider = None;
                }
            }
        }
    });
}

fn add_ball(
    rapier: &mut RapierIntegration,
    random: &mut oorandom::Rand32,
    x: f32,
    y: f32,
    radius: f32,
) -> PhysicsObject {
    let rigid_body = RigidBodyBuilder::dynamic()
        .translation(vector![x as _, y as _])
        .linear_damping(1.4)
        .angular_damping(0.3)
        .build();

    let collider = ColliderBuilder::ball(radius).restitution(0.4).build();
    let ball_body_handle = rapier.rigid_body_set.insert(rigid_body);
    rapier
        .collider_set
        .insert_with_parent(collider, ball_body_handle, &mut rapier.rigid_body_set);
    PhysicsObject {
        rigid_body_handle: ball_body_handle,
        color: COLORS[random.rand_range(0..COLORS.len() as _) as usize],
    }
}

fn add_shape(
    rapier: &mut RapierIntegration,
    random: &mut oorandom::Rand32,
    x: f32,
    y: f32,
    sides: u8,
    side_size: f32,
) -> PhysicsObject {
    let mut points = Vec::new();

    for i in 0..sides {
        let angle = (i as f32 / sides as f32) * std::f32::consts::TAU;
        let (sin, cos) = angle.sin_cos();
        points.push(Vec2::new(sin, cos) * side_size);
    }

    PhysicsObject {
        rigid_body_handle: add_convex_hull(rapier, x, y, &points),
        color: COLORS[random.rand_range(0..COLORS.len() as _) as usize],
    }
}

fn add_convex_hull(
    rapier: &mut RapierIntegration,
    x: f32,
    y: f32,
    points: &[Vec2],
) -> RigidBodyHandle {
    let rigid_body = RigidBodyBuilder::dynamic()
        .translation(vector![x, y])
        .linear_damping(1.4)
        .angular_damping(0.3)
        .build();

    let points: Vec<Point2<Real>> = points.iter().map(|v| [v.x, v.y].into()).collect();
    let collider = ColliderBuilder::convex_hull(&points)
        .unwrap()
        .restitution(0.4)
        .build();
    let body_handle = rapier.rigid_body_set.insert(rigid_body);
    rapier
        .collider_set
        .insert_with_parent(collider, body_handle, &mut rapier.rigid_body_set);
    body_handle
}

fn add_rectangle(
    rapier: &mut RapierIntegration,
    random: &mut oorandom::Rand32,
    x: f32,
    y: f32,
    half_width: f32,
    half_height: f32,
    kinematic: bool,
    color: Option<(u8, u8, u8)>,
) -> PhysicsObject {
    let builder = if kinematic {
        RigidBodyBuilder::kinematic_position_based()
    } else {
        RigidBodyBuilder::dynamic()
    };

    let rigid_body = builder
        .translation(vector![x, y])
        .linear_damping(1.4)
        .angular_damping(0.3)
        .build();

    let collider = ColliderBuilder::cuboid(half_width, half_height)
        .restitution(0.5)
        .build();
    let body_handle = rapier.rigid_body_set.insert(rigid_body);
    rapier
        .collider_set
        .insert_with_parent(collider, body_handle, &mut rapier.rigid_body_set);

    PhysicsObject {
        rigid_body_handle: body_handle,
        color: color.unwrap_or_else(|| COLORS[random.rand_range(0..COLORS.len() as _) as usize]),
    }
}

pub struct RapierIntegration {
    gravity: Vec2,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    query_pipeline: QueryPipeline,
}

impl RapierIntegration {
    pub fn new() -> Self {
        Self {
            gravity: Vec2::new(0.0, 9.81),
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            query_pipeline: QueryPipeline::new(),
        }
    }

    pub fn remove(&mut self, rigid_body_handle: RigidBodyHandle) {
        self.rigid_body_set.remove(
            rigid_body_handle,
            &mut self.island_manager,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            true,
        );
    }

    pub fn step(&mut self) {
        let gravity: [f32; 2] = self.gravity.into();
        let gravity = gravity.into();
        self.physics_pipeline.step(
            &gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            &(),
            &(),
        );

        /*
        for (_, (transform, rigid_body)) in
            world.query::<(&mut Transform, &RapierRigidBody)>().iter()
        {
            let body = &self.rigid_body_set[rigid_body.rigid_body_handle];
            let p: [f32; 2] = body.position().translation.into();
            let r: f32 = body.rotation().angle();
            transform.position = Vec3::new(p[0], p[1], transform.position.z);
            transform.rotation = Quat::from_angle_axis(r, Vec3::Z);
        }
        */

        self.query_pipeline.update(
            &self.island_manager,
            &self.rigid_body_set,
            &self.collider_set,
        );
    }
}
