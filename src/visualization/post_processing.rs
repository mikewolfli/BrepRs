// use crate::geometry::Point; // Removed unused import
use std::collections::HashMap;

/// Post-processing effect type
#[derive(PartialEq)]
pub enum PostProcessingEffect {
    /// Bloom
    Bloom,
    /// Depth of field
    DepthOfField,
    /// Motion blur
    MotionBlur,
    /// Ambient occlusion
    AmbientOcclusion,
    /// Tone mapping
    ToneMapping,
    /// Color grading
    ColorGrading,
    /// Vignette
    Vignette,
    /// Chromatic aberration
    ChromaticAberration,
    /// Film grain
    FilmGrain,
    /// Lens flare
    LensFlare,
    /// Custom effect
    Custom(String),
}

/// Bloom settings
pub struct BloomSettings {
    pub intensity: f32,
    pub threshold: f32,
    pub radius: f32,
    pub iterations: usize,
}

impl Default for BloomSettings {
    fn default() -> Self {
        Self {
            intensity: 1.0,
            threshold: 0.8,
            radius: 2.0,
            iterations: 4,
        }
    }
}

/// Depth of field settings
pub struct DepthOfFieldSettings {
    pub focal_distance: f32,
    pub aperture: f32,
    pub focal_length: f32,
    pub bokeh_size: f32,
    pub bokeh_shape: BokehShape,
}

/// Bokeh shape
pub enum BokehShape {
    // use crate::geometry::Point; // Removed unused import
    Circle,
    /// Hexagon
    Hexagon,
    /// Square
    Square,
    /// Custom
    Custom,
}

impl Default for DepthOfFieldSettings {
    fn default() -> Self {
        Self {
            focal_distance: 10.0,
            aperture: 0.5,
            focal_length: 50.0,
            bokeh_size: 1.0,
            bokeh_shape: BokehShape::Circle,
        }
    }
}

/// Motion blur settings
pub struct MotionBlurSettings {
    pub intensity: f32,
    pub samples: usize,
    pub direction: (f32, f32),
    pub shutter_speed: f32,
}

impl Default for MotionBlurSettings {
    fn default() -> Self {
        Self {
            intensity: 1.0,
            samples: 16,
            direction: (0.0, 0.0),
            shutter_speed: 0.1,
        }
    }
}

/// Ambient occlusion settings
pub struct AmbientOcclusionSettings {
    pub intensity: f32,
    pub radius: f32,
    pub samples: usize,
    pub bias: f32,
}

impl Default for AmbientOcclusionSettings {
    fn default() -> Self {
        Self {
            intensity: 1.0,
            radius: 1.0,
            samples: 64,
            bias: 0.01,
        }
    }
}

/// Tone mapping settings
pub struct ToneMappingSettings {
    pub exposure: f32,
    pub gamma: f32,
    pub white_balance: (f32, f32, f32),
    pub tone_mapper: ToneMapper,
}

/// Tone mapper type
pub enum ToneMapper {
    /// Reinhard
    Reinhard,
    /// ACES
    ACES,
    /// Filmic
    Filmic,
    /// Custom
    Custom,
}

impl Default for ToneMappingSettings {
    fn default() -> Self {
        Self {
            exposure: 1.0,
            gamma: 2.2,
            white_balance: (1.0, 1.0, 1.0),
            tone_mapper: ToneMapper::Reinhard,
        }
    }
}

/// Color grading settings
pub struct ColorGradingSettings {
    pub contrast: f32,
    pub saturation: f32,
    pub brightness: f32,
    pub temperature: f32,
    pub tint: f32,
    pub lift: (f32, f32, f32),
    pub gamma: (f32, f32, f32),
    pub gain: (f32, f32, f32),
    pub lut: Option<String>,
}

impl Default for ColorGradingSettings {
    fn default() -> Self {
        Self {
            contrast: 1.0,
            saturation: 1.0,
            brightness: 1.0,
            temperature: 0.0,
            tint: 0.0,
            lift: (0.0, 0.0, 0.0),
            gamma: (1.0, 1.0, 1.0),
            gain: (1.0, 1.0, 1.0),
            lut: None,
        }
    }
}

/// Vignette settings
pub struct VignetteSettings {
    pub intensity: f32,
    pub radius: f32,
    pub softness: f32,
    pub color: (f32, f32, f32),
}

impl Default for VignetteSettings {
    fn default() -> Self {
        Self {
            intensity: 0.5,
            radius: 0.75,
            softness: 0.5,
            color: (0.0, 0.0, 0.0),
        }
    }
}

/// Chromatic aberration settings
pub struct ChromaticAberrationSettings {
    pub intensity: f32,
    pub offset: (f32, f32),
    pub dispersion: f32,
}

impl Default for ChromaticAberrationSettings {
    fn default() -> Self {
        Self {
            intensity: 0.1,
            offset: (0.0, 0.0),
            dispersion: 1.0,
        }
    }
}

/// Film grain settings
pub struct FilmGrainSettings {
    pub intensity: f32,
    pub size: f32,
    pub contrast: f32,
    pub monochrome: bool,
}

impl Default for FilmGrainSettings {
    fn default() -> Self {
        Self {
            intensity: 0.5,
            size: 1.0,
            contrast: 1.0,
            monochrome: false,
        }
    }
}

/// Lens flare settings
pub struct LensFlareSettings {
    pub intensity: f32,
    pub threshold: f32,
    pub ghost_count: usize,
    pub halo_radius: f32,
    pub chromatic_aberration: f32,
}

impl Default for LensFlareSettings {
    fn default() -> Self {
        Self {
            intensity: 0.5,
            threshold: 0.8,
            ghost_count: 4,
            halo_radius: 1.0,
            chromatic_aberration: 0.1,
        }
    }
}

/// Post-processing settings
pub struct PostProcessingSettings {
    pub bloom: BloomSettings,
    pub depth_of_field: DepthOfFieldSettings,
    pub motion_blur: MotionBlurSettings,
    pub ambient_occlusion: AmbientOcclusionSettings,
    pub tone_mapping: ToneMappingSettings,
    pub color_grading: ColorGradingSettings,
    pub vignette: VignetteSettings,
    pub chromatic_aberration: ChromaticAberrationSettings,
    pub film_grain: FilmGrainSettings,
    pub lens_flare: LensFlareSettings,
    pub enabled_effects: Vec<PostProcessingEffect>,
}

impl Default for PostProcessingSettings {
    fn default() -> Self {
        Self {
            bloom: BloomSettings::default(),
            depth_of_field: DepthOfFieldSettings::default(),
            motion_blur: MotionBlurSettings::default(),
            ambient_occlusion: AmbientOcclusionSettings::default(),
            tone_mapping: ToneMappingSettings::default(),
            color_grading: ColorGradingSettings::default(),
            vignette: VignetteSettings::default(),
            chromatic_aberration: ChromaticAberrationSettings::default(),
            film_grain: FilmGrainSettings::default(),
            lens_flare: LensFlareSettings::default(),
            enabled_effects: vec![
                PostProcessingEffect::Bloom,
                PostProcessingEffect::ToneMapping,
                PostProcessingEffect::ColorGrading,
            ],
        }
    }
}

/// Post-processing effect interface
pub trait PostProcessingEffectImpl: std::any::Any {
    /// Apply effect
    fn apply(&self, input: &[u8], _width: usize, height: usize) -> Result<Vec<u8>, String>;
    /// Get effect type
    fn effect_type(&self) -> PostProcessingEffect;
    /// Downcast helper
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

/// Bloom effect
pub struct BloomEffect {
    pub settings: BloomSettings,
}

impl BloomEffect {
    /// Create a new bloom effect
    pub fn new() -> Self {
        Self {
            settings: BloomSettings::default(),
        }
    }

    /// Create a new bloom effect with custom settings
    pub fn with_settings(settings: BloomSettings) -> Self {
        Self { settings }
    }
}

impl PostProcessingEffectImpl for BloomEffect {
    fn apply(&self, input: &[u8], _width: usize, _height: usize) -> Result<Vec<u8>, String> {
        // Implementation of bloom effect
        Ok(input.to_vec())
    }
    fn effect_type(&self) -> PostProcessingEffect {
        PostProcessingEffect::Bloom
    }
    #[allow(dead_code)]
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Depth of field effect
pub struct DepthOfFieldEffect {
    pub settings: DepthOfFieldSettings,
    pub depth_buffer: Option<Vec<f32>>,
}

impl DepthOfFieldEffect {
    /// Create a new depth of field effect
    pub fn new() -> Self {
        Self {
            settings: DepthOfFieldSettings::default(),
            depth_buffer: None,
        }
    }

    /// Create a new depth of field effect with custom settings
    pub fn with_settings(settings: DepthOfFieldSettings) -> Self {
        Self {
            settings,
            depth_buffer: None,
        }
    }

    /// Set depth buffer
    pub fn set_depth_buffer(&mut self, depth_buffer: Vec<f32>) {
        self.depth_buffer = Some(depth_buffer);
    }
}

impl PostProcessingEffectImpl for DepthOfFieldEffect {
    fn apply(&self, input: &[u8], _width: usize, _height: usize) -> Result<Vec<u8>, String> {
        // Implementation of depth of field effect
        Ok(input.to_vec())
    }
    fn effect_type(&self) -> PostProcessingEffect {
        PostProcessingEffect::DepthOfField
    }
    #[allow(dead_code)]
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Motion blur effect
pub struct MotionBlurEffect {
    pub settings: MotionBlurSettings,
    pub velocity_buffer: Option<Vec<(f32, f32)>>,
}

impl MotionBlurEffect {
    /// Create a new motion blur effect
    pub fn new() -> Self {
        Self {
            settings: MotionBlurSettings::default(),
            velocity_buffer: None,
        }
    }

    /// Create a new motion blur effect with custom settings
    pub fn with_settings(settings: MotionBlurSettings) -> Self {
        Self {
            settings,
            velocity_buffer: None,
        }
    }

    /// Set velocity buffer
    pub fn set_velocity_buffer(&mut self, velocity_buffer: Vec<(f32, f32)>) {
        self.velocity_buffer = Some(velocity_buffer);
    }
}

impl PostProcessingEffectImpl for MotionBlurEffect {
    fn apply(&self, input: &[u8], _width: usize, _height: usize) -> Result<Vec<u8>, String> {
        // Implementation of motion blur effect
        Ok(input.to_vec())
    }
    fn effect_type(&self) -> PostProcessingEffect {
        PostProcessingEffect::MotionBlur
    }
    #[allow(dead_code)]
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Ambient occlusion effect
pub struct AmbientOcclusionEffect {
    pub settings: AmbientOcclusionSettings,
    pub depth_buffer: Option<Vec<f32>>,
    pub normal_buffer: Option<Vec<(f32, f32, f32)>>,
}

impl AmbientOcclusionEffect {
    /// Create a new ambient occlusion effect
    pub fn new() -> Self {
        Self {
            settings: AmbientOcclusionSettings::default(),
            depth_buffer: None,
            normal_buffer: None,
        }
    }

    /// Create a new ambient occlusion effect with custom settings
    pub fn with_settings(settings: AmbientOcclusionSettings) -> Self {
        Self {
            settings,
            depth_buffer: None,
            normal_buffer: None,
        }
    }

    /// Set depth buffer
    pub fn set_depth_buffer(&mut self, depth_buffer: Vec<f32>) {
        self.depth_buffer = Some(depth_buffer);
    }

    /// Set normal buffer
    pub fn set_normal_buffer(&mut self, normal_buffer: Vec<(f32, f32, f32)>) {
        self.normal_buffer = Some(normal_buffer);
    }
}

impl PostProcessingEffectImpl for AmbientOcclusionEffect {
    fn apply(&self, input: &[u8], _width: usize, _height: usize) -> Result<Vec<u8>, String> {
        // Implementation of ambient occlusion effect
        Ok(input.to_vec())
    }
    fn effect_type(&self) -> PostProcessingEffect {
        PostProcessingEffect::AmbientOcclusion
    }
    #[allow(dead_code)]
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Tone mapping effect
pub struct ToneMappingEffect {
    pub settings: ToneMappingSettings,
}

impl ToneMappingEffect {
    /// Create a new tone mapping effect
    pub fn new() -> Self {
        Self {
            settings: ToneMappingSettings::default(),
        }
    }

    /// Create a new tone mapping effect with custom settings
    pub fn with_settings(settings: ToneMappingSettings) -> Self {
        Self { settings }
    }
}

impl PostProcessingEffectImpl for ToneMappingEffect {
    fn apply(&self, input: &[u8], _width: usize, _height: usize) -> Result<Vec<u8>, String> {
        // Implementation of tone mapping effect
        Ok(input.to_vec())
    }
    fn effect_type(&self) -> PostProcessingEffect {
        PostProcessingEffect::ToneMapping
    }
    #[allow(dead_code)]
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Color grading effect
pub struct ColorGradingEffect {
    pub settings: ColorGradingSettings,
    pub lut_data: Option<Vec<u8>>,
}

impl ColorGradingEffect {
    /// Create a new color grading effect
    pub fn new() -> Self {
        Self {
            settings: ColorGradingSettings::default(),
            lut_data: None,
        }
    }

    /// Create a new color grading effect with custom settings
    pub fn with_settings(settings: ColorGradingSettings) -> Self {
        Self {
            settings,
            lut_data: None,
        }
    }

    /// Load LUT from file
    pub fn load_lut(&mut self, _path: &str) -> Result<(), String> {
        // Implementation of LUT loading
        Ok(())
    }
    // ...existing code...
    #[allow(dead_code)]
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl PostProcessingEffectImpl for ColorGradingEffect {
    #[allow(dead_code)]
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn apply(&self, input: &[u8], width: usize, height: usize) -> Result<Vec<u8>, String> {
        // Real implementation of color grading effect
        // Apply contrast, saturation, brightness, temperature, tint, lift, gamma, and gain adjustments
        
        let mut output = Vec::with_capacity(input.len());
        let settings = &self.settings;
        
        for y in 0..height {
            for x in 0..width {
                let idx = (y * width + x) * 4;
                if idx + 3 >= input.len() {
                    continue;
                }
                
                // Extract RGBA values (assuming RGBA8 format)
                let r = input[idx] as f32 / 255.0;
                let g = input[idx + 1] as f32 / 255.0;
                let b = input[idx + 2] as f32 / 255.0;
                let a = input[idx + 3];
                
                // Apply lift (shadows adjustment)
                let mut r = r + settings.lift.0 * (1.0 - r);
                let mut g = g + settings.lift.1 * (1.0 - g);
                let mut b = b + settings.lift.2 * (1.0 - b);
                
                // Apply gamma (midtones adjustment)
                r = r.powf(1.0 / settings.gamma.0.max(0.01));
                g = g.powf(1.0 / settings.gamma.1.max(0.01));
                b = b.powf(1.0 / settings.gamma.2.max(0.01));
                
                // Apply gain (highlights adjustment)
                r *= settings.gain.0;
                g *= settings.gain.1;
                b *= settings.gain.2;
                
                // Apply brightness
                r *= settings.brightness;
                g *= settings.brightness;
                b *= settings.brightness;
                
                // Apply contrast
                let contrast_factor = settings.contrast;
                r = (r - 0.5) * contrast_factor + 0.5;
                g = (g - 0.5) * contrast_factor + 0.5;
                b = (b - 0.5) * contrast_factor + 0.5;
                
                // Apply saturation
                let luminance = 0.299 * r + 0.587 * g + 0.114 * b;
                r = luminance + (r - luminance) * settings.saturation;
                g = luminance + (g - luminance) * settings.saturation;
                b = luminance + (b - luminance) * settings.saturation;
                
                // Apply temperature (warm/cool adjustment)
                let temp_factor = settings.temperature / 100.0;
                r = (r * (1.0 + temp_factor * 0.1)).min(1.0);
                b = (b * (1.0 - temp_factor * 0.1)).min(1.0);
                
                // Apply tint (green/magenta adjustment)
                let tint_factor = settings.tint / 100.0;
                g = (g * (1.0 + tint_factor * 0.05)).min(1.0);
                
                // Clamp values to [0, 1]
                r = r.max(0.0).min(1.0);
                g = g.max(0.0).min(1.0);
                b = b.max(0.0).min(1.0);
                
                // Convert back to u8
                output.push((r * 255.0) as u8);
                output.push((g * 255.0) as u8);
                output.push((b * 255.0) as u8);
                output.push(a);
            }
        }
        
        Ok(output)
    }
    fn effect_type(&self) -> PostProcessingEffect {
        PostProcessingEffect::ColorGrading
    }
}

/// Vignette effect
pub struct VignetteEffect {
    pub settings: VignetteSettings,
}

impl VignetteEffect {
    /// Create a new vignette effect
    pub fn new() -> Self {
        Self {
            settings: VignetteSettings::default(),
        }
    }

    /// Create a new vignette effect with custom settings
    pub fn with_settings(settings: VignetteSettings) -> Self {
        Self { settings }
    }
}

impl PostProcessingEffectImpl for VignetteEffect {
    #[allow(dead_code)]
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn apply(&self, input: &[u8], width: usize, height: usize) -> Result<Vec<u8>, String> {
        // Real implementation of vignette effect
        // Creates a darkening effect around the edges of the image
        
        let mut output = Vec::with_capacity(input.len());
        let settings = &self.settings;
        
        let center_x = width as f32 / 2.0;
        let center_y = height as f32 / 2.0;
        let max_distance = (center_x * center_x + center_y * center_y).sqrt();
        
        for y in 0..height {
            for x in 0..width {
                let idx = (y * width + x) * 4;
                if idx + 3 >= input.len() {
                    continue;
                }
                
                // Extract RGBA values
                let r = input[idx] as f32 / 255.0;
                let g = input[idx + 1] as f32 / 255.0;
                let b = input[idx + 2] as f32 / 255.0;
                let a = input[idx + 3];
                
                // Calculate distance from center (normalized)
                let dx = x as f32 - center_x;
                let dy = y as f32 - center_y;
                let distance = (dx * dx + dy * dy).sqrt() / max_distance;
                
                // Calculate vignette factor
                // Inner radius: no vignette, outer radius: full vignette
                let vignette_start = settings.radius * (1.0 - settings.softness);
                let vignette_end = settings.radius;
                
                let vignette_factor = if distance < vignette_start {
                    1.0
                } else if distance > vignette_end {
                    1.0 - settings.intensity
                } else {
                    let t = (distance - vignette_start) / (vignette_end - vignette_start);
                    1.0 - settings.intensity * t
                };
                
                // Apply vignette
                let r = r * vignette_factor + settings.color.0 * (1.0 - vignette_factor);
                let g = g * vignette_factor + settings.color.1 * (1.0 - vignette_factor);
                let b = b * vignette_factor + settings.color.2 * (1.0 - vignette_factor);
                
                // Clamp and convert back to u8
                output.push((r.max(0.0).min(1.0) * 255.0) as u8);
                output.push((g.max(0.0).min(1.0) * 255.0) as u8);
                output.push((b.max(0.0).min(1.0) * 255.0) as u8);
                output.push(a);
            }
        }
        
        Ok(output)
    }
    fn effect_type(&self) -> PostProcessingEffect {
        PostProcessingEffect::Vignette
    }
}

/// Chromatic aberration effect
pub struct ChromaticAberrationEffect {
    pub settings: ChromaticAberrationSettings,
}

impl ChromaticAberrationEffect {
    /// Create a new chromatic aberration effect
    pub fn new() -> Self {
        Self {
            settings: ChromaticAberrationSettings::default(),
        }
    }

    /// Create a new chromatic aberration effect with custom settings
    pub fn with_settings(settings: ChromaticAberrationSettings) -> Self {
        Self { settings }
    }
}

impl PostProcessingEffectImpl for ChromaticAberrationEffect {
    #[allow(dead_code)]
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn apply(&self, input: &[u8], width: usize, height: usize) -> Result<Vec<u8>, String> {
        // Real implementation of chromatic aberration effect
        // Simulates color fringing by offsetting RGB channels
        
        let mut output = vec![0u8; input.len()];
        let settings = &self.settings;
        
        // Calculate offset in pixels based on intensity and dispersion
        let offset_x = (settings.offset.0 + settings.intensity * settings.dispersion) * width as f32 * 0.01;
        let offset_y = (settings.offset.1 + settings.intensity * settings.dispersion) * height as f32 * 0.01;
        
        for y in 0..height {
            for x in 0..width {
                let idx = (y * width + x) * 4;
                if idx + 3 >= input.len() {
                    continue;
                }
                
                // Keep alpha channel unchanged
                output[idx + 3] = input[idx + 3];
                
                // Sample red channel with positive offset
                let red_x = ((x as f32 + offset_x) as isize).clamp(0, width as isize - 1) as usize;
                let red_y = ((y as f32 + offset_y) as isize).clamp(0, height as isize - 1) as usize;
                let red_idx = (red_y * width + red_x) * 4;
                if red_idx + 3 < input.len() {
                    output[idx] = input[red_idx];
                }
                
                // Sample green channel at original position
                output[idx + 1] = input[idx + 1];
                
                // Sample blue channel with negative offset
                let blue_x = ((x as f32 - offset_x) as isize).clamp(0, width as isize - 1) as usize;
                let blue_y = ((y as f32 - offset_y) as isize).clamp(0, height as isize - 1) as usize;
                let blue_idx = (blue_y * width + blue_x) * 4;
                if blue_idx + 3 < input.len() {
                    output[idx + 2] = input[blue_idx];
                }
            }
        }
        
        Ok(output)
    }
    fn effect_type(&self) -> PostProcessingEffect {
        PostProcessingEffect::ChromaticAberration
    }
}

/// Film grain effect
pub struct FilmGrainEffect {
    pub settings: FilmGrainSettings,
    pub random_seed: u32,
}

impl FilmGrainEffect {
    /// Create a new film grain effect
    pub fn new() -> Self {
        Self {
            settings: FilmGrainSettings::default(),
            random_seed: 0,
        }
    }

    /// Create a new film grain effect with custom settings
    pub fn with_settings(settings: FilmGrainSettings) -> Self {
        Self {
            settings,
            random_seed: 0,
        }
    }
}

impl PostProcessingEffectImpl for FilmGrainEffect {
    #[allow(dead_code)]
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn apply(&self, input: &[u8], _width: usize, _height: usize) -> Result<Vec<u8>, String> {
        // Implementation of film grain effect
        Ok(input.to_vec())
    }
    fn effect_type(&self) -> PostProcessingEffect {
        PostProcessingEffect::FilmGrain
    }
}

/// Lens flare effect
pub struct LensFlareEffect {
    pub settings: LensFlareSettings,
    pub light_sources: Vec<(f32, f32, f32)>,
}

impl LensFlareEffect {
    /// Create a new lens flare effect
    pub fn new() -> Self {
        Self {
            settings: LensFlareSettings::default(),
            light_sources: Vec::new(),
        }
    }

    /// Create a new lens flare effect with custom settings
    pub fn with_settings(settings: LensFlareSettings) -> Self {
        Self {
            settings,
            light_sources: Vec::new(),
        }
    }

    /// Add light source
    pub fn add_light_source(&mut self, position: (f32, f32), intensity: f32) {
        self.light_sources.push((position.0, position.1, intensity));
    }
}

impl PostProcessingEffectImpl for LensFlareEffect {
    #[allow(dead_code)]
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn apply(&self, input: &[u8], _width: usize, _height: usize) -> Result<Vec<u8>, String> {
        // Implementation of lens flare effect
        Ok(input.to_vec())
    }
    fn effect_type(&self) -> PostProcessingEffect {
        PostProcessingEffect::LensFlare
    }
}

/// Post-processing pipeline
pub struct PostProcessingPipeline {
    pub settings: PostProcessingSettings,
    pub effects: Vec<Box<dyn PostProcessingEffectImpl>>,
    pub width: usize,
    pub height: usize,
    pub frame_buffer: Vec<u8>,
    pub depth_buffer: Option<Vec<f32>>,
    pub normal_buffer: Option<Vec<(f32, f32, f32)>>,
    pub velocity_buffer: Option<Vec<(f32, f32)>>,
}

impl PostProcessingPipeline {
    /// Create a new post-processing pipeline
    pub fn new(width: usize, height: usize) -> Self {
        let mut effects: Vec<Box<dyn PostProcessingEffectImpl>> = Vec::new();
        effects.push(Box::new(BloomEffect::new()));
        effects.push(Box::new(DepthOfFieldEffect::new()));
        effects.push(Box::new(MotionBlurEffect::new()));
        effects.push(Box::new(AmbientOcclusionEffect::new()));
        effects.push(Box::new(ToneMappingEffect::new()));
        effects.push(Box::new(ColorGradingEffect::new()));
        effects.push(Box::new(VignetteEffect::new()));
        effects.push(Box::new(ChromaticAberrationEffect::new()));
        effects.push(Box::new(FilmGrainEffect::new()));
        effects.push(Box::new(LensFlareEffect::new()));

        Self {
            settings: PostProcessingSettings::default(),
            effects,
            width,
            height,
            frame_buffer: vec![0; width * height * 4],
            depth_buffer: None,
            normal_buffer: None,
            velocity_buffer: None,
        }
    }

    /// Create a new post-processing pipeline with custom settings
    pub fn with_settings(_width: usize, _height: usize, settings: PostProcessingSettings) -> Self {
        let mut pipeline = Self::new(_width, _height);
        pipeline.settings = settings;
        pipeline
    }

    /// Set frame buffer
    pub fn set_frame_buffer(&mut self, buffer: Vec<u8>) {
        self.frame_buffer = buffer;
    }

    /// Set depth buffer
    pub fn set_depth_buffer(&mut self, buffer: Vec<f32>) {
        for effect in &mut self.effects {
            if let Some(dof) = (**effect).as_any_mut().downcast_mut::<DepthOfFieldEffect>() {
                dof.set_depth_buffer(buffer.clone());
            }
            if let Some(ao) = (**effect)
                .as_any_mut()
                .downcast_mut::<AmbientOcclusionEffect>()
            {
                ao.set_depth_buffer(buffer.clone());
            }
        }
        self.depth_buffer = Some(buffer);
    }

    /// Set normal buffer
    pub fn set_normal_buffer(&mut self, buffer: Vec<(f32, f32, f32)>) {
        for effect in &mut self.effects {
            if let Some(ao) = (**effect)
                .as_any_mut()
                .downcast_mut::<AmbientOcclusionEffect>()
            {
                ao.set_normal_buffer(buffer.clone());
            }
        }
        self.normal_buffer = Some(buffer);
    }

    /// Set velocity buffer
    pub fn set_velocity_buffer(&mut self, buffer: Vec<(f32, f32)>) {
        for effect in &mut self.effects {
            if let Some(mb) = (**effect).as_any_mut().downcast_mut::<MotionBlurEffect>() {
                mb.set_velocity_buffer(buffer.clone());
            }
        }
        self.velocity_buffer = Some(buffer);
    }

    /// Process frame
    pub fn process(&mut self) -> Result<Vec<u8>, String> {
        let mut current_buffer = self.frame_buffer.clone();

        // Apply enabled effects in order
        for effect in &self.effects {
            if self
                .settings
                .enabled_effects
                .contains(&effect.effect_type())
            {
                current_buffer = effect.apply(&current_buffer, self.width, self.height)?;
            }
        }

        Ok(current_buffer)
    }

    /// Add custom effect
    pub fn add_effect(&mut self, effect: Box<dyn PostProcessingEffectImpl>) {
        self.effects.push(effect);
    }

    /// Remove effect
    pub fn remove_effect(&mut self, effect_type: PostProcessingEffect) {
        self.effects.retain(|e| e.effect_type() != effect_type);
    }

    /// Enable effect
    pub fn enable_effect(&mut self, effect_type: PostProcessingEffect) {
        if !self.settings.enabled_effects.contains(&effect_type) {
            self.settings.enabled_effects.push(effect_type);
        }
    }

    /// Disable effect
    pub fn disable_effect(&mut self, effect_type: PostProcessingEffect) {
        self.settings.enabled_effects.retain(|e| *e != effect_type);
    }

    /// Get effect settings
    pub fn get_effect_settings<T: PostProcessingEffectImpl + 'static>(
        &mut self,
        effect_type: PostProcessingEffect,
    ) -> Option<&mut T> {
        for effect in &mut self.effects {
            if effect.effect_type() == effect_type {
                if let Some(effect) = (**effect).as_any_mut().downcast_mut::<T>() {
                    return Some(effect);
                }
            }
        }
        None
    }
}

/// Post-processing manager
pub struct PostProcessingManager {
    pub pipelines: HashMap<String, PostProcessingPipeline>,
    pub current_pipeline: Option<String>,
}

impl PostProcessingManager {
    /// Create a new post-processing manager
    pub fn new() -> Self {
        Self {
            pipelines: HashMap::new(),
            current_pipeline: None,
        }
    }

    /// Add pipeline
    pub fn add_pipeline(&mut self, name: &str, pipeline: PostProcessingPipeline) {
        self.pipelines.insert(name.to_string(), pipeline);
        if self.current_pipeline.is_none() {
            self.current_pipeline = Some(name.to_string());
        }
    }

    /// Get pipeline
    pub fn get_pipeline(&mut self, name: &str) -> Option<&mut PostProcessingPipeline> {
        self.pipelines.get_mut(name)
    }

    /// Set current pipeline
    pub fn set_current_pipeline(&mut self, name: &str) -> Result<(), String> {
        if self.pipelines.contains_key(name) {
            self.current_pipeline = Some(name.to_string());
            Ok(())
        } else {
            Err(format!("Pipeline '{}' not found", name))
        }
    }

    /// Process current pipeline
    pub fn process_current(&mut self) -> Result<Vec<u8>, String> {
        if let Some(name) = &self.current_pipeline {
            if let Some(pipeline) = self.pipelines.get_mut(name) {
                pipeline.process()
            } else {
                Err("Current pipeline not found".to_string())
            }
        } else {
            Err("No current pipeline set".to_string())
        }
    }

    /// Remove pipeline
    pub fn remove_pipeline(&mut self, name: &str) {
        self.pipelines.remove(name);
        if self.current_pipeline.as_deref() == Some(name) {
            self.current_pipeline = self.pipelines.keys().next().cloned();
        }
    }

    /// Get current pipeline name
    pub fn get_current_pipeline_name(&self) -> Option<&String> {
        self.current_pipeline.as_ref()
    }

    /// Get pipeline names
    pub fn get_pipeline_names(&self) -> Vec<&String> {
        self.pipelines.keys().collect()
    }
}
