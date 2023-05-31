use asset::Palette;
use ra2_data::vxl::VxlFile;

pub mod asset;
pub mod loader;

pub struct VxlBuilder {
    pub vxl_file: VxlFile,
    pub palette:  Palette
}
impl VxlBuilder {
    // todo
    // pub fn create_vxl_meshes(&self) {
    //     let n = TextureUtils::texture_from_palette(&self.palette);

    //     let a = &self.vxl_file.sections;
    //     a.iter().for_each(|x| {});
    // }

    // /// geometry 几何结构
    // fn create_vxl_geometry(e: &Section) {
    //     let t = e.get_all_voxels();
    // }
}
