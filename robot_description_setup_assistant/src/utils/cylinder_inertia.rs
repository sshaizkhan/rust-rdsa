use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::{Reader, Writer};
use std::io::Cursor;

use super::data_classes::elements::Origin;
use std::error::Error;

#[derive(Debug, Default)]
pub struct InertiaTensor {
    pub ixx: f64,
    pub ixy: f64,
    pub ixz: f64,
    pub iyy: f64,
    pub iyz: f64,
    pub izz: f64,
}

#[derive(Debug, Clone)]
pub struct CylinderInertia {
    pub inertial_radius: f64,
    pub inertial_length: f64,
    pub inertial_mass: f64,
    pub inertial_origin: Origin,
}

impl Default for CylinderInertia {
    fn default() -> Self {
        Self {
            inertial_radius: Default::default(),
            inertial_length: Default::default(),
            inertial_mass: Default::default(),
            inertial_origin: Default::default(),
        }
    }
}

impl CylinderInertia {
    pub fn calculate_inertia(&self) -> InertiaTensor {
        let ixx_iyy = 0.0833333
            * self.inertial_mass
            * (3.0 * self.inertial_radius.powi(2) + self.inertial_length.powi(2));
        let izz = 0.5 * self.inertial_mass * self.inertial_radius.powi(2);

        InertiaTensor {
            ixx: format!("{:.9}", ixx_iyy).parse().unwrap(),
            ixy: 0.0,
            ixz: 0.0,
            iyy: format!("{:.9}", ixx_iyy).parse().unwrap(),
            iyz: 0.0,
            izz: format!("{:.9}", izz).parse().unwrap(),
        }
    }

    fn pretty_print_xml(input_xml: &str) -> Result<String, Box<dyn Error>> {
        let mut reader = Reader::from_str(input_xml);
        reader.trim_text(true);

        let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 4);
        let mut buf = Vec::new();

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Eof) => break,
                Ok(e) => writer.write_event(&e)?,
                Err(e) => return Err(e.into()),
            }
            buf.clear();
        }

        let result = writer.into_inner().into_inner();
        Ok(String::from_utf8(result)?)
    }

    pub fn to_xml(&self) -> quick_xml::Result<Vec<u8>> {
        let inertia = self.calculate_inertia();
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        let inertial = BytesStart::owned(b"inertial".to_vec(), "inertial".len());
        writer.write_event(Event::Start(inertial))?;

        let mass_element = BytesStart::owned(b"mass".to_vec(), "mass".len());
        writer.write_event(Event::Empty(mass_element.with_attributes(vec![(
            "value",
            (&format!("{:.9}", self.inertial_mass)).as_str(),
        )])))?;
        // writer.write_event(Event::End(BytesEnd::borrowed(b"mass")))?;

        let origin_xyz = format!(
            "{} {} {}",
            self.inertial_origin.xyz.0, self.inertial_origin.xyz.1, self.inertial_origin.xyz.2
        );
        let origin_rpy = format!(
            "{} {} {}",
            self.inertial_origin.rpy.0, self.inertial_origin.rpy.1, self.inertial_origin.rpy.2
        );

        let origin_element = BytesStart::owned(b"origin".to_vec(), "origin".len());
        writer.write_event(Event::Empty(origin_element.with_attributes(vec![
            ("xyz", origin_xyz.as_str()),
            ("rpy", origin_rpy.as_str()),
        ])))?;

        let inertia_element = BytesStart::owned(b"inertia".to_vec(), "inertia".len());
        writer.write_event(Event::Empty(inertia_element.with_attributes(vec![
            ("ixx", (&format!("{:.9}", inertia.ixx)).as_str()),
            ("ixy", (&format!("{:.9}", inertia.ixy)).as_str()),
            ("ixz", (&format!("{:.9}", inertia.ixz)).as_str()),
            ("iyy", (&format!("{:.9}", inertia.iyy)).as_str()),
            ("iyz", (&format!("{:.9}", inertia.iyz)).as_str()),
            ("izz", (&format!("{:.9}", inertia.izz)).as_str()),
        ])))?;

        writer.write_event(Event::End(BytesEnd::borrowed(b"inertial")))?;

        Ok(writer.into_inner().into_inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cylinder_inertia() {
        let origin = Origin {
            xyz: (0.0, 0.0, 0.0),
            rpy: (0.0, 0.0, 0.0),
        };
        let mut cylinder = CylinderInertia::default();
        cylinder.inertial_radius = 0.1;
        cylinder.inertial_length = 0.2;
        cylinder.inertial_mass = 1.0;
        cylinder.inertial_origin = origin;

        let inertia = cylinder.calculate_inertia();

        assert_eq!(inertia.ixx, 0.005833331);
        assert_eq!(inertia.ixy, 0.0);
        assert_eq!(inertia.ixz, 0.0);
        assert_eq!(inertia.iyy, 0.005833331);
        assert_eq!(inertia.iyz, 0.0);
        assert_eq!(inertia.izz, 0.005);
    }

    #[test]
    fn test_cylinder_inertia_to_xml() {
        let origin = Origin {
            xyz: (0.0, 0.0, 0.0),
            rpy: (0.0, 0.0, 0.0),
        };
        let mut cylinder = CylinderInertia::default();
        cylinder.inertial_radius = 0.1;
        cylinder.inertial_length = 0.2;
        cylinder.inertial_mass = 1.0;
        cylinder.inertial_origin = origin;

        let xml = cylinder.to_xml().unwrap();

        let expected_xml = r#"<inertial><mass value="1.000000000"/><origin xyz="0 0 0" rpy="0 0 0"/><inertia ixx="0.005833331" ixy="0.000000000" ixz="0.000000000" iyy="0.005833331" iyz="0.000000000" izz="0.005000000"/></inertial>"#;
        assert_eq!(xml, expected_xml.as_bytes());
    }

    #[test]
    fn test_print_inertia_to_xml() {
        let origin = Origin {
            xyz: (0.0, 0.0, 0.0),
            rpy: (0.0, 0.0, 0.0),
        };
        let mut cylinder = CylinderInertia::default();
        cylinder.inertial_radius = 0.1;
        cylinder.inertial_length = 0.2;
        cylinder.inertial_mass = 1.0;
        cylinder.inertial_origin = origin;

        let xml = cylinder.to_xml().unwrap();

        let xml_string = String::from_utf8(xml).unwrap();

        match CylinderInertia::pretty_print_xml(&xml_string) {
            Ok(pretty_xml) => println!("{}", pretty_xml),
            Err(e) => eprintln!("Error: {}", e),
        }

        // pretty print
    }
}
