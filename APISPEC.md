# TESTAUSCLIP API SPEC

this is a prototype proposal

Testausclip API gives 5 different routes:
- [/clip/](#clip)
- [/auth/](#auth)
- [/user/](#user)
- [/explore/](#explore)
- [/search/](#search)
- [/game/](#game)

### <a name="clip"></a> /clip/

| ROUTE | METHOD | ACTION |
|---|---|---|
|/clip/upload            |POST|        Upload a video|
|/clip/get/`id`          |GET|         Retrieve the clip video|
|/clip/delete`id`        |DELETE|      Delete the clip permanently|
|/clip/metadata/`id`     |GET|         Clip metadata, like name and associated game|
|/clip/update/`id`       |PUT|         Update video metadata|
|/clip/like/`id`         |POST|        Like a video|

### <a name="auth"></a> /auth/

| ROUTE | METHOD | ACTION |
|---|---|---|
|/auth/register          |POST|        Register a user|
|/auth/login             |POST|        Login as a user|
|/auth/totp              |POST|        Verify login with TOTP|
|/auth/totp/secret       |GET|         Generate new TOTP key and invalidate existing ones|
|/auth/changeusername    |POST|        Change the user name|
|/auth/changepassword    |POST|        Change the password|
|/auth/regenerate        |POST|        Regenerate login token|
|/auth/delete            |DELETE|      Permanently delete authenticated user account|



### <a name="user"></a> /user/
| ROUTE | METHOD | ACTION |
|---|---|---|
|/user/me                |ANY|         Username me refers to the user making the request|
|/user/`id`              |GET|         Retrieve user information|
|/user/`id`/clips        |GET|         Retrieve clips owned by user|
|/user/`id`/avatar       |GET|         Retrieve user avatar|


### <a name="explore"></a> /explore/
| ROUTE | METHOD | ACTION |
|---|---|---|
|/explore/clip/random    |GET|         Retrieve a random clip(s)|
|/explore/game/random    |GET|         Retrieve a random game(s)|



### <a name="search"></a> /search/
| ROUTE | METHOD | ACTION |
|---|---|---|
|/search/game/`query`    |GET|         Search for games|
|/search/clip/`query`    |GET|         Search for clips|
|/search/user/`query`    |GET|         Search for user|


### <a name="game"></a> /game/
| ROUTE | METHOD | ACTION |
|---|---|---|
|/game/`igdb-id`/new     |GET|         Retrieve *n* newest clips for a game|
|/game/`igdb-id`/top     |GET|         Retrieve *n* top posts of all time|



### Misc.
| ROUTE | METHOD | ACTION |
|---|---|---|
|/ping                   |ANY|         Pong!|