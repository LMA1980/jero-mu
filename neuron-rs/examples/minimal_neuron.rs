use neuron_rs::{NeuronBuilder, Neuron};

fn main() {
  let builder: NeuronBuilder = NeuronBuilder::new();
  let neuron: Neuron = builder
    .with_id("need.to.implement.a.default.id.provider")
    .build().expect("Runtime error");
  dbg!(&neuron);
  println!("neuron id: {0}", &neuron.id());
}
