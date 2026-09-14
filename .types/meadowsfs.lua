---@class MeadowsFS: Atomic.Package
local package = current()

---@param path string
---@return boolean
function package.isDirectory(path)
end

---@param path string
---@return boolean
function package.isFile(path)
end

--- Opens already existing file, throws an error if not existing
---@param path string
---@return MeadowsFS.File?
function package.open(path)
end

--- Creates new file, if it already exists then just opens it
---@param path string
---@return MeadowsFS.File?
function package.create(path)
end

---@param path string
---@return boolean
function package.removeFile(path)
end

---@class MeadowsFS.File: Atomic.Class
local File = package:class("File")

--- Drops file handle, making current
--- object invalid
---@private
---@internal
function File:__gc()
end

--- Truncates file, writes `content`
---@param content string
function File:write(content)
end

--- Appends new content to the file without removing old content
---@param content string
function File:append(content)
end

--- Reads file
---@return string
function File:read()
end