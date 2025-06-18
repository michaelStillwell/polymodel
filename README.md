# Polymodel

Every been annoyed manually writing data models for your server *and* client? Now you can 
declaritively describe your data models to share to all your different projects.

```yaml
targets: [csharp,rust,typescript]
outputs:
  rust:
      dir: test_outputs/rust_output
  typescript:
      dir: test_outputs/typescript_output
  csharp:
      dir: test_outputs/csharp_output
models:
  - user:
      email: email
      password: string
      id: number
  - vehicle:
      id: string
      nickname: string
  - config:
      id: string
      config: string
  - account:
      balance: float
      accountNumber: number
```


---


### Why not use `proto` files?

I have found that `proto` are great for describing *how services look*. With this project I want to
focus on specifically *how **data models** look*. This way you can have your JS frontend have the
same DTO's as your backend through code. That way when you need to update a model, `User`, with a new
field, `pid`, you can do so in one spot.


---


### TODO:

- [ ] union types
- [ ] omit fields
- [ ] add more languages

