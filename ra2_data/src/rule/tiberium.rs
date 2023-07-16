use crate::ini::IniSection;
use std::sync::Arc;

#[derive(Debug)]
pub struct TiberiumRule {
    value:             i32,
    color:             String,
    // 碎片，残骸
    debris:            Vec<String>,
    growth:            i32,
    growth_percentage: f32,
    image:             usize,
    name:              String,
    power:             i32,
    /// 传播？
    spread:            i32,
    /// 传播百分比？
    spread_percentage: f32 /* Cruentus
                            * "Color": "NeonBlue",
                            * "Debris": "CRYSTAL1,CRYSTAL2,CRYSTAL3,CRYSTAL4",
                            * "Growth": "10000",
                            * "GrowthPercentage": "0",
                            * "Image": "2",
                            * "Name": "Tiberium Cruentus",
                            * "Power": "0",
                            * "Spread": "10000",
                            * "SpreadPercentage": "0",
                            * "Value": "50" */
}
impl TiberiumRule {
    pub fn new(e: Arc<IniSection>) -> Self {
        Self {
            value:             e.get_number_i32("Value"),
            color:             e.get_string("Color"),
            debris:            e.get_array_with_default("Debris", Some(","), None),
            growth:            e.get_number_i32("Growth"),
            growth_percentage: e.get_number("GrowthPercentage"),
            image:             e.get_number_i32("Image") as usize,
            name:              e.get_string("Name"),
            power:             e.get_number_i32("Power"),
            spread:            e.get_number_i32("Spread"),
            spread_percentage: e.get_number("SpreadPercentage")
        }
    }
}
