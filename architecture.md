# Architecture

The architecture is divided in four crates, whereby Delta is used as example for an external crate.

- Common Lib is a library crate that implements most functionality in modules.
- Nebula CLI is the package manager as command line tool.
- Remote Registry is a server that provides endpoints to list and get package informations.
- Delta acts as example for an extrnal crate that uses Nebula as registry for modules.

The following mermaid diagram illustrates some of the relationship.

```mermaid
graph TD
    %% Common Library
    subgraph LIB[Common LIB Modules]
        Configuration[Condfiguration]
        Client[Client Module]
        Datapackage[Datapackage]
        API[API Module]
        Model[Datamodel]
        Registry[Registry Module]
        StorageM[Storage Module]
    end

    %% Define Client Subsystem
    subgraph CLI[Nebula CLI]
        subgraph Frontend
            Legacy[Legacy CLI Implementation]
            Ratatui[Ratatui based UI Implementation]
        end

        Bridge[CLI to API Bridge]
        Events[Post API-call Events]
    end

    %% Define Server Subsystem
    subgraph Remote[Remote Registry]
        Endpoints[Endpoints]
        Metadata[Datapackage Metadata]
        BigFiles[Models and Datasets]
        
        subgraph Storage
            RootFolder[File System]
            SQL[Postgres SQL]
        end
    end

    subgraph Dependency[Delta or other third Party]
        MLSetup[Setup ML Models & Datasets]
        ExternalFunc[Implemented External Function]
    end

    %% Relationships from Dependency to Common Lib
    MLSetup -- "uses" --> API
    MLSetup -- "after ready" --> ExternalFunc

    %% Relationships from common to Remote
    Remote -- "depends on" --> LIB
    %%Registry -- "implemented by" --> Remote
    %%API -- "implemented by" --> Remote
    %%StorageM -- "implemented by" --> Remote
    %%Configuration -- "implemented by" --> Remote
    %%Datapackage -- "implemented by" --> Remote

    %% Relationships form common to CLI
    CLI -- "depends on" --> LIB 
    %%Client -- "implemented by" --> CLI
    %%StorageM -- "implemented by" --> CLI
    %%Configuration -- "implemented by" --> CLI
    %%Datapackage -- "implemented by" --> CLI

    %% Relationships within Remote
    RootFolder -- "data source" --> Metadata
    SQL -- "data source" --> Metadata
    BigFiles -- "stored on" --> RootFolder
    Endpoints -- "access" --> Metadata
    Endpoints -- "access" --> BigFiles

    %% Relationships in Nebula CLI
    Frontend -- "forwards input" --> Bridge
    Events -- "forward API results" --> Frontend
    Bridge -- "invokes" --> API
    Bridge -- "forwards api results" --> Events

    %% Client-Server Interaction
    CLI -- "sync metadata and access ML Models & Datasets" --> Endpoints

```

## Nebula CLI

The command line supports a legacy (interactive) commandline that is powered by [clap](https://github.com/clap-rs/clap). A bridge module is used to transform the clap-related structs into the input arguments of the API module and Model datatypes of the common library.

After the invocation of an API function the results is forwared to a trait that has different implementations for the legacy command line and a [ratatui](https://ratatui.rs/) based TUI.

## Nebula Remote Registry

