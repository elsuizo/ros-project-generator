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

2. Build the program

`cargo build --release`

3. Put the executable in your `PATH` variable

One way is:

`sudo ln -s /home/your_user/path_to_this_repo/ros-project-generator/target/release/ros-project-generator /usr/local/bin`

4. Run the executable in the `catkin` workspace with the `config.yaml` file in it
