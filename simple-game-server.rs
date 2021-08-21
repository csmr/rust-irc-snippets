// Game Server - Simple indices of each game instance's players state
//
// * session-server addr
// * gamerules object-instance -- maxspeed, world bb
// * player-index, composed of sessionkey, vectors 3Dposition && 3Ddirection
//
// - logic for game-session object request = rules, playerlist.
// - logic for player-session validity check.
// - logic for player-move bounds-check
// - http/udp api-endpoint SubmitPlayerMove
