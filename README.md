# WorkSpace eXplorer

`wsx` is a command line utility for quickly cloning (and decloning) Git repositories
into a local directory called a _workspace_.

### Workspaces

Simply defined, a _working set_ is the set of all repositories you're able to work
on at any given time. Even more simply, your _workspace_ is just the local directory
where you keep your Git repositories.

The principle behind `wsx` is that your Git workspace should only consist of your
_working set_. This reduces unnecessary noise as repositories accumulate in your
workspace over time and improves indexing performance of your development tools.
Adhering to this principle involves frequently cloning and deleting repositories
from your workspace, which is exactly what `wsx` is designed to automate.

When repositories are dropped from your workspace, they are cached locally so
restoring them later can be done in an instant.

### Keep your working set clean

```sh
# Clone a repository into the default workspace
wsx clone github.com/cilki/wsx

# The repository's local path always reflects the remote path for identifiability
cd ~/workspace/github.com/cilki/wsx

# Remove everything I'm not currently working on from the workspace (any repo
# that has no unstaged or unpushed changes).
wsx drop

# Or, you can also use prefix patterns to select repositories to drop
wsx drop github.com/cilki

# If there were unstaged changes above, you'll have to force it
wsx drop github.com/cilki --force

# When you need to work on a repository again, it's restored from cache which is
# super fast!
wsx clone github.com/cilki/wsx
```

### Simple configuration

`wsx` attempts to read a configuration file from `~/.config/wsx.toml`. For example:

```toml
# Define any number of workspaces
[[workspace]]
name = "personal"
path = "/home/user/workspace"

[[workspace]]
name = "work"
path = "/home/worker/workspace"
```

