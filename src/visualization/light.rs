//! Lighting for 3D visualization
//!
//! This module provides lighting functionality for 3D visualization,
//! including various light types and lighting calculations.
//! Compatible with OpenCASCADE Open API design.

use crate::geometry::{Point, Vector};
use crate::visualization::primitives::Color;

/// Light type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightType {
    /// Ambient light (global illumination)
    Ambient,
    /// Directional light (parallel rays, like sun)
    Directional,
    /// Point light (omnidirectional, like bulb)
    Point,
    /// Spot light (cone-shaped, like flashlight)
    Spot,
    /// Headlight (directional light from camera)
    Headlight,
}

impl Default for LightType {
    fn default() -> Self {
        LightType::Directional
    }
}

/// Light source for 3D visualization
#[derive(Debug, Clone, PartialEq)]
pub struct Light {
    /// Light type
    pub light_type: LightType,
    /// Light position (for point and spot lights)
    pub position: Point,
    /// Light direction (for directional and spot lights)
    pub direction: Vector,
    /// Light color
    pub color: Color,
    /// Light intensity
    pub intensity: f32,
    /// Ambient coefficient
    pub ambient: f32,
    /// Diffuse coefficient
    pub diffuse: f32,
    /// Specular coefficient
    pub specular: f32,
    /// Spot light inner cone angle (degrees)
    pub spot_inner_angle: f32,
    /// Spot light outer cone angle (degrees)
    pub spot_outer_angle: f32,
    /// Spot light falloff exponent
    pub spot_falloff: f32,
    /// Attenuation constant
    pub att_constant: f32,
    /// Attenuation linear
    pub att_linear: f32,
    /// Attenuation quadratic
    pub att_quadratic: f32,
    /// Light enabled flag
    pub enabled: bool,
    /// Cast shadows flag
    pub cast_shadows: bool,
}

impl Light {
    /// Create a new light
    pub fn new(light_type: LightType) -> Self {
        Self {
            light_type,
            position: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, -1.0),
            color: Color::white(),
            intensity: 1.0,
            ambient: 0.2,
            diffuse: 0.8,
            specular: 0.5,
            spot_inner_angle: 30.0,
            spot_outer_angle: 45.0,
            spot_falloff: 1.0,
            att_constant: 1.0,
            att_linear: 0.0,
            att_quadratic: 0.0,
            enabled: true,
            cast_shadows: false,
        }
    }

    /// Create ambient light
    pub fn ambient(color: Color, intensity: f32) -> Self {
        Self {
            light_type: LightType::Ambient,
            color,
            intensity,
            ambient: 1.0,
            diffuse: 0.0,
            specular: 0.0,
            ..Default::default()
        }
    }

    /// Create directional light
    pub fn directional(direction: Vector, color: Color, intensity: f32) -> Self {
        Self {
            light_type: LightType::Directional,
            direction: direction.normalized(),
            color,
            intensity,
            ..Default::default()
        }
    }

    /// Create point light
    pub fn point(position: Point, color: Color, intensity: f32) -> Self {
        Self {
            light_type: LightType::Point,
            position,
            color,
            intensity,
            att_constant: 1.0,
            att_linear: 0.09,
            att_quadratic: 0.032,
            ..Default::default()
        }
    }

    /// Create spot light
    pub fn spot(
        position: Point,
        direction: Vector,
        color: Color,
        intensity: f32,
        inner_angle: f32,
        outer_angle: f32,
    ) -> Self {
        Self {
            light_type: LightType::Spot,
            position,
            direction: direction.normalized(),
            color,
            intensity,
            spot_inner_angle: inner_angle,
            spot_outer_angle: outer_angle,
            att_constant: 1.0,
            att_linear: 0.09,
            att_quadratic: 0.032,
            ..Default::default()
        }
    }

    /// Create headlight (directional from camera)
    pub fn headlight(color: Color, intensity: f32) -> Self {
        Self {
            light_type: LightType::Headlight,
            color,
            intensity,
            ..Default::default()
        }
    }

    /// Set position
    pub fn with_position(mut self, position: Point) -> Self {
        self.position = position;
        self
    }

    /// Set direction
    pub fn with_direction(mut self, direction: Vector) -> Self {
        self.direction = direction.normalized();
        self
    }

    /// Set color
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set intensity
    pub fn with_intensity(mut self, intensity: f32) -> Self {
        self.intensity = intensity;
        self
    }

    /// Set ambient coefficient
    pub fn with_ambient(mut self, ambient: f32) -> Self {
        self.ambient = ambient.clamp(0.0, 1.0);
        self
    }

    /// Set diffuse coefficient
    pub fn with_diffuse(mut self, diffuse: f32) -> Self {
        self.diffuse = diffuse.clamp(0.0, 1.0);
        self
    }

    /// Set specular coefficient
    pub fn with_specular(mut self, specular: f32) -> Self {
        self.specular = specular.clamp(0.0, 1.0);
        self
    }

    /// Set attenuation
    pub fn with_attenuation(mut self, constant: f32, linear: f32, quadratic: f32) -> Self {
        self.att_constant = constant;
        self.att_linear = linear;
        self.att_quadratic = quadratic;
        self
    }

    /// Set enabled
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set cast shadows
    pub fn with_cast_shadows(mut self, cast_shadows: bool) -> Self {
        self.cast_shadows = cast_shadows;
        self
    }

    /// Calculate attenuation factor for a given distance
    pub fn attenuation(&self, distance: f32) -> f32 {
        if self.light_type == LightType::Directional || self.light_type == LightType::Ambient {
            1.0
        } else {
            let d = distance.max(0.001);
            1.0 / (self.att_constant + self.att_linear * d + self.att_quadratic * d * d)
        }
    }

    /// Calculate spot light intensity factor
    pub fn spot_intensity(&self, light_dir: &Vector) -> f32 {
        if self.light_type != LightType::Spot {
            return 1.0;
        }

        let cos_angle = -light_dir.dot(&self.direction);
        let cos_inner = self.spot_inner_angle.to_radians().cos() as f64;
        let cos_outer = self.spot_outer_angle.to_radians().cos() as f64;

        if cos_angle >= cos_inner {
            1.0
        } else if cos_angle <= cos_outer {
            0.0
        } else {
            let t = (cos_angle - cos_outer) / (cos_inner - cos_outer);
            t.powf(self.spot_falloff as f64) as f32
        }
    }

    /// Get light direction at a given point
    pub fn direction_at(&self, point: &Point) -> Vector {
        match self.light_type {
            LightType::Directional | LightType::Headlight => self.direction.normalized(),
            LightType::Point | LightType::Spot => {
                let dir = Vector::new(
                    point.x - self.position.x,
                    point.y - self.position.y,
                    point.z - self.position.z,
                );
                dir.normalized()
            }
            LightType::Ambient => Vector::new(0.0, 0.0, 0.0),
        }
    }

    /// Calculate distance from light to point (for point/spot lights)
    pub fn distance_to(&self, point: &Point) -> f32 {
        match self.light_type {
            LightType::Point | LightType::Spot => {
                let dx = point.x - self.position.x;
                let dy = point.y - self.position.y;
                let dz = point.z - self.position.z;
                (dx * dx + dy * dy + dz * dz).sqrt() as f32
            }
            _ => 0.0,
        }
    }
}

impl Default for Light {
    fn default() -> Self {
        Self::new(LightType::Directional)
    }
}

/// Lighting model for calculating illumination
#[derive(Debug, Clone)]
pub struct LightingModel {
    /// Lights in the scene
    pub lights: Vec<Light>,
    /// Global ambient light
    pub global_ambient: Color,
    /// Use lighting flag
    pub enabled: bool,
    /// Two-sided lighting
    pub two_sided: bool,
    /// Local viewer (for specular)
    pub local_viewer: bool,
    /// Separate specular
    pub separate_specular: bool,
}

impl LightingModel {
    /// Create a new lighting model
    pub fn new() -> Self {
        Self {
            lights: Vec::new(),
            global_ambient: Color::from_rgb(0.1, 0.1, 0.1),
            enabled: true,
            two_sided: true,
            local_viewer: false,
            separate_specular: false,
        }
    }

    /// Add a light
    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    /// Remove a light
    pub fn remove_light(&mut self, index: usize) {
        if index < self.lights.len() {
            self.lights.remove(index);
        }
    }

    /// Clear all lights
    pub fn clear_lights(&mut self) {
        self.lights.clear();
    }

    /// Get light count
    pub fn light_count(&self) -> usize {
        self.lights.len()
    }

    /// Set global ambient
    pub fn with_global_ambient(mut self, color: Color) -> Self {
        self.global_ambient = color;
        self
    }

    /// Set enabled
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set two-sided lighting
    pub fn with_two_sided(mut self, two_sided: bool) -> Self {
        self.two_sided = two_sided;
        self
    }

    /// Calculate lighting at a point
    pub fn calculate_lighting(
        &self,
        position: &Point,
        normal: &Vector,
        view_dir: &Vector,
        material: &MaterialLighting,
    ) -> Color {
        if !self.enabled {
            return material.diffuse;
        }

        let mut final_color = Color::new(
            self.global_ambient.r * material.ambient.r,
            self.global_ambient.g * material.ambient.g,
            self.global_ambient.b * material.ambient.b,
            material.diffuse.a,
        );

        let n = if self.two_sided {
            let n_dot_v = normal.dot(view_dir);
            if n_dot_v < 0.0 {
                Vector::new(-normal.x, -normal.y, -normal.z)
            } else {
                *normal
            }
        } else {
            *normal
        };

        for light in &self.lights {
            if !light.enabled {
                continue;
            }

            let light_color =
                self.calculate_light_contribution(position, &n, view_dir, light, material);

            final_color = Color::new(
                (final_color.r + light_color.r).min(1.0),
                (final_color.g + light_color.g).min(1.0),
                (final_color.b + light_color.b).min(1.0),
                final_color.a,
            );
        }

        final_color
    }

    /// Calculate contribution from a single light
    fn calculate_light_contribution(
        &self,
        position: &Point,
        normal: &Vector,
        view_dir: &Vector,
        light: &Light,
        material: &MaterialLighting,
    ) -> Color {
        match light.light_type {
            LightType::Ambient => Color::new(
                light.color.r * material.ambient.r * light.ambient * light.intensity,
                light.color.g * material.ambient.g * light.ambient * light.intensity,
                light.color.b * material.ambient.b * light.ambient * light.intensity,
                1.0,
            ),
            _ => {
                let light_dir = light.direction_at(position);
                let distance = light.distance_to(position);
                let attenuation = light.attenuation(distance);
                let spot_factor = light.spot_intensity(&light_dir);

                // Diffuse
                let n_dot_l = normal.dot(&light_dir).max(0.0);
                let diffuse = Color::new(
                    light.color.r
                        * material.diffuse.r
                        * n_dot_l as f32
                        * light.diffuse
                        * light.intensity
                        * attenuation
                        * spot_factor,
                    light.color.g
                        * material.diffuse.g
                        * n_dot_l as f32
                        * light.diffuse
                        * light.intensity
                        * attenuation
                        * spot_factor,
                    light.color.b
                        * material.diffuse.b
                        * n_dot_l as f32
                        * light.diffuse
                        * light.intensity
                        * attenuation
                        * spot_factor,
                    1.0,
                );

                // Specular (Blinn-Phong)
                let half_dir = if self.local_viewer {
                    let view = Vector::new(
                        view_dir.x - light_dir.x,
                        view_dir.y - light_dir.y,
                        view_dir.z - light_dir.z,
                    )
                    .normalized();
                    view
                } else {
                    Vector::new(-light_dir.x, -light_dir.y, -light_dir.z).normalized()
                };

                let n_dot_h = normal.dot(&half_dir).max(0.0);
                let specular_factor = n_dot_h.powf(material.shininess as f64) as f32;
                let specular = Color::new(
                    light.color.r
                        * material.specular.r
                        * specular_factor
                        * light.specular
                        * light.intensity
                        * attenuation
                        * spot_factor,
                    light.color.g
                        * material.specular.g
                        * specular_factor
                        * light.specular
                        * light.intensity
                        * attenuation
                        * spot_factor,
                    light.color.b
                        * material.specular.b
                        * specular_factor
                        * light.specular
                        * light.intensity
                        * attenuation
                        * spot_factor,
                    1.0,
                );

                Color::new(
                    (diffuse.r + specular.r).min(1.0),
                    (diffuse.g + specular.g).min(1.0),
                    (diffuse.b + specular.b).min(1.0),
                    1.0,
                )
            }
        }
    }
}

impl Default for LightingModel {
    fn default() -> Self {
        Self::new()
    }
}

/// Material properties for lighting calculations
#[derive(Debug, Clone, PartialEq)]
pub struct MaterialLighting {
    /// Ambient color
    pub ambient: Color,
    /// Diffuse color
    pub diffuse: Color,
    /// Specular color
    pub specular: Color,
    /// Emissive color
    pub emissive: Color,
    /// Shininess (specular exponent)
    pub shininess: f32,
}

impl MaterialLighting {
    /// Create a new material
    pub fn new() -> Self {
        Self {
            ambient: Color::from_rgb(0.2, 0.2, 0.2),
            diffuse: Color::from_rgb(0.8, 0.8, 0.8),
            specular: Color::from_rgb(0.5, 0.5, 0.5),
            emissive: Color::black(),
            shininess: 32.0,
        }
    }

    /// Create material with colors
    pub fn with_colors(ambient: Color, diffuse: Color, specular: Color, emissive: Color) -> Self {
        Self {
            ambient,
            diffuse,
            specular,
            emissive,
            shininess: 32.0,
        }
    }

    /// Set shininess
    pub fn with_shininess(mut self, shininess: f32) -> Self {
        self.shininess = shininess.clamp(0.0, 128.0);
        self
    }
}

impl Default for MaterialLighting {
    fn default() -> Self {
        Self::new()
    }
}

/// Predefined lighting setups
pub struct LightingPresets;

impl LightingPresets {
    /// Default lighting (directional + ambient)
    pub fn default() -> LightingModel {
        let mut model = LightingModel::new();
        model.add_light(Light::directional(
            Vector::new(-1.0, -1.0, -1.0),
            Color::white(),
            1.0,
        ));
        model.add_light(Light::ambient(Color::white(), 0.3));
        model
    }

    /// Studio lighting (three-point lighting)
    pub fn studio() -> LightingModel {
        let mut model = LightingModel::new();
        // Key light
        model.add_light(Light::directional(
            Vector::new(-1.0, -0.5, -1.0),
            Color::from_rgb(1.0, 0.95, 0.9),
            1.0,
        ));
        // Fill light
        model.add_light(Light::directional(
            Vector::new(1.0, -0.3, -0.5),
            Color::from_rgb(0.6, 0.7, 0.8),
            0.5,
        ));
        // Rim light
        model.add_light(Light::directional(
            Vector::new(0.0, 1.0, -1.0),
            Color::from_rgb(0.8, 0.85, 1.0),
            0.7,
        ));
        model
    }

    /// Outdoor lighting (sun + sky)
    pub fn outdoor() -> LightingModel {
        let mut model = LightingModel::new();
        model.add_light(Light::directional(
            Vector::new(-0.5, -1.0, -0.3),
            Color::from_rgb(1.0, 0.98, 0.95),
            1.2,
        ));
        model.add_light(Light::ambient(Color::from_rgb(0.4, 0.5, 0.7), 0.4));
        model
    }

    /// Indoor lighting (warm)
    pub fn indoor() -> LightingModel {
        let mut model = LightingModel::new();
        model.add_light(Light::point(
            Point::new(5.0, 5.0, 5.0),
            Color::from_rgb(1.0, 0.9, 0.8),
            0.8,
        ));
        model.add_light(Light::ambient(Color::from_rgb(0.3, 0.25, 0.2), 0.3));
        model
    }

    /// Night lighting (cool)
    pub fn night() -> LightingModel {
        let mut model = LightingModel::new();
        model.add_light(Light::directional(
            Vector::new(-0.3, -1.0, -0.5),
            Color::from_rgb(0.4, 0.5, 0.8),
            0.5,
        ));
        model.add_light(Light::ambient(Color::from_rgb(0.1, 0.15, 0.3), 0.2));
        model
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_light_creation() {
        let light = Light::new(LightType::Point);
        assert_eq!(light.light_type, LightType::Point);
        assert!(light.enabled);
    }

    #[test]
    fn test_directional_light() {
        let light = Light::directional(Vector::new(0.0, -1.0, 0.0), Color::white(), 1.0);
        assert_eq!(light.light_type, LightType::Directional);
        assert_eq!(light.intensity, 1.0);
    }

    #[test]
    fn test_point_light() {
        let light = Light::point(Point::new(1.0, 2.0, 3.0), Color::red(), 0.8);
        assert_eq!(light.light_type, LightType::Point);
        assert_eq!(light.position.x, 1.0);
    }

    #[test]
    fn test_spot_light() {
        let light = Light::spot(
            Point::new(0.0, 5.0, 0.0),
            Vector::new(0.0, -1.0, 0.0),
            Color::white(),
            1.0,
            30.0,
            45.0,
        );
        assert_eq!(light.light_type, LightType::Spot);
        assert_eq!(light.spot_inner_angle, 30.0);
        assert_eq!(light.spot_outer_angle, 45.0);
    }

    #[test]
    fn test_light_attenuation() {
        let light = Light::point(Point::new(0.0, 0.0, 0.0), Color::white(), 1.0);
        let att = light.attenuation(10.0);
        assert!(att < 1.0);
        assert!(att > 0.0);
    }

    #[test]
    fn test_lighting_model() {
        let mut model = LightingModel::new();
        model.add_light(Light::directional(
            Vector::new(0.0, -1.0, 0.0),
            Color::white(),
            1.0,
        ));
        assert_eq!(model.light_count(), 1);
    }

    #[test]
    fn test_material_lighting() {
        let material = MaterialLighting::new().with_shininess(64.0);
        assert_eq!(material.shininess, 64.0);
    }

    #[test]
    fn test_lighting_presets() {
        let default = LightingPresets::default();
        assert!(default.light_count() > 0);

        let studio = LightingPresets::studio();
        assert!(studio.light_count() >= 3);

        let outdoor = LightingPresets::outdoor();
        assert!(outdoor.light_count() > 0);
    }
}
