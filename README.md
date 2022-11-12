# Todone

A revolutionary new concept, software that helps you track a todo list. Learning
project to try and build a production-ish version of a full stack rust todo
application using `axum` and `sqlx` for a backend complete with auth, and `egui` for the frontend
client.

## Get Started:

Using [`just`](https://github.com/casey/just) and docker and cargo

```sh
# copy example .env for backend
cp ./todone-backend/.env.example ./todone-backend/.env

# install tools
just install-tools

# setup db using docker and run migrations
just reset-db

# run backend development build in watch mode
just dev
```

## Insomnia

To try out the api, there is an `insomnia.json` file that works with
[`Insomnia`](https://insomnia.rest/) at the project root
