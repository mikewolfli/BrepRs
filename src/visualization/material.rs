//! Material system for 3D visualization
//!
//! This module provides material functionality for 3D visualization,
//! including PBR (Physically Based Rendering) materials.
//! Compatible with OpenCASCADE Open API design.

use crate::visualization::light::MaterialLighting;
use crate::visualization::primitives::Color;

/// Material type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialType {
    /// Basic material (simple lighting)
    Basic,
    /// Lambert material (diffuse only)
    Lambert,
    /// Phong material (diffuse + specular)
    Phong,
    /// PBR metallic-roughness material
    PbrMetallicRoughness,
    /// PBR specular-glossiness material
    PbrSpecularGlossiness,
    /// Wireframe material
    Wireframe,
    /// Point cloud material
    Points,
}

impl Default for MaterialType {
    fn default() -> Self {
        MaterialType::PbrMetallicRoughness
    }
}

/// Texture map type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureType {
    /// Diffuse/albedo map
    Diffuse,
    /// Normal map
    Normal,
    /// Metallic map
    Metallic,
    /// Roughness map
    Roughness,
    /// Specular map
    Specular,
    /// Glossiness map
    Glossiness,
    /// Ambient occlusion map
    AmbientOcclusion,
    /// Emissive map
    Emissive,
    /// Opacity map
    Opacity,
    /// Height/displacement map
    Height,
    /// Cube map
    Cube,
}

/// Texture information
#[derive(Debug, Clone, PartialEq)]
pub struct Texture {
    /// Texture type
    pub texture_type: TextureType,
    /// Texture data (bytes)
    pub data: Vec<u8>,
    /// Width
    pub width: u32,
    /// Height
    pub height: u32,
    /// Number of channels
    pub channels: u32,
    /// SRGB flag
    pub srgb: bool,
    /// Repeat mode U
    pub repeat_u: RepeatMode,
    /// Repeat mode V
    pub repeat_v: RepeatMode,
    /// Filter mode
    pub filter: FilterMode,
    /// Mipmapping enabled
    pub mipmaps: bool,
}

impl Texture {
    /// Create a new texture
    pub fn new(texture_type: TextureType, width: u32, height: u32, channels: u32) -> Self {
        let size = (width * height * channels) as usize;
        Self {
            texture_type,
            data: vec![0; size],
            width,
            height,
            channels,
            srgb: texture_type == TextureType::Diffuse || texture_type == TextureType::Emissive,
            repeat_u: RepeatMode::Repeat,
            repeat_v: RepeatMode::Repeat,
            filter: FilterMode::Linear,
            mipmaps: true,
        }
    }

    /// Set pixel data
    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    /// Set repeat mode
    pub fn with_repeat(mut self, u: RepeatMode, v: RepeatMode) -> Self {
        self.repeat_u = u;
        self.repeat_v = v;
        self
    }

    /// Set filter mode
    pub fn with_filter(mut self, filter: FilterMode) -> Self {
        self.filter = filter;
        self
    }

    /// Set mipmaps
    pub fn with_mipmaps(mut self, mipmaps: bool) -> Self {
        self.mipmaps = mipmaps;
        self
    }

    /// Set SRGB
    pub fn with_srgb(mut self, srgb: bool) -> Self {
        self.srgb = srgb;
        self
    }
}

impl Default for Texture {
    fn default() -> Self {
        Self::new(TextureType::Diffuse, 1, 1, 4)
    }
}

/// Texture repeat mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepeatMode {
    /// Repeat texture
    Repeat,
    /// Clamp to edge
    ClampToEdge,
    /// Clamp to border
    ClampToBorder,
    /// Mirrored repeat
    MirroredRepeat,
}

impl Default for RepeatMode {
    fn default() -> Self {
        RepeatMode::Repeat
    }
}

/// Texture filter mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterMode {
    /// Nearest neighbor
    Nearest,
    /// Linear interpolation
    Linear,
    /// Nearest with mipmaps
    NearestMipmapNearest,
    /// Linear with nearest mipmaps
    LinearMipmapNearest,
    /// Nearest with linear mipmaps
    NearestMipmapLinear,
    /// Linear with linear mipmaps
    LinearMipmapLinear,
}

impl Default for FilterMode {
    fn default() -> Self {
        FilterMode::Linear
    }
}

/// Material for 3D rendering
#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    /// Material type
    pub material_type: MaterialType,
    /// Base color (albedo)
    pub base_color: Color,
    /// Metallic factor (0.0 - 1.0)
    pub metallic: f32,
    /// Roughness factor (0.0 - 1.0)
    pub roughness: f32,
    /// Specular factor (for Phong)
    pub specular: f32,
    /// Shininess (for Phong)
    pub shininess: f32,
    /// Emissive color
    pub emissive: Color,
    /// Emissive intensity
    pub emissive_intensity: f32,
    /// Ambient color
    pub ambient: Color,
    /// Opacity (0.0 - 1.0)
    pub opacity: f32,
    /// Alpha test threshold
    pub alpha_cutoff: f32,
    /// Double-sided rendering
    pub double_sided: bool,
    /// Wireframe rendering
    pub wireframe: bool,
    /// Wireframe line width
    pub wireframe_width: f32,
    /// Point size (for point rendering)
    pub point_size: f32,
    /// Textures
    pub textures: Vec<Texture>,
    /// Normal scale
    pub normal_scale: f32,
    /// Ambient occlusion strength
    pub ao_strength: f32,
    /// Clearcoat factor
    pub clearcoat: f32,
    /// Clearcoat roughness
    pub clearcoat_roughness: f32,
    /// Index of refraction
    pub ior: f32,
    /// Transmission (glass-like)
    pub transmission: f32,
    /// Attenuation color (for transmission)
    pub attenuation_color: Color,
    /// Attenuation distance
    pub attenuation_distance: f32,
    /// Sheen color
    pub sheen_color: Color,
    /// Sheen roughness
    pub sheen_roughness: f32,
}

impl Material {
    /// Create a new material
    pub fn new(material_type: MaterialType) -> Self {
        Self {
            material_type,
            base_color: Color::white(),
            metallic: 0.0,
            roughness: 0.5,
            specular: 0.5,
            shininess: 32.0,
            emissive: Color::black(),
            emissive_intensity: 1.0,
            ambient: Color::from_rgb(0.2, 0.2, 0.2),
            opacity: 1.0,
            alpha_cutoff: 0.5,
            double_sided: false,
            wireframe: false,
            wireframe_width: 1.0,
            point_size: 1.0,
            textures: Vec::new(),
            normal_scale: 1.0,
            ao_strength: 1.0,
            clearcoat: 0.0,
            clearcoat_roughness: 0.0,
            ior: 1.5,
            transmission: 0.0,
            attenuation_color: Color::white(),
            attenuation_distance: f32::INFINITY,
            sheen_color: Color::black(),
            sheen_roughness: 0.0,
        }
    }

    /// Create PBR material
    pub fn pbr(base_color: Color, metallic: f32, roughness: f32) -> Self {
        Self::new(MaterialType::PbrMetallicRoughness)
            .with_base_color(base_color)
            .with_metallic(metallic)
            .with_roughness(roughness)
    }

    /// Create basic material
    pub fn basic(color: Color) -> Self {
        Self::new(MaterialType::Basic).with_base_color(color)
    }

    /// Create wireframe material
    pub fn wireframe(color: Color, width: f32) -> Self {
        Self::new(MaterialType::Wireframe)
            .with_base_color(color)
            .with_wireframe(true)
            .with_wireframe_width(width)
    }

    /// Create plastic material
    pub fn plastic(color: Color) -> Self {
        Self::pbr(color, 0.0, 0.3)
    }

    /// Set base color
    pub fn with_base_color(mut self, color: Color) -> Self {
        self.base_color = color;
        self
    }

    /// Set metallic
    pub fn with_metallic(mut self, metallic: f32) -> Self {
        self.metallic = metallic.clamp(0.0, 1.0);
        self
    }

    /// Set roughness
    pub fn with_roughness(mut self, roughness: f32) -> Self {
        self.roughness = roughness.clamp(0.0, 1.0);
        self
    }

    /// Set specular
    pub fn with_specular(mut self, specular: f32) -> Self {
        self.specular = specular.clamp(0.0, 1.0);
        self
    }

    /// Set shininess
    pub fn with_shininess(mut self, shininess: f32) -> Self {
        self.shininess = shininess.clamp(0.0, 128.0);
        self
    }

    /// Set emissive
    pub fn with_emissive(mut self, color: Color, intensity: f32) -> Self {
        self.emissive = color;
        self.emissive_intensity = intensity;
        self
    }

    /// Set ambient
    pub fn with_ambient(mut self, color: Color) -> Self {
        self.ambient = color;
        self
    }

    /// Set opacity
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Set alpha cutoff
    pub fn with_alpha_cutoff(mut self, cutoff: f32) -> Self {
        self.alpha_cutoff = cutoff;
        self
    }

    /// Set double sided
    pub fn with_double_sided(mut self, double_sided: bool) -> Self {
        self.double_sided = double_sided;
        self
    }

    /// Set wireframe
    pub fn with_wireframe(mut self, wireframe: bool) -> Self {
        self.wireframe = wireframe;
        self
    }

    /// Set wireframe width
    pub fn with_wireframe_width(mut self, width: f32) -> Self {
        self.wireframe_width = width;
        self
    }

    /// Set point size
    pub fn with_point_size(mut self, size: f32) -> Self {
        self.point_size = size;
        self
    }

    /// Set normal scale
    pub fn with_normal_scale(mut self, scale: f32) -> Self {
        self.normal_scale = scale;
        self
    }

    /// Set AO strength
    pub fn with_ao_strength(mut self, strength: f32) -> Self {
        self.ao_strength = strength;
        self
    }

    /// Set clearcoat
    pub fn with_clearcoat(mut self, clearcoat: f32, roughness: f32) -> Self {
        self.clearcoat = clearcoat.clamp(0.0, 1.0);
        self.clearcoat_roughness = roughness.clamp(0.0, 1.0);
        self
    }

    /// Set IOR
    pub fn with_ior(mut self, ior: f32) -> Self {
        self.ior = ior.max(1.0);
        self
    }

    /// Set transmission
    pub fn with_transmission(mut self, transmission: f32) -> Self {
        self.transmission = transmission.clamp(0.0, 1.0);
        self
    }

    /// Add texture
    pub fn add_texture(&mut self, texture: Texture) {
        self.textures.push(texture);
    }

    /// Get texture by type
    pub fn get_texture(&self, texture_type: TextureType) -> Option<&Texture> {
        self.textures
            .iter()
            .find(|t| t.texture_type == texture_type)
    }

    /// Check if has texture
    pub fn has_texture(&self, texture_type: TextureType) -> bool {
        self.textures.iter().any(|t| t.texture_type == texture_type)
    }

    /// Check if material is transparent
    pub fn is_transparent(&self) -> bool {
        self.opacity < 1.0 || self.transmission > 0.0
    }

    /// Check if material uses alpha test
    pub fn uses_alpha_test(&self) -> bool {
        self.opacity < 1.0 && self.alpha_cutoff > 0.0
    }

    /// Convert to lighting material
    pub fn to_lighting_material(&self) -> MaterialLighting {
        MaterialLighting {
            ambient: self.ambient,
            diffuse: self.base_color,
            specular: Color::from_rgb(self.specular, self.specular, self.specular),
            emissive: Color::new(
                self.emissive.r * self.emissive_intensity,
                self.emissive.g * self.emissive_intensity,
                self.emissive.b * self.emissive_intensity,
                1.0,
            ),
            shininess: self.shininess,
        }
    }

    /// Get effective roughness (considering clearcoat)
    pub fn effective_roughness(&self) -> f32 {
        if self.clearcoat > 0.0 {
            // Blend roughness with clearcoat roughness
            self.roughness * (1.0 - self.clearcoat) + self.clearcoat_roughness * self.clearcoat
        } else {
            self.roughness
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new(MaterialType::PbrMetallicRoughness)
    }
}

/// Material presets
pub struct MaterialPresets;

impl MaterialPresets {
    /// Default white material
    pub fn default() -> Material {
        Material::pbr(Color::white(), 0.0, 0.5)
    }

    /// Plastic material
    pub fn plastic(color: Color) -> Material {
        Material::pbr(color, 0.0, 0.3)
    }

    /// Metal material
    pub fn metal(color: Color, roughness: f32) -> Material {
        Material::pbr(color, 1.0, roughness)
    }

    /// Gold
    pub fn gold() -> Material {
        Material::pbr(Color::from_rgb(1.0, 0.78, 0.34), 1.0, 0.15)
    }

    /// Silver
    pub fn silver() -> Material {
        Material::pbr(Color::from_rgb(0.97, 0.96, 0.91), 1.0, 0.1)
    }

    /// Copper
    pub fn copper() -> Material {
        Material::pbr(Color::from_rgb(0.95, 0.64, 0.54), 1.0, 0.2)
    }

    /// Iron
    pub fn iron() -> Material {
        Material::pbr(Color::from_rgb(0.56, 0.57, 0.58), 1.0, 0.4)
    }

    /// Chrome
    pub fn chrome() -> Material {
        Material::pbr(Color::from_rgb(0.85, 0.87, 0.88), 1.0, 0.05)
    }

    /// Rubber
    pub fn rubber(color: Color) -> Material {
        Material::pbr(color, 0.0, 0.9)
    }

    /// Glass
    pub fn glass() -> Material {
        Material::pbr(Color::white(), 0.0, 0.0)
            .with_transmission(1.0)
            .with_ior(1.5)
            .with_opacity(0.0)
    }

    /// Water
    pub fn water() -> Material {
        Material::pbr(Color::from_rgb(0.0, 0.3, 0.8), 0.0, 0.1)
            .with_transmission(0.9)
            .with_ior(1.33)
            .with_opacity(0.2)
    }

    /// Wood
    pub fn wood() -> Material {
        Material::pbr(Color::from_rgb(0.6, 0.4, 0.2), 0.0, 0.6)
    }

    /// Concrete
    pub fn concrete() -> Material {
        Material::pbr(Color::from_rgb(0.65, 0.65, 0.65), 0.0, 0.9)
    }

    /// Brick
    pub fn brick() -> Material {
        Material::pbr(Color::from_rgb(0.7, 0.3, 0.2), 0.0, 0.8)
    }

    /// Grass
    pub fn grass() -> Material {
        Material::pbr(Color::from_rgb(0.2, 0.6, 0.1), 0.0, 0.9)
    }

    /// Red material
    pub fn red() -> Material {
        Material::plastic(Color::red())
    }

    /// Green material
    pub fn green() -> Material {
        Material::plastic(Color::green())
    }

    /// Blue material
    pub fn blue() -> Material {
        Material::plastic(Color::blue())
    }

    /// Yellow material
    pub fn yellow() -> Material {
        Material::plastic(Color::yellow())
    }

    /// Cyan material
    pub fn cyan() -> Material {
        Material::plastic(Color::cyan())
    }

    /// Magenta material
    pub fn magenta() -> Material {
        Material::plastic(Color::magenta())
    }

    /// White material
    pub fn white() -> Material {
        Material::plastic(Color::white())
    }

    /// Black material
    pub fn black() -> Material {
        Material::plastic(Color::black())
    }

    /// Gray material
    pub fn gray() -> Material {
        Material::plastic(Color::gray())
    }

    /// Orange material
    pub fn orange() -> Material {
        Material::plastic(Color::orange())
    }

    /// Pink material
    pub fn pink() -> Material {
        Material::plastic(Color::pink())
    }

    /// Purple material
    pub fn purple() -> Material {
        Material::plastic(Color::purple())
    }

    /// Brown material
    pub fn brown() -> Material {
        Material::plastic(Color::brown())
    }

    /// Teal material
    pub fn teal() -> Material {
        Material::plastic(Color::teal())
    }

    /// Lime material
    pub fn lime() -> Material {
        Material::plastic(Color::lime())
    }

    /// Navy material
    pub fn navy() -> Material {
        Material::plastic(Color::navy())
    }

    /// Maroon material
    pub fn maroon() -> Material {
        Material::plastic(Color::maroon())
    }

    /// Olive material
    pub fn olive() -> Material {
        Material::plastic(Color::olive())
    }

    /// Bronze material
    pub fn bronze() -> Material {
        Material::pbr(Color::bronze(), 0.8, 0.3)
    }

    /// Emerald material
    pub fn emerald() -> Material {
        Material::plastic(Color::emerald())
    }

    /// Sky blue material
    pub fn sky_blue() -> Material {
        Material::plastic(Color::sky_blue())
    }

    /// Coral material
    pub fn coral() -> Material {
        Material::plastic(Color::coral())
    }

    /// Salmon material
    pub fn salmon() -> Material {
        Material::plastic(Color::salmon())
    }

    /// Lavender material
    pub fn lavender() -> Material {
        Material::plastic(Color::lavender())
    }

    /// Mint material
    pub fn mint() -> Material {
        Material::plastic(Color::mint())
    }

    /// Peach material
    pub fn peach() -> Material {
        Material::plastic(Color::peach())
    }

    /// Plum material
    pub fn plum() -> Material {
        Material::plastic(Color::plum())
    }

    /// Khaki material
    pub fn khaki() -> Material {
        Material::plastic(Color::khaki())
    }

    /// Turquoise material
    pub fn turquoise() -> Material {
        Material::plastic(Color::turquoise())
    }

    /// Dark magenta material
    pub fn magenta_dark() -> Material {
        Material::plastic(Color::magenta_dark())
    }

    /// Light yellow material
    pub fn yellow_light() -> Material {
        Material::plastic(Color::yellow_light())
    }

    /// Dark red material
    pub fn red_dark() -> Material {
        Material::plastic(Color::red_dark())
    }

    /// Dark green material
    pub fn green_dark() -> Material {
        Material::plastic(Color::green_dark())
    }

    /// Dark blue material
    pub fn blue_dark() -> Material {
        Material::plastic(Color::blue_dark())
    }

    /// Light gray material
    pub fn gray_light() -> Material {
        Material::plastic(Color::gray_light())
    }

    /// Dark gray material
    pub fn gray_dark() -> Material {
        Material::plastic(Color::gray_dark())
    }

    /// Dark gray material (alias)
    pub fn dark_gray() -> Material {
        Material::plastic(Color::dark_gray())
    }

    /// Light gray material (alias)
    pub fn light_gray() -> Material {
        Material::plastic(Color::light_gray())
    }

    /// Wireframe material
    pub fn wireframe() -> Material {
        Material::wireframe(Color::white(), 1.0)
    }

    /// Selection highlight material
    pub fn selection() -> Material {
        Material::pbr(Color::yellow(), 0.0, 0.3).with_emissive(Color::yellow(), 0.3)
    }

    /// Prehighlight material
    pub fn prehighlight() -> Material {
        Material::pbr(Color::cyan(), 0.0, 0.3).with_emissive(Color::cyan(), 0.2)
    }
}

/// Material library for managing materials
#[derive(Debug, Clone, Default)]
pub struct MaterialLibrary {
    /// Materials by name
    materials: std::collections::HashMap<String, Material>,
}

impl MaterialLibrary {
    /// Create a new material library
    pub fn new() -> Self {
        let mut lib = Self {
            materials: std::collections::HashMap::new(),
        };
        lib.load_defaults();
        lib
    }

    /// Load default materials
    fn load_defaults(&mut self) {
        self.add("default", MaterialPresets::default());
        self.add("gold", MaterialPresets::gold());
        self.add("silver", MaterialPresets::silver());
        self.add("copper", MaterialPresets::copper());
        self.add("chrome", MaterialPresets::chrome());
        self.add("glass", MaterialPresets::glass());
        self.add("plastic", MaterialPresets::plastic(Color::white()));
        self.add("metal", MaterialPresets::metal(Color::gray(), 0.3));
        self.add("rubber", MaterialPresets::rubber(Color::gray()));
        self.add("wood", MaterialPresets::wood());
        self.add("concrete", MaterialPresets::concrete());
        self.add("brick", MaterialPresets::brick());
        self.add("grass", MaterialPresets::grass());
        self.add("water", MaterialPresets::water());

        // Color materials
        self.add("red", MaterialPresets::red());
        self.add("green", MaterialPresets::green());
        self.add("blue", MaterialPresets::blue());
        self.add("yellow", MaterialPresets::yellow());
        self.add("cyan", MaterialPresets::cyan());
        self.add("magenta", MaterialPresets::magenta());
        self.add("white", MaterialPresets::white());
        self.add("black", MaterialPresets::black());
        self.add("gray", MaterialPresets::gray());
        self.add("orange", MaterialPresets::orange());
        self.add("pink", MaterialPresets::pink());
        self.add("purple", MaterialPresets::purple());
        self.add("brown", MaterialPresets::brown());
        self.add("teal", MaterialPresets::teal());
        self.add("lime", MaterialPresets::lime());
        self.add("navy", MaterialPresets::navy());
        self.add("maroon", MaterialPresets::maroon());
        self.add("olive", MaterialPresets::olive());
        self.add("bronze", MaterialPresets::bronze());
        self.add("emerald", MaterialPresets::emerald());
        self.add("sky_blue", MaterialPresets::sky_blue());
        self.add("coral", MaterialPresets::coral());
        self.add("salmon", MaterialPresets::salmon());
        self.add("lavender", MaterialPresets::lavender());
        self.add("mint", MaterialPresets::mint());
        self.add("peach", MaterialPresets::peach());
        self.add("plum", MaterialPresets::plum());
        self.add("khaki", MaterialPresets::khaki());
        self.add("turquoise", MaterialPresets::turquoise());
        self.add("magenta_dark", MaterialPresets::magenta_dark());
        self.add("yellow_light", MaterialPresets::yellow_light());
        self.add("red_dark", MaterialPresets::red_dark());
        self.add("green_dark", MaterialPresets::green_dark());
        self.add("blue_dark", MaterialPresets::blue_dark());
        self.add("gray_light", MaterialPresets::gray_light());
        self.add("gray_dark", MaterialPresets::gray_dark());
        self.add("dark_gray", MaterialPresets::dark_gray());
        self.add("light_gray", MaterialPresets::light_gray());

        // Special materials
        self.add("wireframe", MaterialPresets::wireframe());
        self.add("selection", MaterialPresets::selection());
        self.add("prehighlight", MaterialPresets::prehighlight());
    }

    /// Add material
    pub fn add(&mut self, name: &str, material: Material) {
        self.materials.insert(name.to_string(), material);
    }

    /// Get material
    pub fn get(&self, name: &str) -> Option<&Material> {
        self.materials.get(name)
    }

    /// Get material mutably
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Material> {
        self.materials.get_mut(name)
    }

    /// Remove material
    pub fn remove(&mut self, name: &str) -> Option<Material> {
        self.materials.remove(name)
    }

    /// Check if material exists
    pub fn contains(&self, name: &str) -> bool {
        self.materials.contains_key(name)
    }

    /// Get material names
    pub fn names(&self) -> Vec<&String> {
        self.materials.keys().collect()
    }

    /// Clear all materials
    pub fn clear(&mut self) {
        self.materials.clear();
    }

    /// Material count
    pub fn count(&self) -> usize {
        self.materials.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_creation() {
        let material = Material::new(MaterialType::PbrMetallicRoughness);
        assert_eq!(material.material_type, MaterialType::PbrMetallicRoughness);
        assert_eq!(material.metallic, 0.0);
        assert_eq!(material.roughness, 0.5);
    }

    #[test]
    fn test_pbr_material() {
        let material = Material::pbr(Color::red(), 0.5, 0.3);
        assert_eq!(material.base_color, Color::red());
        assert_eq!(material.metallic, 0.5);
        assert_eq!(material.roughness, 0.3);
    }

    #[test]
    fn test_material_presets() {
        let gold = MaterialPresets::gold();
        assert_eq!(gold.metallic, 1.0);
        assert!(gold.roughness < 0.5);

        let glass = MaterialPresets::glass();
        assert!(glass.is_transparent());

        let wireframe = MaterialPresets::wireframe();
        assert!(wireframe.wireframe);
    }

    #[test]
    fn test_material_transparency() {
        let opaque = Material::pbr(Color::white(), 0.0, 0.5);
        assert!(!opaque.is_transparent());

        let transparent = Material::pbr(Color::white(), 0.0, 0.5).with_opacity(0.5);
        assert!(transparent.is_transparent());
    }

    #[test]
    fn test_material_library() {
        let mut lib = MaterialLibrary::new();
        assert!(lib.contains("default"));
        assert!(lib.contains("gold"));

        let material = Material::pbr(Color::red(), 0.0, 0.5);
        lib.add("custom", material);
        assert!(lib.contains("custom"));

        let retrieved = lib.get("custom");
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_texture_creation() {
        let texture = Texture::new(TextureType::Diffuse, 256, 256, 4);
        assert_eq!(texture.width, 256);
        assert_eq!(texture.height, 256);
        assert_eq!(texture.channels, 4);
    }
}
