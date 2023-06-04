use bevy::{prelude::*, reflect::TypeUuid};
use ra2_data::vxl::{Section, SectionOrigin, Span, Voxel, VxlFileOrigin};

#[derive(Clone, Debug, TypeUuid, Default)]
#[uuid = "d814dd88-474e-429d-b4d4-de4d9d1dec6e"]
pub struct VxlFile {
    // pub name:     String,
    pub sections: Vec<Section>
}

impl VxlFile {
    pub fn deal(origin: VxlFileOrigin) -> Self {
        let mut vxl: VxlFile = VxlFile {
            // name,
            sections: Vec::with_capacity(origin.sections.len())
        };
        origin.sections.into_iter().for_each(|val| {
            let SectionOrigin {
                name,
                hva_multiplier,
                max_bounds,
                min_bounds,
                normals_mode,
                size,
                spans,
                transf_matrix
            } = val;

            let max_bounds = Vec3::from(max_bounds);
            let min_bounds = Vec3::from(min_bounds);
            let [size_x, size_y, size_z] = size;
            let [a0, a1, a2] = transf_matrix;
            let cols: [[f32; 4]; 4] = [a0, a1, a2, [0f32; 4]];
            Mat4::from_cols_array_2d(&cols);
            let mut section = Section {
                name,
                hva_multiplier,
                max_bounds,
                min_bounds,
                size_x,
                size_y,
                size_z,
                normals_mode,
                transf_matrix: Default::default(),
                spans: Vec::with_capacity(spans.len())
            };
            spans.iter().for_each(|x| {
                let r = x.split(" ").collect::<Vec<&str>>();
                let r0 = r[0].parse::<f32>().unwrap();
                let r1 = r[1].parse::<f32>().unwrap();
                let mut n = Span {
                    x:      r0,
                    y:      r1,
                    voxels: Vec::with_capacity(5)
                };
                if r.len() > 2 && r[2] != "" {
                    let a = r[2].split(";").collect::<Vec<&str>>();
                    a.iter().for_each(|y| {
                        let o = y.split(",").collect::<Vec<&str>>();
                        n.voxels.push(Voxel {
                            x:            r0,
                            y:            r1,
                            z:            o[0].parse::<f32>().unwrap(),
                            color_index:  o[1].parse::<usize>().unwrap(),
                            normal_index: o[2].parse::<usize>().unwrap()
                        })
                    });
                }
                section.spans.push(n);
            });
            vxl.sections.push(section);
        });
        vxl
    }

    pub fn get_section(&self, index: usize) -> &Section {
        &self.sections[index]
    }
}
