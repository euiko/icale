# Technical Requirement

To begin to discuss about the technical requirement and what kind of technology being used you need to check the usecase first [here](./usecase.md).
Of course in this application context we will build an web application as my own requirement because this project wanna try some new web technologies for me.
So as web application in commons we have client-server architecture :D.

First of all the point on the technical side from the requirements are there is several kind of tasks form, including to do, schedules, and milestones/goals.
All of them are basically will be represented as tasks in to do, but in some scenarios the tasks can be associated. So when the task's state in to do changes (e.g. completed), it can also mutate the associated entity (achievement, schedules, milestone).
To achieve this look like we need to handle the state changes and propagations properly.
I think it is enough to use centralized event system, to propagate state changes, e.g. NATS Streaming, local/in memory event system, etc.
And to make the life easier first we also need to make some kind of helper or abstraction around this.
The purpose is to handle emiting event, and subscribing event.
For the requirement look like still no particular use case for this, so just basic pattern of emit and subscribe are required.

For the use case of schedule we need to add tasks based on time, so we need to have a generalized schedules system.

Then of course we need to some kind of persistent store, to persist all the data and state. For this I think we need to atleast have file store (e.g. sqlite) and remote store (e.g. postgres).
For the requirement for this is as long as is persisted we can add more in the future.
And the implementation are based on the implementation detail in each domain.

To improve the performance of full blown persistent store we also need much faster storage option such as in memory cache, or remote cache(e.g. redis, other modern in memory kv).
So we can cache all the most frequent accessed data that has least chance to changes.

The system also need to handle concurent use case, even it is more unlikely but I think we need to think it from now on.
All the time data being shown and filtered are based on the client's time (e.g. created at), of course we still need to store the time when server has received the request on each entity.
Any state changes are also propagated to the client through the websocket, of course we need to implement the event system are more like a pipelined event, so we can attach/detach more event pipeline when needed for publishing into more event sink.
Most of the entity has status/state data, so we can use it to represent the entity are on which state.

The development process are start with domain definition, so we can agree about the domain first, then we can start either of api design or business implementation.

Last for the technology we choose, is look like this but we are more flexible around this.
- Server : Rust
- Frontend : Javascript(svelte)
- Persistent Store : sled, postgresql
- Cache : In-memory cache, keydb/redis
- Event System: In-memoty event, NATS Streaming