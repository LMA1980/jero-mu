// tests/minimal_neuron_test.rs
use neuron_rs::{Neuron, NeuronBuilder};
// use uuid::Uuid;

#[test]
fn test_minimal_neuron_initialization() {
    // 1. Arrange
    let expected_id = "test-neuron-01";
    let builder = NeuronBuilder::new();

    // 2. Act
    // We want our builder to return a Result to handle the "Zero-Trust" 
    // validation errors we'll add later, rather than just panicking.
    let neuron_result = builder
        .with_id(expected_id)
        .with_tracking_uuid_v8(1,"Default Tracking")
        .build();
        //.expect("Runtime Error");

    // 3. Assert
    assert!(neuron_result.is_ok(), "Neuron should build successfully with a valid ID");
    
    let neuron: Neuron = neuron_result.unwrap();
    assert_eq!(neuron.id(), expected_id);
}

#[test]
fn test_neuron_id_is_shared_and_immutable() {
    let expected_id = "neuron-01";
    let neuron = NeuronBuilder::new()
        .with_id(expected_id)
        .with_tracking_uuid_v8(1,"Default Tracking")
        .build()
        .expect("Build failed");

    // We want to ensure the ID is accessible as a string slice
    assert_eq!(neuron.id(), expected_id);
}

#[test]
fn test_neuron_uuid_v8_integration() {
    let epoch_1: u64 = 1024;
    let epoch_2: u64 = 1025;
    let config_data = "some_secure_config_payload";
    
    // In our implementation, the builder will handle 
    // packing the epoch and hash into the UUID v8
    let neuron = NeuronBuilder::new()
        .with_id("uuid-neuron")
        .with_tracking_uuid_v8(epoch_1, config_data)
        .build()
        .expect("Build failed");

    let tracking = neuron.tracking();
    
    // Verify it is a valid UUID
    assert_eq!(tracking.get_version_num(), 8);
    
    // Verify sorting (Epoch 1025 should be > 1024)
    let newer_neuron = NeuronBuilder::new()
        .with_id("uuid-neuron")
        .with_tracking_uuid_v8(epoch_2, config_data)
        .build()
        .unwrap();
        
    assert!(newer_neuron.tracking() > neuron.tracking());
}

