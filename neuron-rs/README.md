# Neuron
## A compute unit to be grouped by Neucleus.

A Neuron, is a compute unit with a fixed lifecycle. 

The idea is to group similar unit within given neucleus. Which are part of a region, regions would 
be inter-linked. Compute could be  off-loaded to other regions. Or simply be used to provide a 
consistent framework for the `ctl` commands used to interact with modern distributed environment.

The idea is to provide a building block to integrate your view of a neuron within a brain where the
basal ganglia take the role of your public interface.

### Milestone 1.0

Provide a truly dynamic _with_ building blocks with the goal of trully including only what is
required. To be able to properly implement this, I feel I need to be able to provide dependencies, 
to build a dependencies tree and enforce, hopefully, at compile time, to prevent runtime surprises.

Provide a minimal contructor for the mandatory parameters. Dependency injection is not excluded but
may not be part of this milestone.

Due to the distributed natures. I feel the framework should make it as natural as possible to
integrate, interface with timebased databases using opentelemetry framework: like OpenSearch and other solution.

### Rules Compendium

1. Handle one thing as efficiently as possible and nothing else.
2. There is no limit on the behaviour your Neuron can take:
    - In a compute model, to distinguish ourselves from a real life brain, Neuron could be:
        - Filters
        - Actors
        - Brokers
        - Sink
        - Drain
    - Or a mix of the above for vertical integration.
    - You only _*require*_ to support the core and the building block framework.
3. Follow the lifecycle of the distributed nature of the solution.
    - With proper dependencies declaration, one could delay compilation of elements of the
    distributed system until requirements are met.
∞. Be kind to your neurons.
