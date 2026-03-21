//! Material and Texture Generation Module
//!
//! This module provides functionality for generating and managing materials and textures for 3D models,
//! including material definition, texture generation, and mapping.

use std::collections::HashMap;

use crate::ai_ml::protocol::{AiProtocolError, AiResult};
use crate::mesh::mesh_data::Mesh3D;

/// Material Properties
#[derive(Debug, Clone, PartialEq)]
pub struct MaterialProperties {
    pub name: String,
    pub diffuse_color: (f32, f32, f32),  // RGB
    pub specular_color: (f32, f32, f32), // RGB
    pub shininess: f32,                  // 0.0 to 100.0
    pub opacity: f32,                    // 0.0 to 1.0
    pub roughness: f32,                  // 0.0 to 1.0
    pub metallic: f32,                   // 0.0 to 1.0
    pub emissive_color: (f32, f32, f32), // RGB
    pub emissive_intensity: f32,         // 0.0 to 1.0
}

impl Default for MaterialProperties {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            diffuse_color: (0.5, 0.5, 0.5),
            specular_color: (1.0, 1.0, 1.0),
            shininess: 32.0,
            opacity: 1.0,
            roughness: 0.5,
            metallic: 0.0,
            emissive_color: (0.0, 0.0, 0.0),
            emissive_intensity: 0.0,
        }
    }
}

/// Texture Properties
#[derive(Debug, Clone, PartialEq)]
pub struct TextureProperties {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub channels: u32, // 1: grayscale, 3: RGB, 4: RGBA
    pub data: Vec<u8>, // Raw texture data
    pub texture_type: TextureType,
}

/// Texture Type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextureType {
    Diffuse,
    Specular,
    Normal,
    Roughness,
    Metallic,
    Emissive,
    AmbientOcclusion,
    Height,
}

/// Material Definition
#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub properties: MaterialProperties,
    pub textures: HashMap<TextureType, TextureProperties>,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            properties: MaterialProperties::default(),
            textures: HashMap::new(),
        }
    }
}

/// Material Generation Settings
#[derive(Debug, Default, Clone)]
pub struct MaterialGenerationSettings {
    pub style: String,                  // e.g., "wood", "metal", "plastic", "fabric"
    pub color: Option<(f32, f32, f32)>, // Optional RGB color
    pub texture_resolution: (u32, u32), // (width, height)
    pub detail_level: f64,              // 0.0 to 1.0
    pub seed: Option<u64>,              // random seed for reproducibility
}

/// Material Generation Result
pub struct MaterialGenerationResult {
    pub material: Material,
    pub description: String,
    pub settings: MaterialGenerationSettings,
    pub generation_time: f64, // in seconds
    pub quality_score: f64,   // 0.0 to 1.0
}

/// Perlin Noise Generator for Procedural Textures
struct PerlinNoise {
    seed: u64,
}

impl PerlinNoise {
    fn new(seed: u64) -> Self {
        Self { seed }
    }

    fn noise(&self, x: f64, y: f64) -> f64 {
        let i = x.floor() as i32;
        let j = y.floor() as i32;
        let xf = x - i as f64;
        let yf = y - j as f64;

        let n00 = self.grad(i, j, xf, yf);
        let n10 = self.grad(i + 1, j, xf - 1.0, yf);
        let n01 = self.grad(i, j + 1, xf, yf - 1.0);
        let n11 = self.grad(i + 1, j + 1, xf - 1.0, yf - 1.0);

        let u = self.fade(xf);
        let v = self.fade(yf);

        let nx0 = self.lerp(n00, n10, u);
        let nx1 = self.lerp(n01, n11, u);

        self.lerp(nx0, nx1, v)
    }

    fn fbm(&self, x: f64, y: f64, octaves: u32) -> f64 {
        let mut value = 0.0;
        let mut amplitude = 1.0;
        let mut frequency = 1.0;
        let mut max_value = 0.0;

        for _ in 0..octaves {
            value += amplitude * self.noise(x * frequency, y * frequency);
            max_value += amplitude;
            amplitude *= 0.5;
            frequency *= 2.0;
        }

        value / max_value
    }

    fn grad(&self, i: i32, j: i32, xf: f64, yf: f64) -> f64 {
        let hash = self.hash(i, j);
        let h = hash & 3;
        let u = if h < 2 { xf } else { yf };
        let v = if h < 2 { yf } else { xf };
        (if h & 1 == 0 { u } else { -u }) + (if h & 2 == 0 { v } else { -v })
    }

    fn hash(&self, i: i32, j: i32) -> u32 {
        let mut h = (self.seed as u32).wrapping_add(i as u32);
        h = h.wrapping_mul(374761393);
        h = h.wrapping_add(j as u32);
        h = h.wrapping_mul(668265263);
        h ^ h >> 13
    }

    fn fade(&self, t: f64) -> f64 {
        t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
    }

    fn lerp(&self, a: f64, b: f64, t: f64) -> f64 {
        a + t * (b - a)
    }
}

/// Material and Texture Generator
pub struct MaterialTextureGenerator {
    settings: MaterialGenerationSettings,
    perlin: PerlinNoise,
}

impl MaterialTextureGenerator {
    pub fn new() -> Self {
        Self {
            settings: MaterialGenerationSettings::default(),
            perlin: PerlinNoise::new(42),
        }
    }

    pub fn with_settings(mut self, settings: MaterialGenerationSettings) -> Self {
        let seed = settings.seed.unwrap_or(42);
        self.settings = settings;
        self.perlin = PerlinNoise::new(seed);
        self
    }

    /// Generate material from text description
    pub fn generate_material(&self, description: &str) -> AiResult<MaterialGenerationResult> {
        let start_time = std::time::Instant::now();

        let processed_description = self.process_description(description)?;

        let features = self.extract_features(&processed_description)?;

        let properties = self.generate_material_properties(&features)?;

        let textures = self.generate_textures(&features)?;

        let material = Material {
            properties,
            textures,
        };

        let generation_time = start_time.elapsed().as_secs_f64();

        let quality_score = self.calculate_quality_score(&material, description);

        Ok(MaterialGenerationResult {
            material,
            description: description.to_string(),
            settings: self.settings.clone(),
            generation_time,
            quality_score,
        })
    }

    /// Apply material to mesh with UV mapping
    pub fn apply_material(&self, mesh: &Mesh3D, _material: &Material) -> AiResult<Mesh3D> {
        let mut result_mesh = mesh.clone();

        for vertex in &mut result_mesh.vertices {
            if vertex.uv.is_none() {
                vertex.uv = Some([vertex.point.x % 1.0, vertex.point.y % 1.0]);
            }
        }

        Ok(result_mesh)
    }

    /// Process text description
    fn process_description(&self, description: &str) -> AiResult<String> {
        let processed = description
            .trim()
            .to_lowercase()
            .replace(&['.', ',', '!', '?'][..], "");

        if processed.is_empty() {
            return Err(AiProtocolError::InvalidData(
                "Empty description".to_string(),
            ));
        }

        Ok(processed)
    }

    /// Extract features from description
    fn extract_features(&self, description: &str) -> AiResult<HashMap<String, String>> {
        let mut features = HashMap::new();

        if description.contains("wood") {
            features.insert("type".to_string(), "wood".to_string());
        } else if description.contains("metal") {
            features.insert("type".to_string(), "metal".to_string());
        } else if description.contains("plastic") {
            features.insert("type".to_string(), "plastic".to_string());
        } else if description.contains("fabric") || description.contains("cloth") {
            features.insert("type".to_string(), "fabric".to_string());
        } else if description.contains("glass") {
            features.insert("type".to_string(), "glass".to_string());
        } else if description.contains("stone") {
            features.insert("type".to_string(), "stone".to_string());
        } else if description.contains("ceramic") {
            features.insert("type".to_string(), "ceramic".to_string());
        } else if description.contains("leather") {
            features.insert("type".to_string(), "leather".to_string());
        } else if description.contains("concrete") {
            features.insert("type".to_string(), "concrete".to_string());
        } else {
            features.insert("type".to_string(), "generic".to_string());
        }

        if description.contains("red") {
            features.insert("color".to_string(), "red".to_string());
        } else if description.contains("blue") {
            features.insert("color".to_string(), "blue".to_string());
        } else if description.contains("green") {
            features.insert("color".to_string(), "green".to_string());
        } else if description.contains("yellow") {
            features.insert("color".to_string(), "yellow".to_string());
        } else if description.contains("black") {
            features.insert("color".to_string(), "black".to_string());
        } else if description.contains("white") {
            features.insert("color".to_string(), "white".to_string());
        } else if description.contains("gray") || description.contains("grey") {
            features.insert("color".to_string(), "gray".to_string());
        } else if description.contains("brown") {
            features.insert("color".to_string(), "brown".to_string());
        } else if description.contains("gold") {
            features.insert("color".to_string(), "gold".to_string());
        } else if description.contains("silver") {
            features.insert("color".to_string(), "silver".to_string());
        } else if description.contains("orange") {
            features.insert("color".to_string(), "orange".to_string());
        } else if description.contains("purple") {
            features.insert("color".to_string(), "purple".to_string());
        } else {
            features.insert("color".to_string(), "default".to_string());
        }

        if description.contains("shiny") || description.contains("glossy") {
            features.insert("finish".to_string(), "shiny".to_string());
        } else if description.contains("matte") || description.contains("dull") {
            features.insert("finish".to_string(), "matte".to_string());
        } else if description.contains("rough") {
            features.insert("finish".to_string(), "rough".to_string());
        } else {
            features.insert("finish".to_string(), "default".to_string());
        }

        Ok(features)
    }

    /// Generate material properties
    fn generate_material_properties(
        &self,
        features: &HashMap<String, String>,
    ) -> AiResult<MaterialProperties> {
        let generic_str = "generic".to_string();
        let default_str = "default".to_string();
        let material_type = features.get("type").unwrap_or(&generic_str);
        let color = features.get("color").unwrap_or(&default_str);
        let finish = features.get("finish").unwrap_or(&default_str);

        let mut properties = MaterialProperties::default();
        properties.name = format!("{}_{}_{}", material_type, color, finish);

        match color.as_str() {
            "red" => properties.diffuse_color = (1.0, 0.0, 0.0),
            "blue" => properties.diffuse_color = (0.0, 0.0, 1.0),
            "green" => properties.diffuse_color = (0.0, 1.0, 0.0),
            "yellow" => properties.diffuse_color = (1.0, 1.0, 0.0),
            "black" => properties.diffuse_color = (0.0, 0.0, 0.0),
            "white" => properties.diffuse_color = (1.0, 1.0, 1.0),
            "gray" => properties.diffuse_color = (0.5, 0.5, 0.5),
            "brown" => properties.diffuse_color = (0.5, 0.35, 0.15),
            "gold" => properties.diffuse_color = (1.0, 0.84, 0.0),
            "silver" => properties.diffuse_color = (0.75, 0.75, 0.75),
            "orange" => properties.diffuse_color = (1.0, 0.5, 0.0),
            "purple" => properties.diffuse_color = (0.5, 0.0, 0.5),
            _ => {}
        }

        match material_type.as_str() {
            "wood" => {
                properties.roughness = 0.7;
                properties.metallic = 0.0;
                properties.shininess = 10.0;
            }
            "metal" => {
                properties.roughness = 0.2;
                properties.metallic = 1.0;
                properties.shininess = 80.0;
            }
            "plastic" => {
                properties.roughness = 0.4;
                properties.metallic = 0.0;
                properties.shininess = 60.0;
            }
            "fabric" => {
                properties.roughness = 0.9;
                properties.metallic = 0.0;
                properties.shininess = 5.0;
            }
            "glass" => {
                properties.roughness = 0.0;
                properties.metallic = 0.0;
                properties.shininess = 100.0;
                properties.opacity = 0.3;
            }
            "stone" => {
                properties.roughness = 0.8;
                properties.metallic = 0.0;
                properties.shininess = 15.0;
            }
            "ceramic" => {
                properties.roughness = 0.3;
                properties.metallic = 0.0;
                properties.shininess = 70.0;
            }
            "leather" => {
                properties.roughness = 0.85;
                properties.metallic = 0.0;
                properties.shininess = 8.0;
            }
            "concrete" => {
                properties.roughness = 0.9;
                properties.metallic = 0.0;
                properties.shininess = 5.0;
            }
            _ => {}
        }

        match finish.as_str() {
            "shiny" => {
                properties.roughness = properties.roughness * 0.5;
                properties.shininess = properties.shininess * 1.5;
            }
            "matte" => {
                properties.roughness = properties.roughness * 1.5;
                properties.shininess = properties.shininess * 0.5;
            }
            "rough" => {
                properties.roughness = 0.9;
                properties.shininess = 5.0;
            }
            _ => {}
        }

        if let Some(color) = self.settings.color {
            properties.diffuse_color = color;
        }

        Ok(properties)
    }

    /// Generate textures
    fn generate_textures(
        &self,
        features: &HashMap<String, String>,
    ) -> AiResult<HashMap<TextureType, TextureProperties>> {
        let mut textures = HashMap::new();
        let generic_str = "generic".to_string();
        let material_type = features.get("type").unwrap_or(&generic_str);
        let (width, height) = self.settings.texture_resolution;

        let diffuse_texture = self.generate_texture(
            &format!("{}_diffuse", material_type),
            width,
            height,
            3,
            TextureType::Diffuse,
            features,
        )?;
        textures.insert(TextureType::Diffuse, diffuse_texture);

        let normal_texture = self.generate_texture(
            &format!("{}_normal", material_type),
            width,
            height,
            3,
            TextureType::Normal,
            features,
        )?;
        textures.insert(TextureType::Normal, normal_texture);

        let roughness_texture = self.generate_texture(
            &format!("{}_roughness", material_type),
            width,
            height,
            1,
            TextureType::Roughness,
            features,
        )?;
        textures.insert(TextureType::Roughness, roughness_texture);

        let ao_texture = self.generate_texture(
            &format!("{}_ao", material_type),
            width,
            height,
            1,
            TextureType::AmbientOcclusion,
            features,
        )?;
        textures.insert(TextureType::AmbientOcclusion, ao_texture);

        Ok(textures)
    }

    /// Generate a single texture with procedural patterns
    fn generate_texture(
        &self,
        name: &str,
        width: u32,
        height: u32,
        channels: u32,
        texture_type: TextureType,
        features: &HashMap<String, String>,
    ) -> AiResult<TextureProperties> {
        let data_size = (width * height * channels) as usize;
        let mut data = vec![0u8; data_size];
        let generic_str = "generic".to_string();
        let material_type = features.get("type").unwrap_or(&generic_str);

        match texture_type {
            TextureType::Diffuse => {
                self.generate_diffuse_texture(&mut data, width, height, channels, material_type);
            }
            TextureType::Normal => {
                self.generate_normal_texture(&mut data, width, height, channels, material_type);
            }
            TextureType::Roughness => {
                self.generate_roughness_texture(&mut data, width, height, channels, material_type);
            }
            TextureType::AmbientOcclusion => {
                self.generate_ao_texture(&mut data, width, height, channels, material_type);
            }
            TextureType::Specular => {
                self.generate_specular_texture(&mut data, width, height, channels, material_type);
            }
            _ => {
                for i in 0..data_size {
                    data[i] = 128;
                }
            }
        }

        Ok(TextureProperties {
            name: name.to_string(),
            width,
            height,
            channels,
            data,
            texture_type,
        })
    }

    /// Generate diffuse texture with material-specific patterns
    fn generate_diffuse_texture(
        &self,
        data: &mut [u8],
        width: u32,
        height: u32,
        channels: u32,
        material_type: &str,
    ) {
        let scale = 0.02;
        let detail = self.settings.detail_level as u32 + 1;

        for y in 0..height {
            for x in 0..width {
                let index = (y * width + x) as usize * channels as usize;
                let nx = x as f64 * scale;
                let ny = y as f64 * scale;

                let (r, g, b) = match material_type {
                    "wood" => {
                        let grain = self.perlin.fbm(nx * 4.0, ny * 4.0, detail);
                        let base = 139;
                        let variation = (grain * 30.0) as u8;
                        let r = (base + variation).min(255);
                        let g = (base + variation - 20).max(0).min(255) as u8;
                        let b = (base + variation - 40).max(0).min(255) as u8;
                        (r, g, b)
                    }
                    "metal" => {
                        let noise = self.perlin.fbm(nx * 8.0, ny * 8.0, detail);
                        let value = (128.0 + noise * 40.0) as u8;
                        (value, value, value)
                    }
                    "stone" => {
                        let noise = self.perlin.fbm(nx * 2.0, ny * 2.0, detail);
                        let value = (128.0 + noise * 60.0) as u8;
                        (value, value, value)
                    }
                    "fabric" => {
                        let pattern = ((x as i32 / 4 + y as i32 / 4) % 2) as f64;
                        let noise = self.perlin.noise(nx * 10.0, ny * 10.0);
                        let value = (180.0 + pattern * 40.0 + noise * 20.0) as u8;
                        (value, value, value)
                    }
                    "leather" => {
                        let noise = self.perlin.fbm(nx * 6.0, ny * 6.0, detail);
                        let value = (120.0 + noise * 50.0) as u8;
                        (value, value, value)
                    }
                    "concrete" => {
                        let noise = self.perlin.fbm(nx * 3.0, ny * 3.0, detail);
                        let value = (150.0 + noise * 70.0) as u8;
                        (value, value, value)
                    }
                    _ => {
                        let noise = self.perlin.noise(nx, ny);
                        let value = (128.0 + noise * 64.0) as u8;
                        (value, value, value)
                    }
                };

                for c in 0..channels {
                    data[index + c as usize] = match c {
                        0 => r,
                        1 => g,
                        2 => b,
                        _ => 255,
                    };
                }
            }
        }
    }

    /// Generate normal texture
    fn generate_normal_texture(
        &self,
        data: &mut [u8],
        width: u32,
        height: u32,
        channels: u32,
        material_type: &str,
    ) {
        let scale = 0.02;
        let strength = match material_type {
            "stone" | "concrete" => 2.0,
            "wood" | "leather" => 1.5,
            "fabric" => 0.8,
            _ => 1.0,
        };

        for y in 0..height {
            for x in 0..width {
                let index = (y * width + x) as usize * channels as usize;
                let nx = x as f64 * scale;
                let ny = y as f64 * scale;

                let dx = self.perlin.fbm((nx + 0.01) * 2.0, ny * 2.0, 3) 
                       - self.perlin.fbm((nx - 0.01) * 2.0, ny * 2.0, 3);
                let dy = self.perlin.fbm(nx * 2.0, (ny + 0.01) * 2.0, 3) 
                       - self.perlin.fbm(nx * 2.0, (ny - 0.01) * 2.0, 3);

                let normal_x = (dx * strength * 127.0 + 128.0) as u8;
                let normal_y = (dy * strength * 127.0 + 128.0) as u8;
                let normal_z = 255;

                for c in 0..channels {
                    data[index + c as usize] = match c {
                        0 => normal_x,
                        1 => normal_y,
                        2 => normal_z,
                        _ => 255,
                    };
                }
            }
        }
    }

    /// Generate roughness texture
    fn generate_roughness_texture(
        &self,
        data: &mut [u8],
        width: u32,
        height: u32,
        channels: u32,
        material_type: &str,
    ) {
        let base_roughness = match material_type {
            "wood" => 0.7,
            "metal" => 0.2,
            "plastic" => 0.4,
            "fabric" => 0.9,
            "glass" => 0.0,
            "stone" => 0.8,
            "ceramic" => 0.3,
            "leather" => 0.85,
            "concrete" => 0.9,
            _ => 0.5,
        };

        let scale = 0.03;
        for y in 0..height {
            for x in 0..width {
                let index = (y * width + x) as usize * channels as usize;
                let nx = x as f64 * scale;
                let ny = y as f64 * scale;

                let variation = self.perlin.noise(nx, ny) * 0.2;
                let roughness = (base_roughness + variation).max(0.0).min(1.0);
                let value = (roughness * 255.0) as u8;

                for c in 0..channels {
                    data[index + c as usize] = value;
                }
            }
        }
    }

    /// Generate ambient occlusion texture
    fn generate_ao_texture(
        &self,
        data: &mut [u8],
        width: u32,
        height: u32,
        channels: u32,
        _material_type: &str,
    ) {
        let scale = 0.04;
        for y in 0..height {
            for x in 0..width {
                let index = (y * width + x) as usize * channels as usize;
                let nx = x as f64 * scale;
                let ny = y as f64 * scale;

                let ao = self.perlin.fbm(nx, ny, 4);
                let value = (255.0 - ao * 100.0).max(0.0) as u8;

                for c in 0..channels {
                    data[index + c as usize] = value;
                }
            }
        }
    }

    /// Generate specular texture
    fn generate_specular_texture(
        &self,
        data: &mut [u8],
        width: u32,
        height: u32,
        channels: u32,
        material_type: &str,
    ) {
        let base_specular = match material_type {
            "metal" => 0.9,
            "glass" => 1.0,
            "ceramic" => 0.8,
            "plastic" => 0.6,
            "wood" => 0.2,
            "fabric" => 0.1,
            "stone" => 0.3,
            "leather" => 0.15,
            "concrete" => 0.1,
            _ => 0.5,
        };

        let scale = 0.02;
        for y in 0..height {
            for x in 0..width {
                let index = (y * width + x) as usize * channels as usize;
                let nx = x as f64 * scale;
                let ny = y as f64 * scale;

                let variation = self.perlin.noise(nx, ny) * 0.1;
                let specular = (base_specular + variation).max(0.0).min(1.0);
                let value = (specular * 255.0) as u8;

                for c in 0..channels {
                    data[index + c as usize] = value;
                }
            }
        }
    }

    /// Calculate quality score for generated material
    fn calculate_quality_score(&self, material: &Material, _description: &str) -> f64 {
        let texture_count = material.textures.len() as f64;
        
        let roughness_score = 1.0 - material.properties.roughness;
        let metallic_score = material.properties.metallic;
        let shininess_score = material.properties.shininess / 100.0;
        let opacity_score = material.properties.opacity;
        
        let property_score = (roughness_score + metallic_score + shininess_score + opacity_score) / 4.0;
        
        (texture_count / 5.0 + property_score as f64) / 2.0
    }
}

/// Extension methods for Mesh3D
pub trait MaterialTextureExt {
    fn apply_material(&self, material: &Material) -> AiResult<Mesh3D>;

    fn apply_material_from_text(
        &self,
        description: &str,
        settings: &MaterialGenerationSettings,
    ) -> AiResult<(Mesh3D, Material)>;
}

impl MaterialTextureExt for Mesh3D {
    fn apply_material(&self, material: &Material) -> AiResult<Mesh3D> {
        let generator = MaterialTextureGenerator::new();
        generator.apply_material(self, material)
    }

    fn apply_material_from_text(
        &self,
        description: &str,
        settings: &MaterialGenerationSettings,
    ) -> AiResult<(Mesh3D, Material)> {
        let generator = MaterialTextureGenerator::new().with_settings((*settings).clone());
        let result = generator.generate_material(description)?;
        let mesh = generator.apply_material(self, &result.material)?;
        Ok((mesh, result.material))
    }
}
