# VOD markers from CLI

`marker <my text>`

[Marker API](https://dev.twitch.tv/docs/api/markers/)

Need VODs enabled

- requires User Access Token

- [✅] Set up Client Id and Secret
- [✅] Make a call to mark the stream
    - `twitch api post streams/markers -q user_id=39199217 -q description="this is my description"`
- [✅] Give my app OAuth access
    - `twitch token -u -s "<scopes here>"`

- found out how to make a successful call and get my user id from the API
`twitch api get users -q login=deadairx`

## Future Enhancements

- [❌] simplify the CLI command to call (`marker <optional-description>`)




