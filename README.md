# Project generator for the Rust client library

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

2. Run the program with the catkin workspace path as command input

`cargo run --release /home/user/path-to-catkin-workspace`
