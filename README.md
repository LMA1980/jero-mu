# jero-mu -- a distributed solution

## Goal:

- Learn Rust.
- See how the proposed solution to build a modern TUI with similar capability seen on GUI.
- See how I could integrate an OpenAPI client as a WASM component to try to simplify 
  deployment and normalize runtime between headless usage and TUI/GUI.
- Interface with Cluster API and Kubernetes API to help management.
- Features:
   - an About block Ratatui's Widget
     - Centralized to the terminal main area
     - Display 
        - the package name
        - the package version
        - the binary name

