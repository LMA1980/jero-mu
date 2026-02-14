use neuron_rs::{Neuron, NeuronBuilder};

fn main() {
    let builder: NeuronBuilder = NeuronBuilder::new();
    let neuron: Neuron = builder
        .with_id("minimal neuron")
        .with_tracking_uuid_v8(1, "minimal neuron example")
        .build()
        .expect("Runtime error");
    dbg!(&neuron);
    println!("neuron id: {0}", &neuron.id());
}
