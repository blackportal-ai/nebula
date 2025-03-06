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

## Main Goal

Design of overal architecture and documentation to start onboarding contributors.

Current state of the [Architecture](./architecture.md)

## Nebula CLI

Uses either a legecy (interactive) command-line or a ratatui frontend. Whereas the latter is
experimental and hidden behind the feature flag `tui`. 

### Commands

The Nebula CLI provides a set of commands to interact with the Nebula package manager. 
This is a copy from the `nebula_cli --help` output:

```shell
A package manager for machine learning datasets and models acting as client for Nebula registries.

Usage: nebula_cli [OPTIONS] [COMMAND]

Commands:
  init       init a virtual environment in the given folder (not yet)
  status     prints status information (not yet)
  install    Installs a package (not yet)
  update     Updates a specific package or all packages (not yet)
  uninstall  Uninstall a specific package or all packages (not yet)
  search     Searches packages by complex criteria (not yet)
  list       List packages that fit simple criteria e.g.(non)-installed,
  sync       Sync the local cache with the remote registry
  help       Print this message or the help of the given subcommand(s)

Options:
      --tui                 use a [ratatui] based terminal user interface instead of a simple cmd-tool
  -i, --interactive         start the cmd-tool in interactive mode, that allows typing multiple commands
  -v, --verbose             use verbose output, only in non TUI mode
  -t, --tick-rate <FLOAT>   Tick rate, i.e. number of ticks per second in tui [default: 4]
  -f, --frame-rate <FLOAT>  Frame rate, i.e. number of frames per second in tui [default: 60]
  -h, --help                Print help
  -V, --version             Print version
help # Shows help for a specific command
```

Examples:

```shell
nebula sync # gets the newest metadata locally from the remote registry
nebula search climate_data # Search for packages related to climate data
nebula install neural_net_model_v2 --version 1.0.1 # Install a specific version of a model
nebula install climate_dataset_2023 # Install the latest version of a dataset
nebula update --all # Update all installed datasets and models
nebula uninstall outdated_model # Remove an outdated model
```

## Nebula Registry

The Nebula CLI communicates with the registry via gRPC using [Tonic](https://github.com/hyperium/tonic). The registry can be self-hosted if desired and using the CLI we can configure the registry URL.

As for now the registry supports the following endpoints:

```protobuf
service NebulaPackageQuery {
    // Gets detailed information for one specific package
    rpc GetPackageInfo (PackageRequest) returns (PackageInfo);

    // List all packages with very simple search criteria
    rpc ListPackages (ListPackagesRequest) returns (PackageList);

    // Search packages applying several filters
    rpc SearchPackages (SearchPackagesRequest) returns (PackageList);
}
```

For more information see the [proto file](./nebula_common/proto/nebula.proto).

The datasets and models are stored elsewhere for now and based on the URL the client is expected to send further GET requests.

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
