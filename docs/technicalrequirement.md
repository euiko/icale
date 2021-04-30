# Technical Requirement

To begin to discuss about the technical requirement and what kind of technology being used you need to check the usecase first [here](./usecase.md).

First of all the point on the technical side from the requirements are there is several kind of tasks form, including to do, schedules, and milestones/goals.
All of them are basically will be represented as tasks, but in some scenarios the tasks can be associated. So when the task's state in to do changes (e.g. completed), it can also mutate the associated entity (achievement, schedules, milestone).
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
- Frontend : Typescript/Javascript (svelte/react)
- Persistent Store : sled, postgresql
- Cache : In-memory cache, keydb/redis
- Event System: In-memoty event, NATS Streaming


# Changes
## 2021-04-30
The first idea that I come with is thinking it will be build using basic client-server architecture, so first I think need to handle the server first.
But as time goes on I think in many cases the to do list app are meant to be run on any environment even in standalone mode without any outside connection. Even it will not change some basic idea and technical detail that will be used but it it quite change the direction of the development, that first focus on server application. 
- First thing is the app will offline first, so it fulfill the requirement of the to do list up to be used as standalone application.
- Reusable domain logic, because with several layer of application we need to reuse our domain logic by design, so we can avoid reinventing on each layer we build.

For these additional requirement the one that I can think of that I've already mentioned before all of the system are using event-based architecture, so we can consider any state changes as a event, then we can do anything with that event, like store it to the persistent, etc. When we need to synchronize with the server we just consolidate and synchronize our changes log between two or more parties. So any client/server will be a consumer and publisher of the event.

We have generalized how we can do our business, the we need to reuse as much component as possible. So another idea for this is using the same codebases for the domain parts, then abstract away all the interface that we plugged on to our domain.
This idea is to overcome when we need to implement multiple client application, and the server. To handle this we need all implementation dependent component to be abstracted away not just the api/interface, i.e. persistent store, event store, api. Also we need a stack that can be attached or embedded in many environment, so for this requirement previous decision to use Rust as our main language is still valid, because it has many platform support good support for WebAssembly to be used in the browser context.

First POC that can be made is making a personnal and offline capable application, because it is most wide platform available in the world, it can run on either of full-fledge browser or in a browser-based framework such as electron, tauri, etc.
Because we need to able to reuse our domain, we need to define interface between web components (UI) and our domain. For this purpose I still scatter some idea that possible, but it likes that web (JS Engine) can easily interop with webassembly component in many browser engine, but if it is not supported in particular platform we still can use communication protocol to interop with our domain. 