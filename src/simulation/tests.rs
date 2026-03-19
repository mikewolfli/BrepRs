#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_system_clone() {
        let system = crate::simulation::SimulationSystem::Physics;
        let cloned = system.clone();
        assert_eq!(system, cloned);
    }

    #[test]
    fn test_simulation_parameter_clone() {
        let param = crate::simulation::SimulationParameter::Float(3.14);
        let cloned = param.clone();
        assert_eq!(param, cloned);
    }
}
