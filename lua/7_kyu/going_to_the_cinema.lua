--[[ 
SPDX-FileCopyrightText: 2025 Michael Persico <michael.a.persico@gmail.com>

SPDX-License-Identifier: MIT-0
]]

-- https://www.codewars.com/kata/562f91ff6a8b77dfe900006e

function movie(card, ticket, perc)
  system_a = ticket
  system_b = card + (ticket * perc)
  ticket_reduce = ticket * perc
  cinema_visits = 1
  while (math.ceil(system_b) >= system_a) do
    system_a = system_a + ticket
    ticket_reduce = ticket_reduce * perc
    system_b = system_b + ticket_reduce
    cinema_visits = cinema_visits + 1
  end
  return cinema_visits
end

print(movie(500,15,0.9))
