# Todone

A revolutionary new concept, software that helps you track a todo list. Learning
project to try and build a production-ish version of a full stack rust todo
application using `axum` and `sqlx` for a backend, and `egui` for the frontend
client.

## Get Started:

Using [`just`](https://github.com/casey/just) and docker

```sh
# copy example .env
cp .env.example .env

# install tools
just install-tools

# setup db using docker and run migrations
just reset-db

# run backend development build in watch mode
just dev
```
