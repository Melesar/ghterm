#!/bin/sh

cat request.gql | xargs -t -I{} gh api graphql -F owner=blindflugstudios -F name=FirstStrike_Armageddon -F number=729 -f query='{}'
