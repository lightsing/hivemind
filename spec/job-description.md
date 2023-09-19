## Job Description

job description is a toml file that describes a job.

### Example

```toml
[group]
group = "group1"
create_if_not_exist = true
affinity = ["aws"]
antiaffinity = ["gpu"]

[image]
tag = "image tag"
source = "registry"
# source = "s3+https://s3.amazonaws.com/bucket/image.tar"

[env]
env1 = "value1"
env2 = "value2"

[input]
file1 = { source = "s3+https://s3.amazonaws.com/bucket/file1", path = "/foo/file1" }
file2 = { source = "local+file:///path/to/file2", path = "/foo/file2" }
file3 = { source = "base64+Zm9vCg==", path = "/foo/file3" }

[output]
stdout = true
stderr = true

[output.files]
output1 = "/foo/output1"
output2 = "/foo/output2"
```

### Explanation of Example

#### Group

This job belongs to group `group1`.

If `create_if_not_exist` is true, the group will be created if it doesn't exist.

The job will be dispatched to a worker:
- has tag `aws`
- doesn't have tag `gpu`

#### Image

The image will be pulled from worker's docker registry.

#### Env

The env will be set in the container.

#### Input

The input files will be downloaded/create to worker's local storage
and mounted to the container.

Local file will be uploaded to hive storage while creating the job.

#### Output

The stdout/stderr will be captured and uploaded to hive storage.
The output files will be uploaded to hive storage.


## Job Description Template

Same but with liquid template syntax.

A param jsonl file is required to render the template.
Each line of the jsonl file is a json object that contains the parameters.

### Example of Typical Usage

```toml
[group]
group = "group1"

[image]
tag = "image tag"
source = "s3+https://s3.amazonaws.com/bucket/image.tar"

[env]
env1 = "{{ env1 }}"

[input]
file = { source = "local+file://{{ file }}", path = "/foo/input" }

[output]
stdout = true
stderr = true

[output.files]
output = "/foo/output"
```

```jsonl
{"env1": "value1", "file": "/path/to/file1"}
{"env1": "value2", "file": "/path/to/file2"}
```

This will create two jobs.