## Terminology

### Affinity

Affinity is a tag that describes the property of a worker.

### Job Group

Group can be used to group jobs together.

e.g. a group of jobs can be cancelled together.

Group Properties:
- id
- affinity tags:
    - cpu
    - gpu

### Job

Each job is described by:
- group
- docker image:
    - docker pull from registry?
        - maybe ecr or other private registry
    - docker load from s3?
        - upload image to s3 or other storage
        - worker download image and load
- input
    - env
    - file
- output
    - file
    - stdout/stderr log