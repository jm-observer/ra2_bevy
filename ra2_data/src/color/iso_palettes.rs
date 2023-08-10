use crate::{color::RaColor, lighting::Lighting};
use bevy::reflect::TypeUuid;

#[derive(Clone, Debug, TypeUuid, Default)]
#[uuid = "3c047c2f-2173-44f5-bdfd-c3087c8d89e2"]
pub struct IsoPalettes {
    //调色板
    pub palettes: [IsoPalette; 19]
}

#[derive(Clone, Debug, TypeUuid, Copy)]
#[uuid = "3c047c2f-2173-44f5-bdfd-c3087c8d89e2"]
pub struct IsoPalette {
    //调色板
    pub colors: [RaColor; 256] //[Color; 256],
}

impl Default for IsoPalette {
    fn default() -> Self {
        Self {
            colors: [RaColor::default(); 256]
        }
    }
}

impl IsoPalettes {
    pub fn new(val: &[u8], l: &Lighting) -> Self {
        let mut palettes: [IsoPalette; 19] = [IsoPalette::default(); 19];
        for level in 0..19 {
            let mut ambient_mult = l.ambient - l.ground + l.level * (level as f64);
            let mut red_mult = 1.0;
            let mut green_mult = 1.0;
            let mut blue_mult = 1.0;
            if l.force_tint {
                red_mult = l.red;
                green_mult = l.green;
                blue_mult = l.blue;
            }
            ambient_mult = fit_number(ambient_mult);
            red_mult = fit_number(red_mult);
            green_mult = fit_number(green_mult);
            blue_mult = fit_number(blue_mult);
            let colors = &mut palettes[level as usize].colors;
            for i in 0..256 {
                let rmult = ambient_mult * red_mult;
                let gmult = ambient_mult * green_mult;
                let bmult = ambient_mult * blue_mult;
                let r = (val[i * 3 + 0] as f64 * rmult / 63.0 * 255.0) as u8;
                let g = (val[i * 3 + 1] as f64 * gmult / 63.0 * 255.0) as u8;
                let b = (val[i * 3 + 2] as f64 * bmult / 63.0 * 255.0) as u8;
                colors[i] = RaColor::from_rgb(r, g, b);
            }
        }
        todo!()
        // let len = val.len() / 3 as usize;
        // let mut colors = Vec::<RaColor>::with_capacity(256);
        // let mut index = 0;
        // while index < len {
        //     colors.push(RaColor::from_rgb(
        //         4 * val[3 * index],
        //         4 * val[3 * index + 1],
        //         4 * val[3 * index + 2]
        //     ));
        //     index += 1;
        // }
        // IsoPalettes { colors }
    }
}

fn fit_number(val: f64) -> f64 {
    if val < 0.0 {
        return 0.0;
    } else {
        val
    }
}
