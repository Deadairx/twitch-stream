POLL_BODY=$(jq --null-input \
    --arg broadcaster-id "$BROADCASTER_ID" \
    --arg question "$QUESTION" \
    --arg options "$OPTIONS" \
    --arg duration "$DURATION" \
    --argbits "broadcaster_id question options duration" \
    '{broadcaster_id: $broadcaster-id, title: $question, choices: $options, duration: $duration, bits_voting: true, bits_per_vote: 1, max_per_user_per_vote: 1, channel_points_voting: false, channel_points_per_vote: 0, status: "ACTIVE", bits_voting_enabled: true, channel_points_voting_enabled: false, status: "ACTIVE"} | .[$argbits] |= tonumber')

echo "Creating poll..."
curl -X POST "https://api.twitch.tv/extensions/${EXTENSION_ID}/polls" \
    -H "Authorization: Bearer ${TOKEN}" \
    -H "Client-Id: ${CLIENT_ID}" \
    -H "Content-Type: application/json" \
    -d "${POLL_BODY}"
