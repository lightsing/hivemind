# hivectl

Hivectl is the client of Hive.

## Features

### create job group

```bash
hivectl jobs group create <group name> --affinity <affinity tags>
```

### Submit Jobs

```bash
hivectl job submit <job description file>...
hivectl job submit --template <job description template file> --param <param file>
```

### Retrieve Job Status

```bash
hivectl job status <job id> [-f]
hivectl job status --group <group id> [-f]
```

### Cancel Job

```bash
hivectl job cancel <job id>
hivectl job cancel --group <group id>
```

### Upload Image

upload image to hive storage for workers to download

```bash
hivectl image upload <tag>
hivectl image upload -i <image tar>
```