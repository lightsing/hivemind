## Terminology

### Affinity

Affinity is a tag that describes the property of a worker.

### Worker Group

Worker group is a group of workers.
Group Properties:
- id
- affinity tags
- max number of workers
- on idle policy

### Worker

Worker is a machine that runs jobs.
Worker Properties:
- id
- group
- affinity tags (inherited from group)
- managed by queen or not

e.g. a worker can be externally controlled and 
belongs to a group that managed by queen.

### Job Group

Group can be used to group jobs together.

e.g. a group of jobs can be cancelled together.

Group Properties:
- id
- affinity tags

### Job

Each job is described by:
- job group
- docker image:
    - docker pull from registry?
        - maybe ecr or other private registry
    - docker load from s3?
        - upload image to s3 or other storage
        - worker download image and load
- env
- input
    - file
- output
    - file
    - stdout/stderr log