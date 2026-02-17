# Roadmap

Concentrating on the Neuron solution for now. The goal with Neuron is to provide a core framework to create
Distributed Application and provide opiniated solution to common issues faced with such application.

The Neuron library will not attempt to resolve authentication, authorization, tenancy and such, these types
of system challenges will rather be handled through other crates.

A neuron application is the smallest compute unit in the system: like a brain neuron is a specialized, 
electrically excitable nerve cell that acts as the fundamental, functional unit of the brain and nervous
system, responsible for receiving, processing, and transmitting information via electrical and chemical 
signals. A neuron application should remain as fundamental as possible and be responsible for handling
messages, events, or requests and produce a results.

## Milestone Neuron 0.1.0

Neuron is targetting to be a library to help produce Distributed Application.
- Provide a framework to write a distributed application that has some form of self-awareness
  + Has an instance name (implemented)
  + Has a static version (implemented)
  + Has a tracking number for the current runtime configuration (in-progress)
    - It use Uuid v8 where the first 48 bits are used for the EPOCH and the following 80 bits
      is a partial SHA512 hash value of the configuration
  + Use environment variable(s) and command-line parameters to initialise the runtime configuration (pending)
  + May include a key/value store backend service to complete the runtime configuration (pending)
    - Etcd support
    - a Secret store support 

## Milestone Neuron 0.2.0

Extend Neuron to provide distributed capacity
  + May register itself with a Service Discovery registry (pending)
  + May query the Service Discovery registry to integrate (pending)
    - A shared memory service (for shared application state)
    - A caching service (for frequently access data)
    - A messaging bus (for transaction, requests management)
    - A event/signal bus (for state changes)
