# Cargo sub-command for generate ROS projects for the Rust client library

## Usage

1. Generate a config `config.yaml` file in the catkin workspace folder like this:

```yaml
# define a name
project_name: "navigation-robot"

# define a version
version: 0.0.1

# define the packages names
nodes:
   - listener
   - talker
   - node1
   - node2
```

2. Build the program

`cargo build --release`

3. Put the executable in your cargo installation

One way is:

`cp path-to-this-repo/target/release/cargo-ros-project-generator ~/.cargo/bin`

4. Run the executable like a cargo sub-command in the `catkin` workspace with the `config.yaml` file in it

`cargo ros-project-generator`
