#[cfg(target_os = "none")]
use alloc::vec::Vec;

use nalgebra::{Vector2, Vector3};

use crate::{
    chunk::Chunk,
    constants::{BlockType, world::CHUNK_SIZE},
    world::World,
};

const CHUNK_SIZE_I: isize = CHUNK_SIZE as isize;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum QuadDir {
    Front = 1,
    Back = 2,
    Top = 3,
    Bottom = 4,
    Right = 5,
    Left = 6,
}

impl QuadDir {
    pub fn from_id(id: u8) -> Self {
        match id {
            1 => QuadDir::Front,
            2 => QuadDir::Back,
            3 => QuadDir::Top,
            4 => QuadDir::Bottom,
            5 => QuadDir::Right,
            6 => QuadDir::Left,
            _ => panic!("Unknown quad direction."),
        }
    }

    pub fn get_normal_vector(&self) -> Vector3<isize> {
        match *self {
            QuadDir::Front => Vector3::new(0, 0, -1),
            QuadDir::Back => Vector3::new(0, 0, 1),
            QuadDir::Top => Vector3::new(0, -1, 0),
            QuadDir::Bottom => Vector3::new(0, 1, 0),
            QuadDir::Right => Vector3::new(1, 0, 0),
            QuadDir::Left => Vector3::new(-1, 0, 0),
        }
    }
}

pub struct Quad {
    data: u16,
    texture_id: u8,
}

impl Quad {
    pub fn new(pos: Vector3<u16>, dir: QuadDir, texture_id: u8, light: u16) -> Self {
        // xxx yyy zzz ddd llll
        // tttttttt
        let x = pos.x;
        let y = pos.y;
        let z = pos.z;
        let dir = dir as u16;
        let data = x << 13 | y << 10 | z << 7 | dir << 4 | light;
        Quad { data, texture_id }
    }

    pub fn get_pos(&self) -> nalgebra::Vector3<u16> {
        let x = self.data >> 13; // Equivalent to (self.data & 0b1110000000000000) >> 13
        let y = (self.data & 0b0001110000000000) >> 10;
        let z = (self.data & 0b0000001110000000) >> 7;
        nalgebra::Vector3::new(x, y, z)
    }
    pub fn get_light_level(&self) -> u16 {
        self.data & 0b0000000000001111
    }

    pub fn get_dir(&self) -> QuadDir {
        let dir = (self.data & 0b0000000001110000) >> 4;
        QuadDir::from_id(dir as u8)
    }
}

impl Quad {
    pub fn get_triangles(&self, chunk_pos: Vector3<isize>) -> (Triangle, Triangle) {
        let pos = self.get_pos().map(|x| x as isize) + chunk_pos * CHUNK_SIZE_I;
        let light = self.get_light_level() as u8;
        match self.get_dir() {
            QuadDir::Front => (
                Triangle {
                    p3: Vector3::new(pos.x as f32, pos.y as f32, pos.z as f32),
                    p2: Vector3::new((pos.x + 1) as f32, pos.y as f32, pos.z as f32),
                    p1: Vector3::new((pos.x + 1) as f32, (pos.y + 1) as f32, pos.z as f32),
                    texture_id: self.texture_id,
                    light,
                },
                Triangle {
                    p1: Vector3::new(pos.x as f32, pos.y as f32, pos.z as f32),
                    p2: Vector3::new(pos.x as f32, (pos.y + 1) as f32, pos.z as f32),
                    p3: Vector3::new((pos.x + 1) as f32, (pos.y + 1) as f32, pos.z as f32),
                    texture_id: self.texture_id,
                    light,
                },
            ),
            QuadDir::Back => (
                Triangle {
                    p1: Vector3::new(pos.x as f32, pos.y as f32, (pos.z + 1) as f32),
                    p2: Vector3::new((pos.x + 1) as f32, pos.y as f32, (pos.z + 1) as f32),
                    p3: Vector3::new((pos.x + 1) as f32, (pos.y + 1) as f32, (pos.z + 1) as f32),
                    texture_id: self.texture_id,
                    light,
                },
                Triangle {
                    p3: Vector3::new(pos.x as f32, pos.y as f32, (pos.z + 1) as f32),
                    p2: Vector3::new(pos.x as f32, (pos.y + 1) as f32, (pos.z + 1) as f32),
                    p1: Vector3::new((pos.x + 1) as f32, (pos.y + 1) as f32, (pos.z + 1) as f32), // TODO sort points from p1 to p3
                    texture_id: self.texture_id,
                    light,
                },
            ),
            QuadDir::Top => (
                Triangle {
                    p3: Vector3::new(pos.x as f32, pos.y as f32, (pos.z + 1) as f32),
                    p2: Vector3::new((pos.x + 1) as f32, pos.y as f32, (pos.z + 1) as f32),
                    p1: Vector3::new((pos.x + 1) as f32, pos.y as f32, pos.z as f32),
                    texture_id: self.texture_id,
                    light,
                },
                Triangle {
                    p1: Vector3::new(pos.x as f32, pos.y as f32, (pos.z + 1) as f32),
                    p2: Vector3::new(pos.x as f32, pos.y as f32, pos.z as f32),
                    p3: Vector3::new((pos.x + 1) as f32, pos.y as f32, pos.z as f32),
                    texture_id: self.texture_id,
                    light,
                },
            ),
            QuadDir::Bottom => (
                Triangle {
                    p1: Vector3::new(pos.x as f32, (pos.y + 1) as f32, (pos.z + 1) as f32),
                    p2: Vector3::new((pos.x + 1) as f32, (pos.y + 1) as f32, (pos.z + 1) as f32),
                    p3: Vector3::new((pos.x + 1) as f32, (pos.y + 1) as f32, pos.z as f32),
                    texture_id: self.texture_id,
                    light,
                },
                Triangle {
                    p3: Vector3::new(pos.x as f32, (pos.y + 1) as f32, (pos.z + 1) as f32),
                    p2: Vector3::new(pos.x as f32, (pos.y + 1) as f32, pos.z as f32),
                    p1: Vector3::new((pos.x + 1) as f32, (pos.y + 1) as f32, pos.z as f32),
                    texture_id: self.texture_id,
                    light,
                },
            ),
            QuadDir::Right => (
                Triangle {
                    p1: Vector3::new((pos.x + 1) as f32, (pos.y + 1) as f32, pos.z as f32),
                    p2: Vector3::new((pos.x + 1) as f32, (pos.y + 1) as f32, (pos.z + 1) as f32),
                    p3: Vector3::new((pos.x + 1) as f32, pos.y as f32, pos.z as f32),
                    texture_id: self.texture_id,
                    light,
                },
                Triangle {
                    p3: Vector3::new((pos.x + 1) as f32, pos.y as f32, (pos.z + 1) as f32),
                    p2: Vector3::new((pos.x + 1) as f32, (pos.y + 1) as f32, (pos.z + 1) as f32),
                    p1: Vector3::new((pos.x + 1) as f32, pos.y as f32, pos.z as f32),
                    texture_id: self.texture_id,
                    light,
                },
            ),
            QuadDir::Left => (
                Triangle {
                    p3: Vector3::new(pos.x as f32, (pos.y + 1) as f32, pos.z as f32),
                    p2: Vector3::new(pos.x as f32, (pos.y + 1) as f32, (pos.z + 1) as f32),
                    p1: Vector3::new(pos.x as f32, pos.y as f32, pos.z as f32),
                    texture_id: self.texture_id,
                    light,
                },
                Triangle {
                    p1: Vector3::new(pos.x as f32, pos.y as f32, (pos.z + 1) as f32),
                    p2: Vector3::new(pos.x as f32, (pos.y + 1) as f32, (pos.z + 1) as f32),
                    p3: Vector3::new(pos.x as f32, pos.y as f32, pos.z as f32),
                    texture_id: self.texture_id,
                    light,
                },
            ),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub p1: Vector3<f32>,
    pub p2: Vector3<f32>,
    pub p3: Vector3<f32>,
    pub texture_id: u8,
    pub light: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle2D {
    pub p1: Vector2<i16>,
    pub p2: Vector2<i16>,
    pub p3: Vector2<i16>,
    pub texture_id: u8,
    pub light: u8,
}

impl Triangle2D {
    pub fn to_small(&self) -> SmallTriangle2D {
        SmallTriangle2D {
            p1: self.p1.map(|x| x as u8),
            p2: self.p2.map(|x| x as u8),
            p3: self.p3.map(|x| x as u8),
            texture_id: self.texture_id,
            light: self.light,
        }
    }
}

pub struct SmallTriangle2D {
    pub p1: Vector2<u8>,
    pub p2: Vector2<u8>,
    pub p3: Vector2<u8>,
    pub texture_id: u8,
    pub light: u8,
}

impl SmallTriangle2D {
    pub fn to_tri_2d(&self) -> Triangle2D {
        Triangle2D {
            p1: self.p1.map(|x| x as i16),
            p2: self.p2.map(|x| x as i16),
            p3: self.p3.map(|x| x as i16),
            texture_id: self.texture_id,
            light: self.light,
        }
    }
}

impl Triangle {
    pub fn get_normal(&self) -> Vector3<f32> {
        let a = self.p2 - self.p1;
        let b = self.p3 - self.p1;
        a.cross(&b).normalize()
    }
}

fn get_block_in_chunk_or_world(
    pos: Vector3<isize>,
    world: &World,
    chunk: &Chunk,
) -> Option<BlockType> {
    if pos.x < 0
        || pos.x >= CHUNK_SIZE_I
        || pos.y < 0
        || pos.y >= CHUNK_SIZE_I
        || pos.z < 0
        || pos.z >= CHUNK_SIZE_I
    {
        world.get_block_in_world(pos + *chunk.get_pos() * CHUNK_SIZE_I)
    } else {
        Some(chunk.get_at_unchecked(pos))
    }
}

pub struct Mesh {
    pub quads: Vec<Quad>,
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}

impl Mesh {
    pub fn new() -> Self {
        Mesh { quads: Vec::new() }
    }

    pub fn get_reference_vec(&self) -> Vec<&Quad> {
        self.quads.iter().collect()
    }

    fn get_light_level_from_dir(dir: QuadDir) -> u16 { // Please not bellow 2 to avoid negative light. What is neagative light ?
        match dir {
            QuadDir::Front => 13,
            QuadDir::Back => 10,
            QuadDir::Top => 15,
            QuadDir::Bottom => 6,
            QuadDir::Right => 11,
            QuadDir::Left => 10,
        }
    }

    pub fn generate_chunk(world: &World, chunk: &Chunk) -> Self {
        let mut quads = Vec::new();

        for x in 0..CHUNK_SIZE as isize {
            for y in 0..CHUNK_SIZE as isize {
                for z in 0..CHUNK_SIZE as isize {
                    let block_type = chunk.get_at(Vector3::new(x, y, z)).unwrap();
                    if block_type != BlockType::Air {
                        let bloc_pos = Vector3::new(x as u16, y as u16, z as u16);

                        let grid_additional_light = if (x + y + z) % 2 == 0 { 2 } else { 0 }; // Make one block/2 darker to increase visibility

                        if get_block_in_chunk_or_world(Vector3::new(x, y, z - 1), world, chunk)
                            .is_some_and(|block| block.is_air())
                        {
                            quads.push(Quad::new(
                                bloc_pos,
                                QuadDir::Front,
                                block_type.get_texture_id(QuadDir::Front),
                                Mesh::get_light_level_from_dir(QuadDir::Front)
                                    - grid_additional_light,
                            ));
                        }

                        if get_block_in_chunk_or_world(Vector3::new(x, y, z + 1), world, chunk)
                            .is_some_and(|block| block.is_air())
                        {
                            quads.push(Quad::new(
                                bloc_pos,
                                QuadDir::Back,
                                block_type.get_texture_id(QuadDir::Back),
                                Mesh::get_light_level_from_dir(QuadDir::Back)
                                    - grid_additional_light,
                            ));
                        }

                        if get_block_in_chunk_or_world(Vector3::new(x + 1, y, z), world, chunk)
                            .is_some_and(|block| block.is_air())
                        {
                            quads.push(Quad::new(
                                bloc_pos,
                                QuadDir::Right,
                                block_type.get_texture_id(QuadDir::Right),
                                Mesh::get_light_level_from_dir(QuadDir::Right)
                                    - grid_additional_light,
                            ));
                        }
                        if get_block_in_chunk_or_world(Vector3::new(x - 1, y, z), world, chunk)
                            .is_some_and(|block| block.is_air())
                        {
                            quads.push(Quad::new(
                                bloc_pos,
                                QuadDir::Left,
                                block_type.get_texture_id(QuadDir::Left),
                                Mesh::get_light_level_from_dir(QuadDir::Left)
                                    - grid_additional_light,
                            ));
                        }

                        if get_block_in_chunk_or_world(Vector3::new(x, y - 1, z), world, chunk)
                            .is_some_and(|block| block.is_air())
                        {
                            quads.push(Quad::new(
                                bloc_pos,
                                QuadDir::Top,
                                block_type.get_texture_id(QuadDir::Top),
                                Mesh::get_light_level_from_dir(QuadDir::Top)
                                    - grid_additional_light,
                            ));
                        }

                        if get_block_in_chunk_or_world(Vector3::new(x, y + 1, z), world, chunk)
                            .is_some_and(|block| block.is_air())
                        {
                            quads.push(Quad::new(
                                bloc_pos,
                                QuadDir::Bottom,
                                block_type.get_texture_id(QuadDir::Bottom),
                                Mesh::get_light_level_from_dir(QuadDir::Bottom)
                                    - grid_additional_light,
                            ));
                        }
                    }
                }
            }
        }

        Mesh { quads }
    }
}
