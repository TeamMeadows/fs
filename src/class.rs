use std::ffi::c_int;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
// this file contains code of File class for Atomic Framework
use anyhow::{bail, Result};
use atomicframework_api::{string_to_ptr, Package, ptr_to_string};
use rglua::lua::{LuaState, lua_pushstring, luaL_checkstring};
use rglua::{cstr, lua_function};
use rglua::prelude::{lua_pushboolean, lua_touserdata};
use crate::{log, log_err};

fn get_file<'a>(lua: LuaState) -> Result<&'a mut File> {
  let this = lua_touserdata(lua, 1) as *mut File;

  if this.is_null() {
    bail!("invalid file pointer!");
  }

  Ok(unsafe { &mut *this })
}

#[lua_function]
extern "C" fn handle_gc(lua: LuaState) -> Result<i32> {
  if let Ok(file) = get_file(lua) {
    unsafe {
      std::ptr::drop_in_place(file);
    }
  } else {
    log_err("File::__gc() failed to get_file()")
  }

  Ok(0)
}

#[lua_function]
extern "C" fn handle_read(lua: LuaState) -> Result<i32> {
  let this = get_file(lua)?;

  let len = this.metadata()?
    .len() as usize;
  let mut buf = String::with_capacity(len);
  this.read_to_string(&mut buf)?;

  lua_pushstring(lua, string_to_ptr!(buf));

  Ok(1)
}

fn raw_write(file: &mut File, content: String) -> Result<()> {
  file.set_len(0)?;
  file.seek(SeekFrom::Start(0))?;
  file.write_all(content.as_bytes())?;

  Ok(())
}

#[lua_function]
extern "C" fn handle_write(lua: LuaState) -> Result<i32> {
  let this = get_file(lua)?;
  let content = luaL_checkstring(lua, 2);

  raw_write(this, ptr_to_string!(content))?;

  Ok(0)
}

fn raw_append(file: &mut File, content: String) -> Result<()> {
  file.seek(SeekFrom::End(0))?;
  file.write_all(content.as_bytes())?;

  Ok(())
}

#[lua_function]
extern "C" fn handle_append(lua: LuaState) -> Result<i32> {
  let this = get_file(lua)?;
  let content = luaL_checkstring(lua, 2);

  raw_append(this, ptr_to_string!(content))?;

  Ok(0)
}

pub(super) fn create_file_class(lua: LuaState, package: &Package) -> Result<()> {
  let file_class = package.class("File")?;
  file_class.add_method("write", handle_write);
  file_class.add_method("append", handle_append);
  file_class.add_method("read", handle_read);
  file_class.add_method("__gc", handle_gc);

  Ok(())
}