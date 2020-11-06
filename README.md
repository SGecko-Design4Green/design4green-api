# design4green-api

![Rust](https://github.com/SGecko-Design4Green/design4green-api/workflows/Rust/badge.svg)

- Design4Green 2020 API
- Support only memory mode.

## BIN:

- Contains the project binaries.

### REST-API:

- **Main API project** build on Actix-Web framework.

### IMPORT:

- Import binary to feed the indexes and the database.

## DOMAIN:

Contains all the domain definitions based on hexagonal architecture:

- Business traits
- Business implementation
- Business structs
- Storage traits

## STATIC:

- Folder which is statically served by the API.
- The FRONT application build will be copied her.

## STORAGE:

Contains the storage implementation :

- **csv-entry-storage**: input csv module
- **memory-index-storage**: index in memory story
- **sled-db-entry-storage**: db modules

## DATABASE:

Contains database data.

## RESSOURCES:

### INDEXES:

- Contains the indexes to load in memory
