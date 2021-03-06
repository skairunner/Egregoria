use crate::geometry::Tesselator;
use egregoria::physics::Transform;
use egregoria::rendering::meshrender_component::{MeshRender, MeshRenderEnum};
use egregoria::specs::{Join, World, WorldExt};

pub struct MeshRenderer;

impl MeshRenderer {
    pub fn render(world: &mut World, tess: &mut Tesselator) {
        let transforms = world.read_component::<Transform>();
        let mesh_render = world.read_component::<MeshRender>();

        for (trans, mr) in (&transforms, &mesh_render).join() {
            if mr.hide {
                continue;
            }
            for order in &mr.orders {
                match order {
                    MeshRenderEnum::StrokeCircle(x) => {
                        tess.color = x.color.into();
                        tess.draw_stroke_circle(
                            trans.project(x.offset),
                            mr.z,
                            x.radius,
                            x.thickness,
                        );
                    }
                    MeshRenderEnum::Circle(x) => {
                        tess.color = x.color.into();
                        tess.draw_circle(trans.project(x.offset), mr.z, x.radius);
                    }
                    MeshRenderEnum::Rect(x) => {
                        tess.color = x.color.into();
                        let rect_pos = trans.position() + trans.apply_rotation(x.offset);
                        tess.draw_rect_cos_sin(
                            rect_pos,
                            mr.z,
                            x.width,
                            x.height,
                            trans.direction(),
                        );
                    }
                    MeshRenderEnum::LineTo(x) => {
                        tess.color = x.color.into();
                        let e = x.to;
                        if let Some(trans2) = transforms.get(e) {
                            tess.draw_stroke(
                                trans.position(),
                                trans2.position(),
                                mr.z,
                                x.thickness,
                            );
                        }
                    }
                    MeshRenderEnum::Line(x) => {
                        tess.color = x.color.into();
                        let start = trans.position();
                        let end = start + x.offset;
                        tess.draw_stroke(start, end, mr.z, x.thickness);
                    }

                    MeshRenderEnum::AbsoluteLine(x) => {
                        tess.color = x.color.into();
                        tess.draw_stroke(x.src, x.dst, mr.z, x.thickness);
                    }
                }
            }
        }
    }
}
