# Cargo sub-command for generate ROS projects for the Rust client library (`rosrust`)

## Usage

1. Generate a config file `config.yaml` in the catkin workspace folder like this:

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

1. Install the cargo sub-command

`cargo install ros-project-generator`

2. Run the cargo sub-command in the `catkin` workspace with the `config.yaml` file in it

`cargo ros-project-generator`
