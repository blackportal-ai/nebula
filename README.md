# Nebula

A package manager for machine learning datasets and models.

## Nebula CLI

### Commands

The Nebula CLI provides a set of commands to interact with the Nebula package manager. The following commands will be implemented:

```shell
nebula init # Initializes the local Nebula environment

nebula install <package_name> [--version <version>] # Downloads and installs a dataset or model
nebula search <query> # Searches for datasets or models in the Nebula repository
nebula list # Lists all installed packages
nebula update [--all | <package_name>] # Updates datasets and models to the latest version
nebula remove <package_name> # Removes a specified dataset or model
nebula info <package_name> # Displays detailed dataset or model information

nebula explore <package_name> [--filter <json_path>] # Interactively explores package data

nebula config --set <key> <value> # Sets a configuration option
nebula config --get <key> # Retrieves a configuration value

nebula sync # Synchronizes local datasets and models with the remote repository

nebula cache clean [--force] # Clears the local dataset and models cache
nebula cache list # Lists cached datasets and models

nebula <command> --help # Shows help for a specific command
```

Examples:

```shell
nebula search climate_data # Search for packages related to climate data
nebula install neural_net_model_v2 --version 1.0.1 # Install a specific version of a model
nebula install climate_dataset_2023 # Install the latest version of a dataset
nebula explore climate_data_2023 --filter .temperature # Explore specific data within a dataset
nebula update --all # Update all installed datasets and models
nebula remove outdated_model # Remove an outdated model
nebula info new_dataset # Get detailed info about a new dataset
```

## Nebula Backend

The Nebula CLI communicates with the backend via gRPC using Tonic.

## Contributors

The following contributors have either helped to start this project, have contributed
code, are actively maintaining it (including documentation), or in other ways
being awesome contributors to this project. **We'd like to take a moment to recognize them.**

[<img src="https://github.com/mjovanc.png?size=72" alt="mjovanc" width="72">](https://github.com/mjovanc)

## License

The BSD 3-Clause License.
