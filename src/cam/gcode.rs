use crate::foundation::types::StandardReal;
use super::toolpath::{Toolpath, ToolpathPoint};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GCodeUnit {
    Metric,
    Imperial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GCodePlane {
    XY,
    XZ,
    YZ,
}

#[derive(Debug, Clone)]
pub struct GCodeConfig {
    pub unit: GCodeUnit,
    pub plane: GCodePlane,
    pub decimal_places: usize,
    pub use_g43: bool,
    pub use_g54: bool,
    pub use_coolant: bool,
    pub program_number: i32,
}

impl Default for GCodeConfig {
    fn default() -> Self {
        Self {
            unit: GCodeUnit::Metric,
            plane: GCodePlane::XY,
            decimal_places: 3,
            use_g43: true,
            use_g54: true,
            use_coolant: true,
            program_number: 1000,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GCodeBlock {
    pub line_number: Option<i32>,
    pub commands: Vec<GCodeCommand>,
    pub comment: Option<String>,
}

impl GCodeBlock {
    pub fn new() -> Self {
        Self {
            line_number: None,
            commands: Vec::new(),
            comment: None,
        }
    }

    pub fn with_line_number(mut self, num: i32) -> Self {
        self.line_number = Some(num);
        self
    }

    pub fn add_command(mut self, cmd: GCodeCommand) -> Self {
        self.commands.push(cmd);
        self
    }

    pub fn with_comment(mut self, comment: String) -> Self {
        self.comment = Some(comment);
        self
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();

        if let Some(num) = self.line_number {
            result.push_str(&format!("N{:04} ", num));
        }

        for cmd in &self.commands {
            result.push_str(&cmd.to_string());
            result.push(' ');
        }

        if let Some(comment) = &self.comment {
            result.push_str(&format!("({})", comment));
        }

        result.trim().to_string()
    }
}

impl Default for GCodeBlock {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum GCodeCommand {
    G00 { x: Option<StandardReal>, y: Option<StandardReal>, z: Option<StandardReal> },
    G01 { x: Option<StandardReal>, y: Option<StandardReal>, z: Option<StandardReal>, f: Option<StandardReal> },
    G02 { x: StandardReal, y: StandardReal, i: StandardReal, j: StandardReal, f: Option<StandardReal> },
    G03 { x: StandardReal, y: StandardReal, i: StandardReal, j: StandardReal, f: Option<StandardReal> },
    G17,
    G18,
    G19,
    G20,
    G21,
    G40,
    G41,
    G42,
    G43 { h: i32 },
    G49,
    G54,
    G80,
    G81 { x: StandardReal, y: StandardReal, z: StandardReal, r: StandardReal, f: StandardReal },
    G90,
    G91,
    M03 { s: StandardReal },
    M05,
    M08,
    M09,
    M30,
    T { t: i32 },
    M06,
}

impl GCodeCommand {
    pub fn to_string(&self) -> String {
        match self {
            GCodeCommand::G00 { x, y, z } => {
                let mut result = "G00".to_string();
                if let Some(x) = x { result.push_str(&format!(" X{:.3}", x)); }
                if let Some(y) = y { result.push_str(&format!(" Y{:.3}", y)); }
                if let Some(z) = z { result.push_str(&format!(" Z{:.3}", z)); }
                result
            }
            GCodeCommand::G01 { x, y, z, f } => {
                let mut result = "G01".to_string();
                if let Some(x) = x { result.push_str(&format!(" X{:.3}", x)); }
                if let Some(y) = y { result.push_str(&format!(" Y{:.3}", y)); }
                if let Some(z) = z { result.push_str(&format!(" Z{:.3}", z)); }
                if let Some(f) = f { result.push_str(&format!(" F{:.0}", f)); }
                result
            }
            GCodeCommand::G02 { x, y, i, j, f } => {
                let mut result = format!("G02 X{:.3} Y{:.3} I{:.3} J{:.3}", x, y, i, j);
                if let Some(f) = f { result.push_str(&format!(" F{:.0}", f)); }
                result
            }
            GCodeCommand::G03 { x, y, i, j, f } => {
                let mut result = format!("G03 X{:.3} Y{:.3} I{:.3} J{:.3}", x, y, i, j);
                if let Some(f) = f { result.push_str(&format!(" F{:.0}", f)); }
                result
            }
            GCodeCommand::G17 => "G17".to_string(),
            GCodeCommand::G18 => "G18".to_string(),
            GCodeCommand::G19 => "G19".to_string(),
            GCodeCommand::G20 => "G20".to_string(),
            GCodeCommand::G21 => "G21".to_string(),
            GCodeCommand::G40 => "G40".to_string(),
            GCodeCommand::G41 => "G41".to_string(),
            GCodeCommand::G42 => "G42".to_string(),
            GCodeCommand::G43 { h } => format!("G43 H{:02}", h),
            GCodeCommand::G49 => "G49".to_string(),
            GCodeCommand::G54 => "G54".to_string(),
            GCodeCommand::G80 => "G80".to_string(),
            GCodeCommand::G81 { x, y, z, r, f } => {
                format!("G81 X{:.3} Y{:.3} Z{:.3} R{:.3} F{:.0}", x, y, z, r, f)
            }
            GCodeCommand::G90 => "G90".to_string(),
            GCodeCommand::G91 => "G91".to_string(),
            GCodeCommand::M03 { s } => format!("M03 S{:.0}", s),
            GCodeCommand::M05 => "M05".to_string(),
            GCodeCommand::M08 => "M08".to_string(),
            GCodeCommand::M09 => "M09".to_string(),
            GCodeCommand::M30 => "M30".to_string(),
            GCodeCommand::T { t } => format!("T{:02}", t),
            GCodeCommand::M06 => "M06".to_string(),
        }
    }
}

pub struct GCodeGenerator {
    config: GCodeConfig,
    line_number: i32,
    line_increment: i32,
}

impl GCodeGenerator {
    pub fn new(config: GCodeConfig) -> Self {
        Self {
            config,
            line_number: 10,
            line_increment: 10,
        }
    }

    pub fn generate(&mut self, toolpath: &Toolpath, tool_number: i32) -> Vec<GCodeBlock> {
        let mut blocks = Vec::new();

        blocks.push(GCodeBlock::new()
            .with_comment(format!("Program: {}", toolpath.name)));

        blocks.push(GCodeBlock::new()
            .with_line_number(self.next_line())
            .add_command(GCodeCommand::T { t: tool_number })
            .add_command(GCodeCommand::M06));

        match self.config.unit {
            GCodeUnit::Metric => {
                blocks.push(GCodeBlock::new()
                    .with_line_number(self.next_line())
                    .add_command(GCodeCommand::G21));
            }
            GCodeUnit::Imperial => {
                blocks.push(GCodeBlock::new()
                    .with_line_number(self.next_line())
                    .add_command(GCodeCommand::G20));
            }
        }

        blocks.push(GCodeBlock::new()
            .with_line_number(self.next_line())
            .add_command(GCodeCommand::G90));

        blocks.push(GCodeBlock::new()
            .with_line_number(self.next_line())
            .add_command(GCodeCommand::G17));

        if self.config.use_g54 {
            blocks.push(GCodeBlock::new()
                .with_line_number(self.next_line())
                .add_command(GCodeCommand::G54));
        }

        if self.config.use_g43 {
            blocks.push(GCodeBlock::new()
                .with_line_number(self.next_line())
                .add_command(GCodeCommand::G43 { h: tool_number }));
        }

        blocks.push(GCodeBlock::new()
            .with_line_number(self.next_line())
            .add_command(GCodeCommand::G40));

        blocks.push(GCodeBlock::new()
            .with_line_number(self.next_line())
            .add_command(GCodeCommand::M03 { s: toolpath.spindle_speed }));

        if self.config.use_coolant {
            blocks.push(GCodeBlock::new()
                .with_line_number(self.next_line())
                .add_command(GCodeCommand::M08));
        }

        for (i, point) in toolpath.points.iter().enumerate() {
            let block = self.point_to_gcode(point, i == 0);
            blocks.push(block.with_line_number(self.next_line()));
        }

        if self.config.use_coolant {
            blocks.push(GCodeBlock::new()
                .with_line_number(self.next_line())
                .add_command(GCodeCommand::M09));
        }

        blocks.push(GCodeBlock::new()
            .with_line_number(self.next_line())
            .add_command(GCodeCommand::M05));

        if self.config.use_g43 {
            blocks.push(GCodeBlock::new()
                .with_line_number(self.next_line())
                .add_command(GCodeCommand::G49));
        }

        blocks.push(GCodeBlock::new()
            .with_line_number(self.next_line())
            .add_command(GCodeCommand::G80));

        blocks.push(GCodeBlock::new()
            .with_line_number(self.next_line())
            .add_command(GCodeCommand::M30));

        blocks
    }

    fn point_to_gcode(&self, point: &ToolpathPoint, is_first: bool) -> GCodeBlock {
        let pos = &point.position;

        if point.is_rapid {
            GCodeBlock::new()
                .add_command(GCodeCommand::G00 {
                    x: Some(pos.x),
                    y: Some(pos.y),
                    z: Some(pos.z),
                })
        } else {
            GCodeBlock::new()
                .add_command(GCodeCommand::G01 {
                    x: Some(pos.x),
                    y: Some(pos.y),
                    z: Some(pos.z),
                    f: point.feed_rate.or(if is_first { Some(1000.0) } else { None }),
                })
        }
    }

    fn next_line(&mut self) -> i32 {
        let current = self.line_number;
        self.line_number += self.line_increment;
        current
    }

    pub fn generate_string(&mut self, toolpath: &Toolpath, tool_number: i32) -> String {
        let blocks = self.generate(toolpath, tool_number);
        blocks.iter()
            .map(|b| b.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

pub struct GCodeParser;

impl GCodeParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, gcode: &str) -> Result<Vec<GCodeBlock>, String> {
        let mut blocks = Vec::new();

        for line in gcode.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let block = self.parse_line(line)?;
            blocks.push(block);
        }

        Ok(blocks)
    }

    fn parse_line(&self, line: &str) -> Result<GCodeBlock, String> {
        let mut block = GCodeBlock::new();
        let mut remaining = line;

        if let Some(rest) = remaining.strip_prefix('N') {
            let num_end = rest.find(|c: char| !c.is_ascii_digit()).unwrap_or(rest.len());
            let num_str = &rest[..num_end];
            if let Ok(num) = num_str.parse::<i32>() {
                block.line_number = Some(num);
                remaining = &rest[num_end..];
            }
        }

        if let Some(start) = remaining.find('(') {
            if let Some(end) = remaining.find(')') {
                block.comment = Some(remaining[start + 1..end].to_string());
                remaining = &remaining[..start];
            }
        }

        remaining = remaining.trim();

        while !remaining.is_empty() {
            remaining = remaining.trim_start();
            if remaining.is_empty() {
                break;
            }

            let letter = remaining.chars().next().unwrap();
            remaining = &remaining[1..];

            let value_end = remaining.find(|c: char| c.is_ascii_alphabetic()).unwrap_or(remaining.len());
            let value_str = &remaining[..value_end];
            remaining = &remaining[value_end..];

            match letter {
                'G' | 'g' => {
                    if let Ok(code) = value_str.parse::<i32>() {
                        let cmd = self.parse_g_command(code, &mut remaining)?;
                        block.commands.push(cmd);
                    }
                }
                'M' | 'm' => {
                    if let Ok(code) = value_str.parse::<i32>() {
                        let cmd = self.parse_m_command(code, &mut remaining)?;
                        block.commands.push(cmd);
                    }
                }
                'T' | 't' => {
                    if let Ok(t) = value_str.parse::<i32>() {
                        block.commands.push(GCodeCommand::T { t });
                    }
                }
                _ => {}
            }
        }

        Ok(block)
    }

    fn parse_g_command(&self, code: i32, remaining: &mut &str) -> Result<GCodeCommand, String> {
        match code {
            0 => {
                let (x, y, z) = self.parse_coordinates(remaining);
                Ok(GCodeCommand::G00 { x, y, z })
            }
            1 => {
                let (x, y, z) = self.parse_coordinates(remaining);
                let f = self.parse_feed_rate(remaining);
                Ok(GCodeCommand::G01 { x, y, z, f })
            }
            17 => Ok(GCodeCommand::G17),
            18 => Ok(GCodeCommand::G18),
            19 => Ok(GCodeCommand::G19),
            20 => Ok(GCodeCommand::G20),
            21 => Ok(GCodeCommand::G21),
            40 => Ok(GCodeCommand::G40),
            41 => Ok(GCodeCommand::G41),
            42 => Ok(GCodeCommand::G42),
            43 => {
                let h = self.parse_h_value(remaining).unwrap_or(1);
                Ok(GCodeCommand::G43 { h })
            }
            49 => Ok(GCodeCommand::G49),
            54 => Ok(GCodeCommand::G54),
            80 => Ok(GCodeCommand::G80),
            90 => Ok(GCodeCommand::G90),
            91 => Ok(GCodeCommand::G91),
            _ => Err(format!("Unknown G code: {}", code)),
        }
    }

    fn parse_m_command(&self, code: i32, remaining: &mut &str) -> Result<GCodeCommand, String> {
        match code {
            3 => {
                let s = self.parse_s_value(remaining).unwrap_or(0.0);
                Ok(GCodeCommand::M03 { s })
            }
            5 => Ok(GCodeCommand::M05),
            6 => Ok(GCodeCommand::M06),
            8 => Ok(GCodeCommand::M08),
            9 => Ok(GCodeCommand::M09),
            30 => Ok(GCodeCommand::M30),
            _ => Err(format!("Unknown M code: {}", code)),
        }
    }

    fn parse_coordinates(&self, remaining: &mut &str) -> (Option<StandardReal>, Option<StandardReal>, Option<StandardReal>) {
        let mut x = None;
        let mut y = None;
        let mut z = None;

        let original = *remaining;
        let mut temp = original;

        while !temp.is_empty() {
            temp = temp.trim_start();
            if temp.is_empty() {
                break;
            }

            let letter = temp.chars().next().unwrap();
            if !matches!(letter, 'X' | 'Y' | 'Z' | 'x' | 'y' | 'z') {
                break;
            }
            temp = &temp[1..];

            let value_end = temp.find(|c: char| c.is_ascii_alphabetic()).unwrap_or(temp.len());
            let value_str = &temp[..value_end];
            temp = &temp[value_end..];

            if let Ok(val) = value_str.parse::<StandardReal>() {
                match letter.to_ascii_uppercase() {
                    'X' => x = Some(val),
                    'Y' => y = Some(val),
                    'Z' => z = Some(val),
                    _ => {}
                }
            }
        }

        *remaining = temp;
        (x, y, z)
    }

    fn parse_feed_rate(&self, remaining: &mut &str) -> Option<StandardReal> {
        let original = *remaining;
        if let Some(pos) = original.find('F') {
            let rest = &original[pos + 1..];
            let value_end = rest.find(|c: char| c.is_ascii_alphabetic()).unwrap_or(rest.len());
            let value_str = &rest[..value_end];
            if let Ok(val) = value_str.parse::<StandardReal>() {
                *remaining = &rest[value_end..];
                return Some(val);
            }
        }
        None
    }

    fn parse_s_value(&self, remaining: &mut &str) -> Option<StandardReal> {
        let original = *remaining;
        if let Some(pos) = original.find('S') {
            let rest = &original[pos + 1..];
            let value_end = rest.find(|c: char| c.is_ascii_alphabetic()).unwrap_or(rest.len());
            let value_str = &rest[..value_end];
            if let Ok(val) = value_str.parse::<StandardReal>() {
                *remaining = &rest[value_end..];
                return Some(val);
            }
        }
        None
    }

    fn parse_h_value(&self, remaining: &mut &str) -> Option<i32> {
        let original = *remaining;
        if let Some(pos) = original.find('H') {
            let rest = &original[pos + 1..];
            let value_end = rest.find(|c: char| c.is_ascii_alphabetic()).unwrap_or(rest.len());
            let value_str = &rest[..value_end];
            if let Ok(val) = value_str.parse::<i32>() {
                *remaining = &rest[value_end..];
                return Some(val);
            }
        }
        None
    }
}

impl Default for GCodeParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;

    #[test]
    fn test_gcode_command_to_string() {
        let cmd = GCodeCommand::G01 {
            x: Some(10.0),
            y: Some(20.0),
            z: None,
            f: Some(1000.0),
        };
        assert!(cmd.to_string().contains("G01"));
        assert!(cmd.to_string().contains("X10.000"));
    }

    #[test]
    fn test_gcode_block_to_string() {
        let block = GCodeBlock::new()
            .with_line_number(10)
            .add_command(GCodeCommand::G01 {
                x: Some(10.0),
                y: Some(20.0),
                z: None,
                f: Some(1000.0),
            })
            .with_comment("Move to position".to_string());

        let s = block.to_string();
        assert!(s.contains("N0010"));
        assert!(s.contains("G01"));
        assert!(s.contains("(Move to position)"));
    }

    #[test]
    fn test_gcode_generator() {
        let config = GCodeConfig::default();
        let mut generator = GCodeGenerator::new(config);

        let mut toolpath = Toolpath::new("Test".to_string(), super::super::toolpath::ToolpathType::Contour);
        toolpath.add_point(ToolpathPoint::new(Point::new(0.0, 0.0, 10.0)).rapid());
        toolpath.add_point(ToolpathPoint::new(Point::new(10.0, 10.0, 0.0)));

        let gcode = generator.generate(&toolpath, 1);
        assert!(!gcode.is_empty());
    }

    #[test]
    fn test_gcode_parser() {
        let parser = GCodeParser::new();
        let gcode = "N0010 G01 X10.0 Y20.0 F1000\nN0020 M03 S3000";

        let blocks = parser.parse(gcode).unwrap();
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].line_number, Some(10));
    }
}
