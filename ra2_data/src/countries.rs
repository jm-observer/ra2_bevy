pub struct Countries;
pub const NEUTRAL: &'static str = "Neutral";

impl Countries {
    pub fn get_contry_name(name: &String) -> &'static str {
        Self::get_contry_name_by_str(name.as_ref())
    }

    pub fn get_contry_name_by_str(name: &str) -> &'static str {
        match name {
            "Americans" => "Americans",
            "Alliance" => "Alliance",
            "Nod" => "Nod",
            "Neutral" => "Neutral",
            "Special" => "Special",
            "French" => "French",
            "Germans" => "Germans",
            "British" => "British",
            "Africans" => "Africans",
            "Arabs" => "Arabs",
            "Confederation" => "Confederation",
            "Russians" => "Russians",
            "GDI" => "GDI",
            _ => panic!()
        }
    }
}
