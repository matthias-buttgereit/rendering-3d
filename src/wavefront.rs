use nalgebra::Vector3;
use nom::{character::complete::multispace0, number::complete, IResult};

type ThreeVectors<T> = (Vector3<T>, Vector3<T>, Vector3<T>);

#[derive(Default, Debug)]
pub struct WavefrontObject {
    faces: Vec<Face>,
}

impl WavefrontObject {
    pub fn new() -> Self {
        Self { faces: vec![] }
    }

    pub fn parse_obj_file(text: &str) -> WavefrontObject {
        let input = text;

        let mut vertices = Vec::new();
        let mut texture = Vec::new();
        let mut normals = Vec::new();

        for line in input.lines() {
            if let Ok((_, vertex)) = Self::parse_vertex(line, "v") {
                vertices.push(vertex);
            } else if let Ok((_, texture_coordinate)) = Self::parse_vertex(line, "vt") {
                texture.push(texture_coordinate);
            } else if let Ok((_, vertex_normal)) = Self::parse_vertex(line, "vn") {
                normals.push(vertex_normal);
            }
        }

        let input = text;
        let mut faces: Vec<Face> = vec![];

        for line in input.lines() {
            if let Ok((_, index)) = Self::parse_face_indices(line) {
                let vertices: ThreeVectors<f32> = (
                    *vertices.get(index[0]).unwrap(),
                    *vertices.get(index[1]).unwrap(),
                    *vertices.get(index[2]).unwrap(),
                );

                let texture = (
                    *texture.get(index[3]).unwrap(),
                    *texture.get(index[4]).unwrap(),
                    *texture.get(index[5]).unwrap(),
                );

                let normals = (
                    *normals.get(index[6]).unwrap(),
                    *normals.get(index[7]).unwrap(),
                    *normals.get(index[8]).unwrap(),
                );
                faces.push(Face::new(vertices, texture, normals));
            }
        }

        Self { faces }
    }

    fn parse_vertex<'a>(input: &'a str, starting_tag: &str) -> IResult<&'a str, Vector3<f32>> {
        let (input, _) = nom::bytes::complete::tag(starting_tag)(input)?;
        let (input, _) = multispace0(input)?;
        let (input, x) = complete::float(input)?;
        let (input, _) = multispace0(input)?;
        let (input, y) = complete::float(input)?;
        let (input, _) = multispace0(input)?;
        let (input, z) = complete::float(input)?;

        Ok((input, Vector3::new(x, y, z)))
    }

    fn parse_face_indices(input: &str) -> IResult<&str, [usize; 9]> {
        let mut result = [0; 9];

        let (input, _) = nom::character::complete::char('f')(input)?;

        let (input, _) = multispace0(input)?;
        let (input, index) = nom::character::complete::u16(input)?;
        result[0] = (index - 1) as usize;

        let (input, _) = nom::character::complete::char('/')(input)?;
        let (input, index) = nom::character::complete::u16(input)?;
        result[3] = (index - 1) as usize;

        let (input, _) = nom::character::complete::char('/')(input)?;
        let (input, index) = nom::character::complete::u16(input)?;
        result[6] = (index - 1) as usize;

        let (input, _) = multispace0(input)?;
        let (input, index) = nom::character::complete::u16(input)?;
        result[1] = (index - 1) as usize;

        let (input, _) = nom::character::complete::char('/')(input)?;
        let (input, index) = nom::character::complete::u16(input)?;
        result[4] = (index - 1) as usize;

        let (input, _) = nom::character::complete::char('/')(input)?;
        let (input, index) = nom::character::complete::u16(input)?;
        result[7] = (index - 1) as usize;

        let (input, _) = multispace0(input)?;
        let (input, index) = nom::character::complete::u16(input)?;
        result[2] = (index - 1) as usize;

        let (input, _) = nom::character::complete::char('/')(input)?;
        let (input, index) = nom::character::complete::u16(input)?;
        result[5] = (index - 1) as usize;

        let (input, _) = nom::character::complete::char('/')(input)?;
        let (input, index) = nom::character::complete::u16(input)?;
        result[8] = (index - 1) as usize;

        Ok((input, result))
    }

    pub fn get_face(&self, index: usize) -> Option<&Face> {
        self.faces.get(index)
    }

    pub fn faces(&self) -> &Vec<Face> {
        &self.faces
    }
}

#[derive(Default, Debug)]
pub struct Face {
    vertices: ThreeVectors<f32>,
    texture: ThreeVectors<f32>,
    normals: ThreeVectors<f32>,
}

impl Face {
    pub fn new(
        vertices: ThreeVectors<f32>,
        texture: ThreeVectors<f32>,
        normals: ThreeVectors<f32>,
    ) -> Self {
        Self {
            vertices,
            texture,
            normals,
        }
    }

    pub fn vertices(&self) -> &ThreeVectors<f32> {
        &self.vertices
    }

    pub fn texture(&self) -> &ThreeVectors<f32> {
        &self.texture
    }

    pub fn normals(&self) -> &ThreeVectors<f32> {
        &self.normals
    }
}
