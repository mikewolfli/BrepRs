//! Unit tests for BrepRs modeling module

use breprs::modeling::*;
use breprs::topology::*;

#[test]
fn test_primitive_creation() {
    // Test box creation
    let box_shape = Primitives::create_box(1.0, 1.0, 1.0);
    assert_eq!(box_shape.shape_type(), ShapeType::Solid);
    
    // Test sphere creation
    let sphere_shape = Primitives::create_sphere(1.0);
    assert_eq!(sphere_shape.shape_type(), ShapeType::Solid);
    
    // Test cylinder creation
    let cylinder_shape = Primitives::create_cylinder(1.0, 2.0);
    assert_eq!(cylinder_shape.shape_type(), ShapeType::Solid);
}

#[test]
fn test_boolean_operations() {
    // Test boolean operations
    let box1 = Primitives::create_box(1.0, 1.0, 1.0);
    let box2 = Primitives::create_box(1.0, 1.0, 1.0);
    
    let boolean = BooleanOperations::new();
    
    // Test fuse operation
    let fused = boolean.fuse(&box1, &box2);
    assert_eq!(fused.shape_type(), ShapeType::Solid);
    
    // Test cut operation
    let cut = boolean.cut(&box1, &box2);
    assert_eq!(cut.shape_type(), ShapeType::Solid);
    
    // Test common operation
    let common = boolean.common(&box1, &box2);
    assert_eq!(common.shape_type(), ShapeType::Solid);
}

#[test]
fn test_fillet_chamfer() {
    // Test fillet and chamfer operations
    let box_shape = Primitives::create_box(1.0, 1.0, 1.0);
    let fillet_chamfer = FilletChamfer::new();
    
    // Test edge filleting
    let filleted = fillet_chamfer.fillet(&box_shape, 0.1);
    assert_eq!(filleted.shape_type(), ShapeType::Solid);
    
    // Test face chamfering
    let chamfered = fillet_chamfer.chamfer(&box_shape, 0.1);
    assert_eq!(chamfered.shape_type(), ShapeType::Solid);
}

#[test]
fn test_pcb_modeling() {
    // Test PCB board creation
    let pcb = PcbBoard::two_layer_pcb("TestPCB", 10.0, 10.0);
    assert_eq!(pcb.name, "TestPCB");
    assert_eq!(pcb.width, 10.0);
    assert_eq!(pcb.height, 10.0);
    assert_eq!(pcb.layers.len(), 3); // Top, Dielectric, Bottom
    
    // Test PCB to solid conversion
    let pcb_solid = pcb.to_solid();
    assert_eq!(pcb_solid.shape_type(), ShapeType::Solid);
    
    // Test total thickness calculation
    let thickness = pcb.total_thickness();
    assert!(thickness > 0.0);
}

#[test]
fn test_board_components() {
    // Test resistor creation
    let resistor = BoardComponent::resistor("Resistor1", Point::origin(), 100.0, 0.25);
    assert_eq!(resistor.name, "Resistor1");
    assert_eq!(resistor.component_type, ComponentType::Resistor);
    
    // Test capacitor creation
    let capacitor = BoardComponent::capacitor("Capacitor1", Point::origin(), 1e-6, 25.0);
    assert_eq!(capacitor.name, "Capacitor1");
    assert_eq!(capacitor.component_type, ComponentType::Capacitor);
    
    // Test component to solid conversion
    let resistor_solid = resistor.to_solid();
    assert_eq!(resistor_solid.shape_type(), ShapeType::Solid);
    
    let capacitor_solid = capacitor.to_solid();
    assert_eq!(capacitor_solid.shape_type(), ShapeType::Solid);
}

#[test]
fn test_connectors() {
    // Test header connector creation
    let header = Connector::header("Header1", Point::origin(), 4, 0.00254); // 4 pins, 0.00254 pitch
    assert_eq!(header.name, "Header1");
    assert_eq!(header.connector_type, ConnectorType::Header);
    assert_eq!(header.pin_count, 4);
    
    // Test USB connector creation
    let usb = Connector::usb("USB1", Point::origin());
    assert_eq!(usb.name, "USB1");
    assert_eq!(usb.connector_type, ConnectorType::USB);
    
    // Test connector to solid conversion
    let header_solid = header.to_solid();
    assert_eq!(header_solid.shape_type(), ShapeType::Solid);
    
    let usb_solid = usb.to_solid();
    assert_eq!(usb_solid.shape_type(), ShapeType::Solid);
}

#[test]
fn test_sensors() {
    // Test temperature sensor creation
    let temp_sensor = Sensor::temperature("TempSensor1", Point::origin());
    assert_eq!(temp_sensor.name, "TempSensor1");
    assert_eq!(temp_sensor.sensor_type, SensorType::Temperature);
    
    // Test light sensor creation
    let light_sensor = Sensor::light("LightSensor1", Point::origin());
    assert_eq!(light_sensor.name, "LightSensor1");
    assert_eq!(light_sensor.sensor_type, SensorType::Light);
    
    // Test sensor to solid conversion
    let temp_sensor_solid = temp_sensor.to_solid();
    assert_eq!(temp_sensor_solid.shape_type(), ShapeType::Solid);
    
    let light_sensor_solid = light_sensor.to_solid();
    assert_eq!(light_sensor_solid.shape_type(), ShapeType::Solid);
}

#[test]
fn test_electronic_device_structure() {
    // Test simple IC creation
    let ic = ElectronicDeviceStructure::simple_ic("TestIC", 1.0, 1.0, 0.1);
    assert_eq!(ic.name, "TestIC");
    assert_eq!(ic.width, 1.0);
    assert_eq!(ic.length, 1.0);
    assert_eq!(ic.height, 0.1);
    assert_eq!(ic.layers.len(), 4); // Substrate, Oxide, Metal, Passivation
    
    // Test IC to solid conversion
    let ic_solid = ic.to_solid();
    assert_eq!(ic_solid.shape_type(), ShapeType::Solid);
    
    // Test total thickness calculation
    let thickness = ic.total_thickness();
    assert!(thickness > 0.0);
}

#[test]
fn test_chip_structure() {
    // Test simple chip creation
    let chip = ChipStructure::simple_chip("TestChip", 0.5, 0.5, 0.05);
    assert_eq!(chip.name, "TestChip");
    assert_eq!(chip.bond_wires.len(), 3);
    
    // Test chip to solid conversion
    let chip_solid = chip.to_solid();
    assert_eq!(chip_solid.shape_type(), ShapeType::Solid);
}

#[test]
fn test_pcb_internal_structure() {
    // Test 4-layer PCB creation
    let pcb_internal = PcbInternalStructure::four_layer_pcb("TestPCBInternal", 10.0, 10.0);
    assert_eq!(pcb_internal.name, "TestPCBInternal");
    assert_eq!(pcb_internal.width, 10.0);
    assert_eq!(pcb_internal.length, 10.0);
    assert_eq!(pcb_internal.layers.len(), 5); // Top, Dielectric1, Internal1, Dielectric2, Bottom
    assert_eq!(pcb_internal.vias.len(), 1);
    
    // Test PCB internal structure to solid conversion
    let pcb_internal_solid = pcb_internal.to_solid();
    assert_eq!(pcb_internal_solid.shape_type(), ShapeType::Solid);
}

#[test]
fn test_library_compatibility() {
    // Test library manager creation
    let manager = LibraryManager::default();
    assert!(!manager.electrical_libraries.is_empty());
    assert!(!manager.logic_libraries.is_empty());
    assert!(!manager.chip_libraries.is_empty());
    
    // Test getting components from libraries
    let resistor = manager.get_electrical_component("Standard_Electrical", "Resistor_100.00Ohm_0.25W");
    assert!(resistor.is_some());
    
    let and_gate = manager.get_logic_component("Standard_Logic", "AND_Gate");
    assert!(and_gate.is_some());
    
    let microcontroller = manager.get_chip_component("Standard_Chips", "Microcontroller");
    assert!(microcontroller.is_some());
    
    // Test component to solid conversion
    if let Some(resistor) = resistor {
        let resistor_solid = resistor.to_solid();
        assert_eq!(resistor_solid.shape_type(), ShapeType::Solid);
    }
}