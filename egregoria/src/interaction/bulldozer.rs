use crate::engine_interaction::{MouseButton, MouseInfo};
use crate::interaction::Tool;
use crate::physics::Transform;
use crate::rendering::meshrender_component::{CircleRender, MeshRender};
use crate::rendering::Color;
use geom::Vec2;
use map_model::{Map, ProjectKind};
use specs::prelude::*;
use specs::shred::PanicHandler;

pub struct BulldozerSystem;

pub struct BulldozerResource {
    project: Entity,
}

impl BulldozerResource {
    pub fn new(world: &mut World) -> Self {
        let mut mr = MeshRender::simple(
            CircleRender {
                offset: Vec2::ZERO,
                radius: 2.0,
                color: Color::RED,
            },
            0.9,
        );
        mr.hide = true;

        let e = world
            .create_entity()
            .with(Transform::zero())
            .with(mr)
            .build();
        Self { project: e }
    }
}

#[derive(SystemData)]
pub struct BulldozerData<'a> {
    tool: Read<'a, Tool>,
    mouseinfo: Read<'a, MouseInfo>,
    map: Write<'a, Map>,
    self_r: Write<'a, BulldozerResource, PanicHandler>,
    mr: WriteStorage<'a, MeshRender>,
    transforms: WriteStorage<'a, Transform>,
}

impl<'a> System<'a> for BulldozerSystem {
    type SystemData = BulldozerData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        let mr = data.mr.get_mut(data.self_r.project).unwrap(); // unwrap ok: self_r.project was defined with a meshrender in new
        if !matches!(*data.tool, Tool::Bulldozer) {
            mr.hide = true;
            return;
        }
        mr.hide = false;

        let cur_proj = data.map.project(data.mouseinfo.unprojected);

        data.transforms
            .get_mut(data.self_r.project)
            .unwrap() // unwrap ok: self_r.project was defined with a transform in new
            .set_position(cur_proj.pos);

        if data.mouseinfo.just_pressed.contains(&MouseButton::Left) {
            let mut potentially_empty = Vec::new();
            info!("bulldozer {:?}", cur_proj);
            match cur_proj.kind {
                ProjectKind::Inter(id) => {
                    potentially_empty
                        .extend(data.map.intersections()[id].neighbors(data.map.roads()));
                    data.map.remove_intersection(id)
                }
                ProjectKind::Road(id) => {
                    let r = &data.map.roads()[id];

                    potentially_empty.push(r.src);
                    potentially_empty.push(r.dst);

                    data.map.remove_road(id);
                }
                ProjectKind::House(id) => {
                    data.map.remove_house(id);
                }
                ProjectKind::Ground => {}
            }

            for id in potentially_empty {
                if data.map.intersections()[id].roads.is_empty() {
                    data.map.remove_intersection(id);
                }
            }
        }
    }
}
