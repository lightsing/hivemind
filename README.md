# HIVE

Hive is a framework to dispatch and manage tasks across a cluster of machines.

## Components

### Queen

Queen is the job server.
It accepts jobs from clients and dispatches them to workers.

### Bee

Bee is the worker.
It's created by Queen and acquires jobs from Queen.

### Hivectl

Hivectl is the client.
Use hivectl to submit/monitor/cancel jobs.