use ra2_data::lighting::Lighting;

pub fn mp02t2_lighting() -> Lighting {
    Lighting {
        level:      0.008,
        ambient:    0.85,
        red:        1.0,
        green:      1.0,
        blue:       1.10,
        ground:     0.0,
        force_tint: true
    }
}
