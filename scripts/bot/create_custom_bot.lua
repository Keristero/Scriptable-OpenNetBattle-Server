local function create_custom_bot(id, avatar_id, x, y, z)
  local bot = {
    id = id,
    avatar = avatar_id,
    x = x,
    y = y,
    z = z,
    path = {},
    path_target_index = 1,
    talking_to = nil,
    speed = 1,
    size = .3
  }

  function bot._tick(delta_time)
    if bot.talking_to ~= nil then
      Bots.move_bot(bot.id, bot.x, bot.y, bot.z)
      return
    end

    local player_ids = Players.list_players()

    for i = 1, #player_ids, 1 do
      local player_pos = Players.get_player_position(player_ids[i])

      if
        math.abs(player_pos.x - bot.x) < bot.size and
        math.abs(player_pos.y - bot.y) < bot.size and
        player_pos.z == bot.z
      then
        Bots.move_bot(bot.id, bot.x, bot.y, bot.z)
        return
      end
    end

    local target = bot.path[bot.path_target_index]
    local angle = math.atan(target.y - bot.y, target.x - bot.x)

    local vel_x = math.cos(angle) * bot.speed
    local vel_y = math.sin(angle) * bot.speed

    bot.x = bot.x + vel_x * delta_time
    bot.y = bot.y + vel_y * delta_time

    local distance = math.sqrt((target.x - bot.x) ^ 2 + (target.y - bot.y) ^ 2)

    Bots.move_bot(bot.id, bot.x, bot.y, bot.z)

    if distance < bot.speed * delta_time then
      bot.path_target_index = bot.path_target_index % #bot.path + 1
    end
  end

  function bot._handle_player_conversation(player_id, other_id)
    if bot.talking_to or other_id ~= bot.id then
      Players.send_player_message(player_id, "Sorry I'm busy talking to someone right now.")
      return
    end

    Players.send_player_message(player_id, "Hello!")

    bot.talking_to = player_id

    local player_pos = Players.get_player_position(player_id)
    local angle = math.atan(player_pos.y - bot.y, player_pos.x - bot.x)
    bot.x = bot.x + math.cos(angle) * .02
    bot.y = bot.y + math.sin(angle) * .02
  end

  function bot._handle_player_response(player_id, response)
    if bot.talking_to == player_id then
      bot.talking_to = nil
    end
  end

  function bot._handle_player_disconnect(player_id)
    if bot.talking_to == player_id then
      bot.talking_to = nil
    end
  end

  Bots.create_bot(bot.id, bot.avatar, bot.x, bot.y, bot.z)

  return bot
end

return create_custom_bot