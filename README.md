# design4green-api

![Rust](https://github.com/SGecko-Design4Green/design4green-api/workflows/Rust/badge.svg)

## TODO:

### Referentiel:

- [_] Routes:
  - Regions [X]
  - Departements [X]
  - Towns (NUM + NOMs)
- [x] National statistics (JB)
- [x] Insert into SLED (Laurent) => Check get all
- [_] Full text search.
- [x] Create Index for JSON (loading at start)
- [x] Create Index for Code (IRIS) Insee / CP
- [_] Geoloc neighbour (TOP 5 / KM)
- [x] Memory index
- [x] Github Workflow
- [_] Cache
- [_] Generify Mem-index-storage Vec<String> -> Vec<T>

Si Region (Hauts de France) => Resultats Regions + Resultats Départements Regions
Si Departement (59- Nord) => Resultats Departements de la Region + Resultats Villes départements
Si Ville (Code postal + Nom) => Resultats Villes autours (ou villes départements) + Resultats Quartiers
Si Quartier (Code iris + quartiers) => Resultats quartiers.

- Index :
  - Regions -> Departement (DONE)
  - CodePostal -> CodeInsee (DONE)
  - NomVille -> CodeInsee (DONE)
  - CodeInsee -> Vec<CodeIris> (DONE)
