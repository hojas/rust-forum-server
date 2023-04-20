# rust-forum-server

## 功能模块

- 用户管理
- 帖子管理
- 评论管理

## migration

**1. Setup**

```shell
$ diesel setup
```

**2. Create migration**

```shell
$ diesel migration generate create_posts
```

**3. Write SQL in `up.sql` and `down.sql`**

**4. Run migration**

```shell
$ diesel migration run
$ diesel migration redo
```

参考：https://diesel.rs/guides/getting-started
