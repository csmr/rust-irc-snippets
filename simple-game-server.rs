// Game Server - Simple indices of each game instance's players state
//
// * session-server addr
// * gamerules object-instance -- maxspeed, world bb
// * player-index, composed of sessionkey, vectors 3Dposition && 3Ddirection
//
// - logic for requesting game-session object = rules, playerlist.
// - logic for checking player session validity.
// - logic for bounds-checking for player-submitted move
// - logic for (http/udp) api endpoint SubmitPlayerMove, refresh 10 000 ms
