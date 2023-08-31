# 数据库迁移

本项目基于[sea-orm-migration](https://crates.io/crates/sea-orm-migration)，旨在降低数据库表管理的难度。



## 配置

sea-orm-migration 内置了[dotenvy](https://crates.io/crates/dotenvy)，可以自动导入 **.env** 文件内的环境变量。
它利用这个包读取你的数据库链接，例如：

```bash
DATABASE_URL='postgres://postgres:postgres@localhost:5432/system'
```

能连接到名为 *system* 的数据库。

> [*DATABASE_URL 规范*](https://www.postgresql.org/docs/10/libpq-connect.html#id-1.7.3.8.3.6)

**.env 文件应该放到项目根目录下**。



## 建表

目前，我们只开发了空表的创建，也就是**还不具备带数据迁移的能力**。

运行子命令`up`，可以施用[MigrationTrait::up](https://docs.rs/sea-orm-migration/latest/sea_orm_migration/trait.MigrationTrait.html#tymethod.up)方法进行迁移：

```bash
$ cargo run -- up
```



## 删表

**警告：请不要在生产环境运行此命令！**

```bash
$ cargo run -- down
```

> 还有`refresh`、`fresh`子命令一律不能在生产环境使用，他们会销毁现存的所有表并施用`up`方法



## 更多用法

请参阅

```bash
$ cargo run -- --help
```

以及[官方文档](https://www.sea-ql.org/SeaORM/docs/migration/setting-up-migration)。
