use atomicframework_api::{string_to_ptr, AtomicFramework, Package, PackageKind, PackageMetadata};
use anyhow::Result;
use std::cell::OnceCell;
use rglua::prelude::*;
use crate::class::create_file_class;
use crate::logger::set_logger_instance;

mod class;
mod api;
mod logger;

pub use logger::{log, log_err};

thread_local! {
  static PACKAGE_INSTANCE: OnceCell<Package> = const { OnceCell::new() };
}

const FS_PACKAGE: PackageMetadata = PackageMetadata {
  id: "team.meadows.fs",
  title: "Meadows Filesystem",
  version: env!("CARGO_PKG_VERSION"),
  description: Some("Filesystem API for Garry's Mod"),
  documentation: Some("https://github.com/TeamMeadows/fs/wiki"),
  homepage_url: Some("https://github.com/TeamMeadows/fs"),
  kind: PackageKind::Library,
  icon: None,
};

#[gmod_open]
fn gmod_open(lua: LuaState) -> Result<i32> {
  let atomic_framework = AtomicFramework::new(lua)?;
  let package = atomic_framework.new_package(FS_PACKAGE)?;

  set_package_instance(package.clone());
  set_logger_instance(package.logger().clone());

  // create `File` class associated with team.meadows.fs package
  create_file_class(lua, &package)?;

  // setup `package` api methods
  setup_api(lua, &package);

  Ok(0)
}

fn setup_api(lua: LuaState, package: &Package) {
  package.push_on_stack();

  lua_pushcfunction(lua, api::is_directory);
  lua_setfield(lua, -2, string_to_ptr!("isDirectory"));

  lua_pushcfunction(lua, api::is_file);
  lua_setfield(lua, -2, string_to_ptr!("isFile"));

  lua_pushcfunction(lua, api::remove_file);
  lua_setfield(lua, -2, string_to_ptr!("removeFile"));

  lua_pushcfunction(lua, api::open);
  lua_setfield(lua, -2, string_to_ptr!("open"));

  lua_pushcfunction(lua, api::create);
  lua_setfield(lua, -2, string_to_ptr!("create"));
}

fn set_package_instance(lua: Package) {
  PACKAGE_INSTANCE.with(|lock| lock.set(lua))
    .ok();
}

#[allow(unused)]
pub fn get_package_instance() -> Package {
  PACKAGE_INSTANCE.with(|cell| cell.get().unwrap().clone())
}

#[gmod_close]
fn gmod_close(_lua: LuaState) -> i32 {
  0
}