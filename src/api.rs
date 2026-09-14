use std::ffi::c_int;
use std::fs::{self, File, OpenOptions};
use std::path;
use std::path::PathBuf;
use anyhow::{bail, Result};
use atomicframework_api::ptr_to_string;
use atomicframework_api::raw::RawClass;
use rglua::{cstr, lua_function, lua::{LuaState, lua_pushboolean}};
use rglua::lua::{luaL_checkstring, lua_newuserdata, lua_pop, lua_setmetatable};
use crate::get_package_instance;

#[inline]
fn has_access(path: &PathBuf) -> bool {
  // todo!()
  true
}

fn validate_path(path_str: String) -> Result<PathBuf> {
  let path = path::absolute(path_str)?;

  if !has_access(&path) {
    bail!("")
  }

  Ok(path)
}

fn get_file_class() -> Result<RawClass> {
  get_package_instance()
    .get_class("File")
}

fn convert_into_file_instance(lua: LuaState, file: File) -> Result<i32> {
  let userdata = lua_newuserdata(lua, size_of::<File>()) as *mut File;

  unsafe {
    std::ptr::write(userdata, file);
  }

  let file_class = if let Ok(file) = get_file_class() {
    file
  } else {
    lua_pop(lua, 1);
    bail!("failed to get 'File' class");
  };

  file_class.push_on_stack();
  lua_setmetatable(lua, -2);

  Ok(1)
}

#[lua_function]
pub fn is_directory(lua: LuaState) -> Result<i32> {
  let path_raw = luaL_checkstring(lua, 1);
  let path = validate_path(ptr_to_string!(path_raw))?;

  lua_pushboolean(lua, path.is_dir() as c_int);

  Ok(1)
}

#[lua_function]
pub fn is_file(lua: LuaState) -> Result<i32> {
  let path_raw = luaL_checkstring(lua, 1);
  let path = validate_path(ptr_to_string!(path_raw))?;

  lua_pushboolean(lua, path.is_file() as c_int);

  Ok(1)
}

#[lua_function]
pub fn remove_file(lua: LuaState) -> Result<i32> {
  let path_raw = luaL_checkstring(lua, 1);
  let path = validate_path(ptr_to_string!(path_raw))?;

  lua_pushboolean(lua, fs::remove_file(path).is_ok() as c_int);

  Ok(1)
}

#[lua_function]
pub fn open(lua: LuaState) -> Result<i32> {
  let path_raw = luaL_checkstring(lua, 1);
  let path = validate_path(ptr_to_string!(path_raw))?;

  let file = OpenOptions::new()
    .read(true)
    .write(true)
    .open(path)?;

  convert_into_file_instance(lua, file)
}

#[lua_function]
pub fn create(lua: LuaState) -> Result<i32> {
  let path_raw = luaL_checkstring(lua, 1);
  let path = validate_path(ptr_to_string!(path_raw))?;

  let file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(path)?;

  lua_pop(lua, 1);
  convert_into_file_instance(lua, file)
}