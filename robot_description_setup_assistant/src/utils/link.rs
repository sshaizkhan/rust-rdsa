use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::{Reader, Writer};
use std::io::Cursor;
use std::option::Option;

use std::error::Error;

use super::cylinder_inertia::CylinderInertia;
use super::data_classes::elements::Origin;

#[derive(Debug, Clone)]
pub struct Link {
    pub link_name: String,
    pub visual_origin: Option<Origin>,
    pub visual_mesh_filename: Option<String>,
    pub material_name: Option<String>,
    pub material_color: Option<String>,
    pub collision_origin: Option<Origin>,
    pub collision_mesh: Option<String>,
    pub inertial_origin: Option<Origin>,
    pub inertial_radius: Option<f64>,
    pub inertial_length: Option<f64>,
    pub inertial_mass: Option<f64>,
    pub geometry_type: Option<String>,
    pub geometry_dimensions: Vec<String>,
}

impl Default for Link {
    fn default() -> Self {
        Self {
            link_name: Default::default(),
            visual_origin: None,
            visual_mesh_filename: None,
            material_name: None,
            material_color: None,
            collision_origin: None,
            collision_mesh: None,
            inertial_origin: None,
            inertial_radius: None,
            inertial_length: None,
            inertial_mass: None,
            geometry_type: Some(String::from("mesh")),
            geometry_dimensions: Vec::new(),
        }
    }
}

impl Link {
    pub fn to_xml(&self) -> quick_xml::Result<Vec<u8>> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        // Start `link` tag
        let mut link = BytesStart::owned(b"link".to_vec(), "link".len());
        link.push_attribute(("name", self.link_name.as_ref()));
        writer.write_event(Event::Start(link))?;

        // Handle visual element
        if let Some(origin) = &self.visual_origin {
            let visual = BytesStart::owned(b"visual".to_vec(), "visual".len());
            writer.write_event(Event::Start(visual))?;

            let mut origin_element = BytesStart::owned(b"origin".to_vec(), "origin".len());
            origin_element.push_attribute((
                "xyz",
                format!("{} {} {}", origin.xyz.0, origin.xyz.1, origin.xyz.2).as_str(),
            ));
            origin_element.push_attribute((
                "rpy",
                format!("{} {} {}", origin.rpy.0, origin.rpy.1, origin.rpy.2).as_str(),
            ));
            writer.write_event(Event::Empty(origin_element))?;

            let geometry_visual = BytesStart::owned(b"geometry".to_vec(), "geometry".len());
            writer.write_event(Event::Start(geometry_visual))?;

            match self.geometry_type.as_ref().unwrap().as_str() {
                "mesh" => {
                    if let Some(mesh) = &self.visual_mesh_filename {
                        let mut mesh_element = BytesStart::owned(b"mesh".to_vec(), "mesh".len());
                        mesh_element.push_attribute(("filename", mesh.as_str()));
                        writer.write_event(Event::Empty(mesh_element))?;
                    }
                }
                "cylinder" => {
                    // Handle cylinder geometry...
                    if let (Some(radius), Some(length)) = (
                        &self.geometry_dimensions.get(0),
                        &self.geometry_dimensions.get(1),
                    ) {
                        let mut cylinder_element =
                            BytesStart::owned(b"cylinder".to_vec(), "cylinder".len());
                        cylinder_element.push_attribute(("radius", radius.as_str()));
                        cylinder_element.push_attribute(("length", length.as_str()));
                        writer.write_event(Event::Empty(cylinder_element))?;
                    }
                }
                "box" => {
                    // Handle box geometry...
                    if let (Some(x), Some(y), Some(z)) = (
                        &self.geometry_dimensions.get(0),
                        &self.geometry_dimensions.get(1),
                        &self.geometry_dimensions.get(2),
                    ) {
                        let mut box_element = BytesStart::owned(b"box".to_vec(), "box".len());
                        box_element.push_attribute(("size", format!("{} {} {}", x, y, z).as_str()));
                        writer.write_event(Event::Empty(box_element))?;
                    }
                }
                "sphere" => {
                    // Handle sphere geometry...
                    if let Some(radius) = &self.geometry_dimensions.get(0) {
                        let mut sphere_element =
                            BytesStart::owned(b"sphere".to_vec(), "sphere".len());
                        sphere_element.push_attribute(("radius", radius.as_str()));
                        writer.write_event(Event::Empty(sphere_element))?;
                    }
                }
                _ => {}
            }
            writer.write_event(Event::End(BytesEnd::borrowed(b"geometry")))?;

            // Material element
            if let (Some(name), Some(color)) = (&self.material_name, &self.material_color) {
                let mut material = BytesStart::owned(b"material".to_vec(), "material".len());
                material.push_attribute(("name", name.as_str()));
                writer.write_event(Event::Start(material))?;

                let mut color_element = BytesStart::owned(b"color".to_vec(), "color".len());
                color_element.push_attribute(("rgba", color.as_str()));
                writer.write_event(Event::Empty(color_element))?;

                writer.write_event(Event::End(BytesEnd::borrowed(b"material")))?;
            }

            writer.write_event(Event::End(BytesEnd::borrowed(b"visual")))?;
        }

        if let Some(collision_origin) = &self.collision_origin {
            let collision = BytesStart::owned(b"collision".to_vec(), "collision".len());
            writer.write_event(Event::Start(collision))?;

            let mut origin_element = BytesStart::owned(b"origin".to_vec(), "origin".len());
            origin_element.push_attribute((
                "xyz",
                format!(
                    "{} {} {}",
                    collision_origin.xyz.0, collision_origin.xyz.1, collision_origin.xyz.2
                )
                .as_str(),
            ));
            origin_element.push_attribute((
                "rpy",
                format!(
                    "{} {} {}",
                    collision_origin.rpy.0, collision_origin.rpy.1, collision_origin.rpy.2
                )
                .as_str(),
            ));
            writer.write_event(Event::Empty(origin_element))?;

            let geometry_collision = BytesStart::owned(b"geometry".to_vec(), "geometry".len());
            writer.write_event(Event::Start(geometry_collision))?;

            match self.geometry_type.as_ref().unwrap().as_str() {
                "mesh" => {
                    if let Some(mesh) = &self.collision_mesh {
                        let mut mesh_element = BytesStart::owned(b"mesh".to_vec(), "mesh".len());
                        mesh_element.push_attribute(("filename", mesh.as_str()));
                        writer.write_event(Event::Empty(mesh_element))?;
                    }
                }
                "cylinder" => {
                    // Handle cylinder geometry...
                    if let (Some(radius), Some(length)) = (
                        &self.geometry_dimensions.get(0),
                        &self.geometry_dimensions.get(1),
                    ) {
                        let mut cylinder_element =
                            BytesStart::owned(b"cylinder".to_vec(), "cylinder".len());
                        cylinder_element.push_attribute(("radius", radius.as_str()));
                        cylinder_element.push_attribute(("length", length.as_str()));
                        writer.write_event(Event::Empty(cylinder_element))?;
                    }
                }
                "box" => {
                    // Handle box geometry...
                    if let (Some(x), Some(y), Some(z)) = (
                        &self.geometry_dimensions.get(0),
                        &self.geometry_dimensions.get(1),
                        &self.geometry_dimensions.get(2),
                    ) {
                        let mut box_element = BytesStart::owned(b"box".to_vec(), "box".len());
                        box_element.push_attribute(("size", format!("{} {} {}", x, y, z).as_str()));
                        writer.write_event(Event::Empty(box_element))?;
                    }
                }
                "sphere" => {
                    // Handle sphere geometry...
                    if let Some(radius) = &self.geometry_dimensions.get(0) {
                        let mut sphere_element =
                            BytesStart::owned(b"sphere".to_vec(), "sphere".len());
                        sphere_element.push_attribute(("radius", radius.as_str()));
                        writer.write_event(Event::Empty(sphere_element))?;
                    }
                }
                _ => {}
            }
            writer.write_event(Event::End(BytesEnd::borrowed(b"geometry")))?;
        }
        if let Some(inertial_origin) = &self.inertial_origin {
            let cylinder_inertia = CylinderInertia::new(
                self.inertial_radius.unwrap(),
                self.inertial_length.unwrap(),
                self.inertial_mass.unwrap(),
                inertial_origin.clone(),
            );

            let inertia_xml = cylinder_inertia.to_xml()?;
            // append the inertia_xml to the writer
            writer.write(&inertia_xml)?;
        }

        // End `link` tag
        writer.write_event(Event::End(BytesEnd::borrowed(b"link")))?;

        Ok(writer.into_inner().into_inner())
    }

    pub fn pretty_print_xml(input_xml: &str) -> Result<String, Box<dyn Error>> {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_to_xml() {
        let mut link = Link::default();
        link.link_name = "link1".to_string();
        link.visual_origin = Some(Origin {
            xyz: (0.0, 0.0, 0.0),
            rpy: (0.0, 0.0, 0.0),
        });
        link.visual_mesh_filename = Some("package://package_name/meshes/mesh.stl".to_string());
        link.material_name = Some("material_name".to_string());
        link.material_color = Some("0.1 0.2 0.3 0.4".to_string());
        link.collision_origin = Some(Origin {
            xyz: (0.0, 0.0, 0.0),
            rpy: (0.0, 0.0, 0.0),
        });
        link.collision_mesh = Some("package://package_name/meshes/mesh.stl".to_string());
        link.inertial_origin = Some(Origin {
            xyz: (0.0, 0.0, 0.0),
            rpy: (0.0, 0.0, 0.0),
        });
        link.inertial_radius = Some(0.1);
        link.inertial_length = Some(0.2);
        link.inertial_mass = Some(1.0);
        link.geometry_type = Some("mesh".to_string());
        link.geometry_dimensions = vec!["0.1".to_string(), "0.2".to_string()];

        let link_xml = link.to_xml().unwrap();
        let link_xml_str = String::from_utf8(link_xml).unwrap();

        match Link::pretty_print_xml(&link_xml_str) {
            Ok(pretty_xml) => println!("{}", pretty_xml),
            Err(e) => eprintln!("Error: {}", e),
        }

        let expected_xml = r#"<link name="link1">
<visual>
<origin xyz="0 0 0" rpy="0 0 0"/>
<geometry>
<mesh filename="package://package_name/meshes/mesh.stl"/>
</geometry>
<material name="material_name">
<color rgba="0.1 0.2 0.3 0.4"/>
</material>
</visual>
<collision>
<origin xyz="0 0 0" rpy="0 0 0"/>
<geometry>
<mesh filename="package://package_name/meshes/mesh.stl"/>
</geometry>
</collision>
<inertial>
<mass value="1.000000000"/>
<origin xyz="0 0 0" rpy="0 0 0"/>
<inertia ixx="0.005833331" ixy="0.000000000" ixz="0.000000000" iyy="0.005833331" iyz="0.000000000" izz="0.005000000"/>
</inertial>
</link>"#;
        assert_eq!(link_xml_str, expected_xml);
    }
}
