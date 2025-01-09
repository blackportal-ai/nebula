<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/delta-rs/resources/refs/heads/main/nebula/logo/nebula.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/delta-rs/resources/refs/heads/main/nebula/logo/nebula.svg">
    <img alt="Nebula - A package manager for machine learning datasets and models."
         src="https://raw.githubusercontent.com/delta-rs/resources/refs/heads/main/nebula/logo/nebula.svg"
         width="55%">
  </picture>

<br/>
<br/>

![build](https://img.shields.io/github/actions/workflow/status/blackportal-ai/nebula/core.yml?branch=master) 
[![discord badge]](https://discord.gg/g5HtkAzRNG)
[![x handle]][x badge]
</div>

[x badge]: https://twitter.com/intent/follow?screen_name=BlackPortal_AI
[x handle]: https://img.shields.io/twitter/follow/BlackPortal_AI.svg?style=social&label=Follow
[discord badge]: https://img.shields.io/discord/1320514043424931861

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

nebula registry set <registry_url> # Sets the target registry URL

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
nebula registry set https://myregistry.example.com # Set the target registry URL
```

## Nebula Registry

The Nebula CLI communicates with the registry via gRPC using Tonic. The registry can be self-hosted if desired and using the CLI we can configure the registry URL.

## Nebula Registry Web

In the far away future we might implement a web interface for the Nebula registry.

## Contributors

The following contributors have either helped to start this project, have contributed
code, are actively maintaining it (including documentation), or in other ways
being awesome contributors to this project. **We'd like to take a moment to recognize them.**

[<img src="https://github.com/mjovanc.png?size=72" alt="mjovanc" width="72">](https://github.com/mjovanc)
[<img src="https://github.com/DarthB.png?size=72" alt="DarthB" width="72">](https://github.com/DarthB)

## License

The BSD 3-Clause License.
