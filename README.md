# TODO server
## getting start
```sh
$ cargo run
```

## endpoints
### GET http://127.0.0.1:8080/tasks
**description**
show all tasks.

**curl**
```sh
$ curl 'http://127.0.0.1:8080/tasks'
```

### POST http://127.0.0.1:8080/tasks
**description**
create new task.

**header**
```
Content-Type:application/json
```

**body**
```
{
  "name": ${task name}
}
```


**curl**
```sh
$ curl 'http://127.0.0.1:8080/tasks'\
-H 'Content-Type:application/json'\
-d '{"name":"finish homework"}'
```
### PATCH http://127.0.0.1:8080/tasks/${task_id}
**description**
toggle task status.

**params**
```
${task_id}: target task id.
```
**curl**
```sh
$ curl -X PATCH 'http://127.0.0.1:8080/tasks/8695086d-943f-4313-a9a0-41bdecd17e9a'
```

### DELETE http://127.0.0.1:8080/tasks/${task_id}
**description**
deelte task.

**params**
```
${task_id}: target task id.
```

**curl**
```sh
$ curl -X DELETE 'http://127.0.0.1:8080/tasks/8695086d-943f-4313-a9a0-41bdecd17e9a'
```