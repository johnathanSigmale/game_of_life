# Conway's Game of Life with Egui Interface

This project offers an implementation of Conway's Game of Life, utilizing [Egui](https://github.com/emilk/egui) for the user interface. It provides highly customizable parameters, including support for different species with configurable interactions.

## Features

- **Egui Integration**: Leverages the Egui library for a responsive and interactive graphical user interface.
- **Multiple Species Support**: Simulate various species with distinct behaviors and interactions.
- **Customizable Parameters**: Adjust settings such as grid size, simulation speed, and species interaction rules.

## Installation

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/johnathanSigmale/game_of_life.git
   cd game_of_life
   ```

2. **Build the Project**:

   Ensure you have [Rust](https://www.rust-lang.org/) installed. Then, build the project using Cargo:

   ```bash
   cargo build --release
   ```

3. **Run the Application**:

   ```bash
   cargo run --release
   ```

## Usage

- **Interactive Grid**: Click on cells to toggle their state (alive or dead).
- **Simulation Controls**:
  - **Start/Pause**: Begin or pause the simulation.
  - **Step**: Advance the simulation by one generation.
  - **Reset**: Clear the grid or reset to the initial configuration.
- **Settings**:
  - **Grid Size**: Modify the dimensions of the simulation grid.
  - **Simulation Speed**: Adjust how quickly generations progress.
  - **Species Configuration**: Define the number of species and set interaction rules between them.

## Contributing

Contributions are welcome! Feel free to fork the repository and submit pull requests with enhancements or bug fixes. This project is currently incomplete, i want to add a way of storing specific ecosystems, complexify the possible inter-species interractions, and alse better optimize the step function by using multithreading or just a better architecture. 

## License

This project is licensed under the Apache-2.0 License. See the [LICENSE](LICENSE) file for details.
