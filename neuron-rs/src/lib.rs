use sha2::{Digest, Sha512};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct Neuron {
    id: Arc<str>,
    tracking: Arc<uuid::Uuid>,
}

impl Neuron {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn tracking(&self) -> &Uuid {
        &self.tracking
    }

    pub fn version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }
}

#[derive(Debug)]
pub struct NeuronBuilder {
    id: Option<Arc<str>>,
    tracking: Option<Arc<uuid::Uuid>>,
}

impl Default for NeuronBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl NeuronBuilder {
    // Initialize the builder with "None" values
    pub fn new() -> Self {
        Self {
            id: None,
            tracking: None,
        }
    }

    pub fn with_id(mut self, id: &str) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn with_tracking_uuid_v8(mut self, epoch: u64, data: &str) -> Self {
        let mut buf = [0u8; 16];

        // 1. Pack 48-bit Epoch (Bytes 0-5)
        let epoch_bytes = epoch.to_be_bytes();
        buf[0..6].copy_from_slice(&epoch_bytes[2..8]);

        // 2. Generate SHA-512 for high-entropy source
        let mut hasher = Sha512::new();
        hasher.update(data.as_bytes());
        let hash = hasher.finalize();

        // 3. Extract the remaining bytes (Bytes 6-15)
        // We fill the remaining 10 bytes with the start of the hash.
        // The Uuid::new_v8 call later will mask out the 6 bits
        // required for the Version (4) and Variant (2).
        buf[6..16].copy_from_slice(&hash[0..10]);

        // 4. Finalize the UUIDv8
        // This ensures the 128-bit number is a valid UUID while
        // preserving as much of our 74-bit hash as possible.
        self.tracking = Some(Uuid::new_v8(buf).into());
        self
    }

    pub fn build(self) -> Result<Neuron, String> {
        let id = self.id.ok_or("Identity required")?;
        let tracking = self.tracking.ok_or("V8 Bootstrap required")?;

        Ok(Neuron { id, tracking })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_build_a_neuron_with_identity() -> Result<(), String> {
        // 1. Arrange: Define what we want
        let expected_id = "neuron-01";
        let builder: NeuronBuilder = NeuronBuilder::new();

        // 2. Act: Try to build the neuron
        // We want a fluent API that allows setting identity
        let neuron = builder
            .with_id(expected_id)
            .with_tracking_uuid_v8(1, "Default Tracking")
            .build()
            .expect("Runtime error");

        // 3. Assert: Verify the result
        assert_eq!(neuron.id(), expected_id);
        Ok(())
    }

    #[test]
    fn test_neuron_defaults_to_crate_version() -> Result<(), String> {
        let neuron = NeuronBuilder::new()
            .with_id("version-test")
            .with_tracking_uuid_v8(1, "Default Tracking")
            .build()
            .expect("Build failed");

        // We expect the version string from Cargo.toml
        // e.g., "0.1.0"
        assert_eq!(neuron.version(), env!("CARGO_PKG_VERSION"));
        Ok(())
    }
}
