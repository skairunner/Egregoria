---
--- Generated by EmmyLua(https://github.com/EmmyLua)
--- Created by paris.
--- DateTime: 11/08/2020 11:05
---

---@class World
world = world

---@class Draw
draw = draw

---@class Vec2
---@class Entity
---@class Color

---@param x number
---@param y number
---@return Vec2
function vec2(x, y) end

---@param r number
---@param g number
---@param b number
---@param a number
---@return Color
function color(r, g, b, a) end

---@param self Vec2
---@return number
function magnitude(self) end

---@param draw Draw
---@param pos Vec2
---@param radius number
function draw.circle(draw, pos, radius) end

---@param world World
---@param e Entity
---@return Vec2
function world.pos(world, e) end

---@param world World
---@param pos Vec2
---@param dir Vec2
---@param objective Vec2
---@return Entity
function world.add_car(world, pos, dir, objective) end